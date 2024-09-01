use crate::calculation::dto::{CalculationResult, IncomingCalculationDetails};
use axum::http::StatusCode;
use axum::Json;

pub async fn calculate_pace(
    Json(payload): Json<IncomingCalculationDetails>,
) -> (StatusCode, Json<CalculationResult>) {
    println!("Incoming: {:?}", payload);
    let result = CalculationResult {
        time: 450f32,
        distance: 50f32,
        pace: 50f32,
    };

    (StatusCode::OK, Json(result))
}
