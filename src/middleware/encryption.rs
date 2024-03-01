use crate::{
    config::{ENABLE_ENCRYPTION, ENCRYPTION_KEY},
    handlers::error_handler::Errors,
};
use actix_http::h1;
use actix_web::{
    dev::{forward_ready, Payload, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Encryption;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Encryption
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = EncryptionMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(EncryptionMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct EncryptionMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for EncryptionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());
        let svc = self.service.clone();
        Box::pin(async move {
            let body = req.extract::<web::Bytes>().await.unwrap();
            println!("request body (middleware): {body:?}");
            //let res = fut.await?;
            //req.set_payload(bytes_to_payload(body));

            let res = svc.call(req).await?;
            println!("Hi from response");
            Ok(res)
        })
    }
}

fn bytes_to_payload(buf: web::Bytes) -> Payload {
    let (_, mut pl) = h1::Payload::create(true);
    pl.unread_data(buf);
    Payload::from(pl)
}
