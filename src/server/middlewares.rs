use std::convert::TryFrom;
use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use crate::errors::{TicxError, TicxResult};
use actix_web::http::StatusCode;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::HeaderValue,
    Error,
};
use db::Db;
use futures::{
    future::{ok, Ready},
    Future,
};

macro_rules! pin_svc_call {
    ($me:ident, $req:ident) => {
        Box::pin($me.service.call($req))
    };
}

macro_rules! tracing_span {
    ($level:ident, $name:expr, $guard:ident) => {
        let span = tracing::span!(tracing::Level::$level, stringify!($name));
        let $guard = span.enter();
    };
}

fn box_error<B>(e: TicxError) -> Pin<Box<dyn Future<Output = Result<ServiceResponse<B>, Error>>>> {
    Box::pin(async move { Err(e.into()) })
}

pub(super) struct BasicAuthMiddleware {
    pub(super) db: Arc<Db>,
}

impl<S, B> Transform<S> for BasicAuthMiddleware
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = S::Request;
    type Response = S::Response;
    type Error = Error;
    type Transform = BasicAuthService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(BasicAuthService {
            service,
            db: self.db.clone(),
        })
    }
}

pub(super) struct BasicAuthService<S> {
    service: S,
    db: Arc<Db>,
}

impl<S, B> Service for BasicAuthService<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<ServiceResponse<B>, Error>>>>; // maybe we could turn thins into Ready<..> ?

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        tracing_span!(TRACE, BasicAuthService, guard);

        tracing::trace!("parsing credentials from headers");

        let credentials: super::routes::auth::Credentials = match req
            .headers()
            .get(actix_web::http::header::AUTHORIZATION)
            .ok_or(TicxError::MissingAuthHeader)
            .and_then(|header_value| super::routes::auth::Credentials::try_from(header_value))
        {
            Ok(credentials) => {
                tracing::trace!(?credentials, "basic auth credentials parsed");
                credentials
            }
            Err(e) => {
                tracing::error!( err = %e, "failed to parse basic auth credentials");
                return box_error(e);
            }
        };

        tracing::trace!(
            username = credentials.username(),
            "checking username & password"
        );

        if let Err(_) = self
            .db
            .check_credentials(credentials.username(), credentials.password())
            .map_err(|err| {
                tracing::error!(%err, "no user found for provided credentials");
                TicxError::InvalidCredentials
            })
            .and_then(|user| {
                tracing::trace!(?user, "credentials match for user");
                Ok(user)
            })
        {
            return box_error(TicxError::InvalidCredentials);
        }

        pin_svc_call!(self, req)
    }
}

pub(super) struct JWTValidationMiddleware {
    pub secret: Arc<super::routes::auth::Secret>,
}

impl<S, B> Transform<S> for JWTValidationMiddleware
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JWTValidationService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JWTValidationService {
            service,
            secret: self.secret.clone(),
        })
    }
}

pub(super) struct JWTValidationService<S> {
    secret: Arc<super::routes::auth::Secret>,
    service: S,
}

impl<'a, S, B> Service for JWTValidationService<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<ServiceResponse<B>, Error>>>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        tracing_span!(TRACE, JWTValidationService, guard);

        #[derive(serde::Deserialize, Debug)]
        struct Claims {
            iss: String,
            aud: String,
            sub: String,
            iat: i64,
            exp: i64,
            nbf: i64,
            jti: String,
        }

        let raw_token: String = match req
            .headers()
            .get(actix_web::http::header::AUTHORIZATION)
            .and_then(|header_value| {
                tracing::debug!("AUTHORIZATION header found");
                Some(header_value)
            })
            .ok_or(TicxError::MissingAuthHeader)
            .map_err(|err| {
                tracing::error!("AUTHORIZATION header not found");
                err
            }) {
            Ok(v) => match parse_token(v) {
                Ok(t) => t,
                Err(e) => return box_error(e),
            },
            Err(e) => return box_error(e),
        };

        if let Err(err) = jsonwebtoken::decode::<Claims>(
            raw_token.as_str(),
            &jsonwebtoken::DecodingKey::from_secret(self.secret.0.as_bytes()),
            &jsonwebtoken::Validation {
                leeway: 10,
                validate_exp: true,
                validate_nbf: true,
                aud: Some({
                    let mut set = std::collections::HashSet::new();
                    set.insert(super::routes::auth::AUD.to_owned());
                    set
                }),
                iss: Some(super::routes::auth::ISS.into()),
                sub: None,
                algorithms: vec![jsonwebtoken::Algorithm::HS512],
            },
        ) {
            tracing::error!(%raw_token, %err, "failed to decode JWT");
            return box_error(TicxError::InvalidCredentials);
        }

        tracing::trace!("JTW validation OK");

        drop(guard);

        pin_svc_call!(self, req)
    }
}

