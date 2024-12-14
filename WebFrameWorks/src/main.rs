use axum::{response::Html, routing::get, Router};
use std::net::TcpListener;
use pyo3::prelude::*;
use crate::TcpListener;


#[tokio::main]
async fn main(){
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0:3000").await.unwrap();

    println!("Listening on http://{}",listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1> Hello World!</h1>")
}