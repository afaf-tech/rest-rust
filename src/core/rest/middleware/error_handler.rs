use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, Result,
};
use futures::future::{ok, Ready};
use std::{future::Future, pin::Pin};

use crate::core::domain::error::AppError;

/// Error handling middleware that provides centralized error processing
pub struct ErrorHandler;

impl<S, B> Transform<S, ServiceRequest> for ErrorHandler
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ErrorHandlerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ErrorHandlerMiddleware { service })
    }
}

pub struct ErrorHandlerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ErrorHandlerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await;

            // Log errors for monitoring and debugging
            if let Err(ref e) = res {
                // Check if it's our custom AppError
                if let Some(app_err) = e.as_error::<AppError>() {
                    log::error!("Application error: {}", app_err);
                } else {
                    log::error!("Unexpected error: {}", e);
                }
            }

            res
        })
    }
}
