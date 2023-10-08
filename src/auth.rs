use async_trait::async_trait;
use axum::{response::Response, http::{Request, request::Parts},middleware::Next, extract::FromRequestParts, RequestPartsExt};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;
use crate::{error::{ Result, Error }, ctx::Context};


pub async fn middleware<B>(ctx:Result<Context>, req:Request<B>, next:Next<B>)->Result<Response> {
      ctx?;
      Ok(next.run(req).await) 
}

fn parse_token(token:String) -> Result<(u64,String,String)>{
     let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#,
        &token
     ).ok_or(Error::NotAuthorizedError)?;
     let user_id = user_id.parse().map_err(|_| Error::NotAuthorizedError)?;
     Ok((user_id, exp.to_string(), sign.to_string()))
}

#[async_trait]
impl<S:Send + Sync> FromRequestParts<S> for Context{
    type Rejection = Error;
    async fn from_request_parts(parts:&mut Parts, _state:&S) ->Result<Self> {
        let cookies = parts.extract::<Cookies>().await.unwrap();
        let auth_token = cookies.get("auth-token").map(|c| c.value().to_string());
        let ( user_id, exp, sign ) = auth_token.ok_or(Error::NotAuthorizedError).and_then(parse_token)?;
        Ok(Context::new(user_id))
    }
}



