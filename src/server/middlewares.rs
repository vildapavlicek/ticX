use std::convert::TryFrom;
use std::marker::PhantomData;
use std::{
    pin::Pin,
    str::FromStr,
    sync::Arc,
    task::{Context, Poll},
};

use crate::errors::{TicxError, TicxResult};
use actix_web::dev::Payload;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::HeaderValue,
    Error,
};
use db::Db;
use futures::{
    future::{err, ok, ready, Ready},
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
        // let span = tracing::span!(tracing::Level::TRACE, "BasicAuthService");
        // let guard = span.enter();

        tracing_span!(TRACE, BasicAuthService, guard);

        let credentials: super::routes::auth::Credentials = match req
            .headers()
            .get(actix_web::http::header::AUTHORIZATION)
            .ok_or(TicxError::MissingAuthHeader)
            .and_then(|header_value| super::routes::auth::Credentials::try_from(header_value))
        {
            Ok(credentials) => credentials,
            Err(e) => return box_error(e), //Box::pin(async move { Err(e.into()) }),
        };

        let _ = self
            .db
            .check_credentials(credentials.username(), credentials.password())
            .map_err(|err| {
                tracing::error!(%err, "credentials do not match");
                err // todo map this error to denied
            })
            .and_then(|u| {
                tracing::trace!(?u, "credentials match found user");
                Ok(u)
            });

        // Box::pin(self.service.call(req))
        drop(guard);
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

        println!("hello from JWT middleware");

        let raw_token: String = match parse_token(
            req.headers()
                .get(actix_web::http::header::AUTHORIZATION)
                .expect("authorization header missing cannot parse token"),
        ) {
            Ok(v) => v,
            Err(e) => return box_error(e),
        };

        let decoded = jsonwebtoken::decode::<Claims>(
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
        )
        .expect("token is invalid :( or some other problem");

        let claims = decoded.claims;
        let header = decoded.header;

        drop(guard);

        pin_svc_call!(self, req)
    }
}

fn parse_token(header_value: &HeaderValue) -> TicxResult<String> {
    let value = header_value
        .to_str()
        .map_err(|err| TicxError::InvalidHeader {
            header: "AUTHORIZATION",
            value: "failed to decode".into(),
            error: err.to_string(),
        })?;

    value
        .split_once(' ')
        .ok_or(TicxError::InvalidHeader {
            header: "AUTHORIZATION",
            value: value.to_string(),
            error: "failed to retrieve token from header value".into(),
        })
        .map(|(bearer, raw_token)| raw_token.to_string())
}

fn box_error<B>(e: TicxError) -> Pin<Box<dyn Future<Output = Result<ServiceResponse<B>, Error>>>> {
    Box::pin(async move { Err(e.into()) })
}
