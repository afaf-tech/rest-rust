use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ok, Ready};
use log::info;
use serde_json::json;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};

pub struct HttpLogger;

impl<S, B> Transform<S, ServiceRequest> for HttpLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = HttpLoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(HttpLoggerMiddleware { service })
    }
}

pub struct HttpLoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for HttpLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start_time = Instant::now();
        let method = req.method().to_string();
        let uri = req.uri().to_string();

        // Clone the User-Agent header value to avoid borrowing 'req'
        let user_agent = req
            .headers()
            .get("User-Agent")
            .and_then(|value| value.to_str().ok())
            .unwrap_or("-")
            .to_string(); // Clone to make it owned

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let duration = start_time.elapsed();
            let latency = format!("{:?}", duration);
            let status = res.status().as_u16();

            // Log essential data including the cloned User-Agent
            info!(
                "{}",
                json!({
                    "method": method,
                    "uri": uri,
                    "status": status,
                    "latency": latency,
                    "user_agent": user_agent,  // Use the owned User-Agent here
                })
            );

            Ok(res)
        })
    }
}
