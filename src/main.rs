use std::net::SocketAddr;
mod error;
mod routes_login;
mod  model;
mod routes_tickets;
mod auth;
mod ctx;


use axum::extract::{Path,Query};
use axum::middleware;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{ get, get_service };
use axum::Router;
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use error::{ Error, Result };
use auth::middleware;

use crate::model::ModelController;



#[derive(Debug, Deserialize)]
struct GetParams {
    name: Option<String>,
}



//Main Server//
#[tokio::main]
async fn main()->Result<()>{
    let mc = ModelController::new().await?;
    let api_routes = routes_tickets::router(mc.clone())
    .route_layer(middleware::from_fn(middleware));
    let app = Router::new()
        .merge(merge_routes())
        .merge(routes_login::routes())
        .nest("/api", api_routes)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    //Here the layer executed from bottom to top

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn main_response_mapper(res:Response)->Response {
    println!("--> {:<12} - Main_Response_Mapper", "Res Mapper"); 
    println!();
    res  
}

//Grouping Routes//
fn merge_routes() -> Router {
    Router::new()
        .route("/hello", get(some_handler))
        .route("/hello/:name", get(some_handler2))
}

//Some handler is being using for /hello route
async fn some_handler(Query(params): Query<GetParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler", "Handler");
    let name = params.name.as_deref().unwrap_or("World");
    println!("{:?}", name);
    Html(format!("Hello <h1>{}</h1>", name))
}

//Some handler is being using for /hello/:name
async fn some_handler2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler", "Handler");
    println!("{:?}", name);
    Html(format!("Hello <h1>{}</h1>", name))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
