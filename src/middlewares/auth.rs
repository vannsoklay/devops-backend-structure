use actix_service::{Service, Transform};
use actix_session::SessionExt as _;
use actix_web::{
    body::{BoxBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpMessage, HttpResponse,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::task::{Context, Poll};

// Define the struct for the Authentication middleware
pub struct Authentication;

// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (usually user ID)
    pub exp: usize,  // Expiration time (as a timestamp)
}

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
        let session = req.get_session();

        let message = session.get::<String>("message").unwrap();

        println!("message {:?}", message);

        // Extract the Authorization header
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = auth_str.trim_start_matches("Bearer ").trim();

                    // Decode and validate the token
                    let validation = Validation::new(Algorithm::HS256);
                    let secret = "my_secret_key"; // Replace with your secret key

                    match decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(secret.as_ref()),
                        &validation,
                    ) {
                        Ok(token_data) => {
                            println!("Valid token for user: {:?}", token_data.claims.sub);
                            // Insert claims into the request extensions for access in handlers
                            req.extensions_mut().insert(token_data.claims);

                            // Call the next service in the middleware chain
                            let fut = self.service.call(req);
                            return Box::pin(async move {
                                fut.await.map(|res| res.map_into_boxed_body())
                            });
                        }
                        Err(err) => {
                            println!("Token error: {:?}", err);
                        }
                    }
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
