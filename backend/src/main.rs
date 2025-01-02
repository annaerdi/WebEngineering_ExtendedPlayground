use axum::{
    http::StatusCode,
    routing::get,
    Json, Router,
};
use reqwest::Client;
use serde_json::Value;
use tokio;
use tower_http::cors::{Any, CorsLayer};

/// Fetching the bear data from Wikipedia
async fn fetch_bear_data() -> Result<Json<Value>, (StatusCode, String)> {
    let base_url = "https://en.wikipedia.org/w/api.php";
    let params = [
        ("action", "parse"),
        ("page", "List_of_ursids"),
        ("prop", "wikitext"),
        ("section", "3"),
        ("format", "json"),
        ("origin", "*"),
    ];

    let client = Client::new();

    // make the request
    let response = client
        .get(base_url)
        .query(&params)
        .send()
        .await
        .map_err(|err| (StatusCode::BAD_GATEWAY, format!("Request error: {}", err)))?;

    // deserialize JSON
    let data = response
        .json::<Value>()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    // === debugging ===
    // println!("Raw Wikipedia JSON: {:#?}", data);
    // just the "wikitext":
    // if let Some(wikitext) = data["parse"]["wikitext"]["*"].as_str() {
    //     println!("Wikitext from JSON:\n{}", wikitext);
    // }

    // return the full json to the client
    Ok(Json(data))
}

#[tokio::main]
async fn main() {
    // Building the router
    let app = Router::new()
        .route("/bears", get(fetch_bear_data))
        // Adding a CORS layer so the frontend can call this endpoint
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );
    // Running the server on port 3000
    println!("Server is running on http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