#[tracing::instrument(skip(header_value))]
fn parse_token(header_value: &HeaderValue) -> TicxResult<String> {
    tracing::trace!("parsing token from header value");
    let value = header_value.to_str().map_err(|err| {
        tracing::error!(%err, "failed to parse header value to string");
        TicxError::InvalidHeader {
            header: "AUTHORIZATION",
            value: "failed to decode".into(),
            error: err.to_string(),
        }
    })?;

    value
        .split_once(' ')
        .ok_or({
            TicxError::InvalidHeader {
                header: "AUTHORIZATION",
                value: value.to_string(),
                error: "failed to retrieve token from header value".into(),
            }
        })
        .map_err(|err| {
            tracing::error!("AUTHORIZATION token in invalid format. Expected 'Bearer {{JWT}}'");
            err
        })
        .map(|(_bearer, raw_token)| {
            tracing::trace!("successfully parsed JWT");
            raw_token.to_string()
        })
}

// ---------------------------------------------------------------------------------------------- \\

pub(super) struct RequestsCounterMiddleware;

impl<S, B> Transform<S> for RequestsCounterMiddleware
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestCounterService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestCounterService { service })
    }
}

pub(super) struct RequestCounterService<S> {
    service: S,
}

impl<'a, S, B> Service for RequestCounterService<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<ServiceResponse<B>, Error>>>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        crate::metrics::REQUESTS_COUNTER.inc();
        pin_svc_call!(self, req)
    }
}

pub(super) struct ResponseStatusCounterMiddleware;

impl<'a, S, B> Transform<S> for ResponseStatusCounterMiddleware
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ResponseStatusCounterService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ResponseStatusCounterService { service })
    }
}

pub(super) struct ResponseStatusCounterService<S> {
    service: S,
}

impl<'a, S, B> Service for ResponseStatusCounterService<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<ServiceResponse<B>, Error>>>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let start = std::time::Instant::now();
        let fut = self.service.call(req);

        Box::pin(async move {
            match fut.await {
                Ok(res) => {
                    crate::metrics::RESPONSE_COUNTER.inc();
                    let status = res.status();
                    status_counter(status);
                    duration_counter(start.elapsed().as_secs_f64());
                    Ok(res)
                }
                Err(e) => {
                    crate::metrics::ERROR_RESPONSE_COUNTER.inc();
                    crate::metrics::RESPONSE_COUNTER.inc();
                    let status = e.as_response_error().status_code();
                    status_counter(status);
                    duration_counter(start.elapsed().as_secs_f64());
                    Err(e)
                }
            }
        })
    }
}

fn status_counter(status: StatusCode) {
    match status {
        StatusCode::OK => crate::metrics::OK_COUNTER.inc(),
        StatusCode::BAD_REQUEST => crate::metrics::BAD_REQUEST_COUNTER.inc(),
        StatusCode::NOT_FOUND => {
            println!("not found");
            crate::metrics::NOT_FOUND_COUNTER.inc()
        }
        StatusCode::UNAUTHORIZED => crate::metrics::UNAUTHORIZED_COUNTER.inc(),
        StatusCode::INTERNAL_SERVER_ERROR => crate::metrics::INTERNAL_SRV_ERR_COUNTER.inc(),
        _ => (),
    }
}

fn duration_counter(dur: f64) {
    crate::metrics::REQUEST_PROCESS_TIME_TOTAL.inc_by(dur);
}
