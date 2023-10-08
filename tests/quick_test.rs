#![allow(unused)]

use anyhow::{Result, Ok};
use serde_json::json;

#[tokio::test]
async fn quic_test()->Result<()>{
    let client = httpc_test::new_client("http://127.0.0.1:8080")?;
    client.do_get("/hello?name=Pritam").await?.print().await?;
    
    Ok(()) 
}

#[tokio::test]
async fn quic_test2()->Result<()>{
    let client = httpc_test::new_client("http://127.0.0.1:8080")?;
    client.do_get("/hello/pritam").await?.print().await?;
    Ok(()) 
}

#[tokio::test]
async fn quick_test3()->Result<()> {
    let client = httpc_test::new_client("http://127.0.0.1:8080")?;
    client.do_post("/api/login",json!({
        "username":"demo",
        "password":"demo"
    })).await?.print().await?;
    Ok(())
}

#[tokio::test]
async fn quick_test4()->Result<()>{
     let client = httpc_test::new_client("http://127.0.0.1:8080")?;
     client.do_post("/api/tickets", json!({
        "title":"some title"
     })).await?.print().await?;
     Ok(())
}

#[tokio::test]
async fn quic_test5()->Result<()>{
    let client = httpc_test::new_client("http://127.0.0.1:8080")?;
    client.do_get("/api/tickets").await?.print().await?;
    Ok(())
}