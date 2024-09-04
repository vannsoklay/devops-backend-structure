use actix_service::{Service, Transform};
use actix_session::SessionExt as _;
use actix_web::{
    body::{BoxBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpMessage, HttpResponse,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::task::{Context, Poll};

use crate::{get_config, jwt::Claims};

// Define the struct for the Authentication middleware
pub struct Authentication;

// Implement the Transform trait for the Authentication middleware
impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static, // Ensure B is a valid MessageBody
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

// Define the struct for the inner middleware
pub struct AuthenticationMiddleware<S> {
    service: S,
}

// Implement the Service trait for the inner middleware
impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static, // Ensure B is a valid MessageBody
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let secret = get_config().jwt_secret;
        let session = req.get_session();

        let token = session.get::<String>("token").unwrap();

        if token.is_some() {
            let validation = Validation::new(Algorithm::HS256);
            match decode::<Claims>(
                &token.unwrap().clone(),
                &DecodingKey::from_secret(secret.as_ref()),
                &validation,
            ) {
                Ok(token_data) => {
                    // Insert claims into the request extensions for access in handlers
                    req.extensions_mut().insert(token_data.claims);

                    // Call the next service in the middleware chain
                    let fut = self.service.call(req);
                    return Box::pin(async move { fut.await.map(|res| res.map_into_boxed_body()) });
                }
                Err(err) => {
                    let response = HttpResponse::Unauthorized()
                        .body(format!("Invalid {:?}", err))
                        .map_into_boxed_body();
                    return Box::pin(async move { Ok(req.into_response(response)) });
                }
            }
        }

        // If token is invalid or missing, return Unauthorized response
        let response = HttpResponse::Unauthorized()
            .body("Unauthorized")
            .map_into_boxed_body();
        Box::pin(async move { Ok(req.into_response(response)) })
    }
}
