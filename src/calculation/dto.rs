use serde::{Deserialize, Serialize};

// Think about it like an equation.
// Any one of the properties can be missing, but only one.
#[derive(Debug, Deserialize)]
pub struct IncomingCalculationDetails {
    time: Option<f32>,
    distance: Option<f32>,
    pace: Option<f32>,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct CalculationResult {
    pub time: f32,
    pub distance: f32,
    pub pace: f32,
}
