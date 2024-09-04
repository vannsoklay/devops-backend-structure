use actix_web::{HttpMessage as _, HttpRequest, HttpResponse};

use crate::jwt::Claims;

pub async fn handler(req: HttpRequest) -> Result<Claims, HttpResponse> {
    // Extract claims from the request extensions
    if let Some(claims) = req.extensions().get::<Claims>() {
        return Ok(claims.to_owned());
    } else {
        return Err(HttpResponse::Unauthorized().body("Invalid credentials"));
    }
}
