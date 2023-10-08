use crate::{Error, Result};
use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookies, Cookie};

pub const AUTH_TOKEN:&str  = "auth-token";

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn login(cookies: Cookies,payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("--> {:<12} - API-LOGIN", "Handler");
    if payload.username != "demo" || payload.password != "demo" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(AUTH_TOKEN, "some_user1"));

    let body = Json(json!(
        {
            "result" : {
                 "success": true
            }
        }
    ));
    Ok(body)
}

pub fn routes() -> Router{
    Router::new().route("/api/login", post(login))
}