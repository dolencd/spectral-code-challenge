use std::net::SocketAddr;
mod dataclient;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use dataclient::fetch_sensor_data;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/api", get(handler))
        // Using the same server for the frontend for simplicity.
        .route("/", get(serve_frontend));

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct SensorDataResponse {
    time: String,
    meterusage: f32,
}

async fn serve_frontend() -> Html<&'static str> {
    Html(include_str!("../frontend.html"))
}

async fn handler() -> impl IntoResponse {
    match fetch_and_format_sensor_data().await {
        Err(e) => {
            println!("Failed to get data {:?}", e);
            (StatusCode::IM_A_TEAPOT, "".to_string())
        }
        Ok(data) => (StatusCode::OK, data),
    }
}

async fn fetch_and_format_sensor_data() -> anyhow::Result<String> {
    let data: Vec<SensorDataResponse> = fetch_sensor_data()
        .await?
        .meter_usage
        .into_iter()
        .map(|meter_data_point| SensorDataResponse::try_from(meter_data_point))
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(serde_json::to_string(&data)?)
}
