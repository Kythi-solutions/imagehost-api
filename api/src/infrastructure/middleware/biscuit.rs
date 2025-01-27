// credit to: https://crates.io/crates/biscuit-actix-middleware
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::Header,
    Error, HttpMessage, HttpResponse, ResponseError,
};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use biscuit_auth::{Biscuit, RootKeyProvider};
use derive_more::Display;
use futures_util::future::LocalBoxFuture;
use log::warn;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

pub(crate) type MiddlewareResult<R> = Result<R, HttpResponse>;

#[derive(Debug, Display)]
pub enum MiddlewareError {
    InvalidHeader,
    InvalidToken,
}

impl ResponseError for MiddlewareError {
    fn error_response(&self) -> HttpResponse {
        match self {
            MiddlewareError::InvalidHeader => HttpResponse::Unauthorized().finish(),
            MiddlewareError::InvalidToken => HttpResponse::Forbidden().finish(),
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

type ErrorHandler = fn(MiddlewareError, &ServiceRequest) -> HttpResponse;
type TokenExtractor = fn(&ServiceRequest) -> Option<Vec<u8>>;

pub struct BiscuitMiddleware {
    public_key: Rc<dyn RootKeyProvider>,
    error_handler: ErrorHandler,
    token_extractor: TokenExtractor,
}

impl BiscuitMiddleware {
    pub fn new<KP>(public_key: KP) -> BiscuitMiddleware
    where
        KP: RootKeyProvider + 'static,
    {
        BiscuitMiddleware {
            public_key: Rc::new(public_key),
            error_handler: |err: MiddlewareError, _: &ServiceRequest| err.error_response(),
            token_extractor: Self::default_token_extractor,
        }
    }

    pub fn token_extractor(mut self, extractor: fn(&ServiceRequest) -> Option<Vec<u8>>) -> Self {
        self.token_extractor = extractor;

        self
    }

    pub fn default_token_extractor(req: &ServiceRequest) -> Option<Vec<u8>> {
        Some(
            Authorization::<Bearer>::parse(req)
                .map_err(|_e| {
                    warn!("{}", _e.to_string());
                })
                .ok()?
                .as_ref()
                .token()
                .to_string()
                .into_bytes(),
        )
    }
}

impl<S, B> Transform<S, ServiceRequest> for BiscuitMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = ImplBiscuitMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ImplBiscuitMiddleware {
            service,
            public_key: self.public_key.clone(),
            error_handler: self.error_handler,
            token_extractor: self.token_extractor,
        }))
    }
}

pub struct ImplBiscuitMiddleware<S> {
    service: S,
    public_key: Rc<dyn RootKeyProvider>,
    error_handler: ErrorHandler,
    token_extractor: TokenExtractor,
}

impl<S> ImplBiscuitMiddleware<S> {
    fn extract_biscuit(&self, req: &ServiceRequest) -> MiddlewareResult<Biscuit> {
        let token = (self.token_extractor)(req)
            .ok_or((self.error_handler)(MiddlewareError::InvalidHeader, req))?;

        // Parse token
        Biscuit::from_base64(token, self.public_key.clone()).map_err(|_e| {
            warn!("{}", _e.to_string());
            (self.error_handler)(MiddlewareError::InvalidToken, req)
        })
    }
}

impl<S, B> Service<ServiceRequest> for ImplBiscuitMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        match self.extract_biscuit(&req) {
            Ok(biscuit) => {
                req.extensions_mut().insert(biscuit);
                let fut = self.service.call(req);

                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res.map_into_left_body())
                })
            }
            Err(e) => Box::pin(async move {
                let r = req.into_response(e).map_into_right_body::<B>();
                Ok(r)
            }),
        }
    }
}
