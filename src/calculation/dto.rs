use serde::{Deserialize, Serialize};

/// How many seconds needed to travel a km
pub(crate) type Pace = f32;
pub(crate) type Meter = f32;
pub(crate) type Seconds = f32;

// Think about it like an equation.
// Any one of the properties can be missing, but only one.
#[derive(Debug, Deserialize)]
pub struct IncomingCalculationDetails {
    pub time: Option<f32>,
    pub distance: Option<f32>,
    pub pace: Option<f32>,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct CalculationResult {
    pub time: f32,
    pub distance: f32,
    pub pace: f32,
}
