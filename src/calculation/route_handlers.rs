use crate::calculation::dto::{CalculationResult, IncomingCalculationDetails, Meter, Seconds};
use crate::calculation::service::CalculatorService;
use crate::response::api_response::ApiErrorResponse;
use crate::validation::valid_json_request::ValidJsonRequest;
use crate::SharedState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;
use axum::Json;

pub async fn run_calculation(
    State(state): State<SharedState>,
    ValidJsonRequest(payload): ValidJsonRequest<IncomingCalculationDetails>,
) -> Result<(StatusCode, Json<CalculationResult>), Response> {
    let calculator = &state.read().unwrap().calculator;

    let response = match (payload.pace, payload.distance, payload.time) {
        (Some(pace), Some(distance), None) => Some(calculate_time(distance, pace, calculator)),
        (Some(pace), None, Some(time)) => Some(calculate_dist(pace, time, calculator)),
        (None, Some(distance), Some(time)) => Some(calculate_pace(time, distance, calculator)),
        _ => None,
    };

    if response.is_none() {
        return Err(ApiErrorResponse::new(
            StatusCode::BAD_REQUEST,
            Some("Invalid input! Provide two parameters!".to_string()),
        ));
    }

    Ok((StatusCode::OK, Json(response.unwrap())))
}

fn calculate_pace(time: Seconds, dist: Meter, calculator: &CalculatorService) -> CalculationResult {
    CalculationResult {
        time,
        distance: dist,
        pace: calculator.calculate_pace(time, dist),
    }
}

fn calculate_time(dist: Meter, pace: Seconds, calculator: &CalculatorService) -> CalculationResult {
    CalculationResult {
        time: calculator.calculate_time(dist, pace),
        distance: dist,
        pace,
    }
}

fn calculate_dist(
    pace: Seconds,
    time: Seconds,
    calculator: &CalculatorService,
) -> CalculationResult {
    CalculationResult {
        time,
        distance: calculator.calculate_dist(pace, time),
        pace,
    }
}
