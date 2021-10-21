use std::convert::TryFrom;
use std::marker::PhantomData;
use std::{
    pin::Pin,
    str::FromStr,
    sync::Arc,
    task::{Context, Poll},
};

use crate::errors::TicxError;
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
        let span = tracing::span!(tracing::Level::TRACE, "basic auth validator");
        let _guard = span.enter();

        let credentials: super::routes::auth::Credentials = match req
            .headers()
            .get(actix_web::http::header::AUTHORIZATION)
            .ok_or(TicxError::MissingAuthHeader)
            .and_then(|header_value| super::routes::auth::Credentials::try_from(header_value))
        {
            Ok(credentials) => credentials,
            Err(e) => return Box::pin(async move { Err(e.into()) }),
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

        let fut = self.service.call(req);

        Box::pin(async move { fut.await })
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
        println!("hello from JWT middleware");
        Box::pin(self.service.call(req))
    }
}
