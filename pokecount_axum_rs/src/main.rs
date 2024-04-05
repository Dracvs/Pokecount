use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Deserialize)]
struct PokeElement {
    name: String,
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Pokemon {
    number: i32,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CPUTime {
    spent_time: String,
    date: String,
}

#[derive(Debug)]
enum ApiError {
    FailedState,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/pokecount", post(pokecount));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn pokecount(Json(payload): Json<Vec<PokeElement>>) -> Result<String, ApiError> {
    let start = Instant::now();
    let mut pokedex: Vec<Pokemon> = vec![];
    let mut counter = 1;
    for number in 1..100_000 {
        for element in payload.iter() {
            pokedex.push(Pokemon {
                number: counter,
                name: String::from(&element.name),
            });
            counter = number;
        }
    }

    let duration = start.elapsed();
    Ok(format!("Gotta Catch em All in {:?}", duration))
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let error_message = match self {
            ApiError::FailedState => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Couldnt Parse the PokeDex",
            ),
        };

        let message = error_message;
        message.into_response()
    }
}
