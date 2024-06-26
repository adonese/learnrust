use std::collections::HashMap;

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    response::IntoResponse,
    response::Response,
    routing::{get, post},
    Router,
};

use axum::extract::Json;
use serde::{Deserialize, Serialize};
use tokio;

use serde_json::Value;

use tracing::{info, Level};

use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let app = Router::new()
        .route("/", get(root))
        .route("/log", get(log))
        .route("/extern", post(send_extrn))
        .route("/register", post(create_user));
    let listener = tokio::net::TcpListener::bind("localhost:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn root() -> &'static str {
    return "Hello World!";
}

async fn log(Json(body): Json<Value>) -> impl IntoResponse {
    // Continue processing the request

    info!("the request is: {}", body);
    Json("this is me")
}

async fn send_extrn(payload: Json<Message>) -> impl IntoResponse {
    let mut req: HashMap<&str, &str> = HashMap::new();
    req.insert("mobile", payload.mobile.as_str());
    req.insert("password", payload.password.as_str());

    let b = reqwest::Client::new();
    let res = b
        .post("https://dapi.nil.sd/register")
        .json(&req)
        .send()
        .await;
    info!("the response is: {:?}", res);
}

async fn create_user(Json(payload): Json<Req>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

#[derive(Serialize)]
struct User {
    id: u16,
    username: String,
}

#[derive(Deserialize)]
struct Req {
    username: String,
}

#[derive(Deserialize)]
struct Message {
    mobile: String,
    password: String,
    is_agent: bool,
}
