use serde::{Deserialize, Serialize};
use validator::Validate;

/// How many seconds needed to travel a km
pub(crate) type Pace = f32;
pub(crate) type Meter = f32;
pub(crate) type Seconds = f32;

// Think about it like an equation.
// Any one of the properties can be missing, but only one.
#[derive(Debug, Deserialize, Validate)]
// #[validate(schema(function = "validate_calc_details", skip_on_field_errors = false))]
pub struct IncomingCalculationDetails {
    #[validate(range(exclusive_min = 0.0))]
    pub time: Option<Seconds>,
    #[validate(range(exclusive_min = 0.0))]
    pub distance: Option<Meter>,
    #[validate(range(exclusive_min = 0.0))]
    pub pace: Option<Pace>,
}

// Not doing this, because the resulting error msg is just too ugly for me. :')
// Will have to find a solution to fix it. I cannot bear this:
// __all__: Two parameters should be provided. .
// I mean ... man, please, give me a way to get rid of that __all__ and stuff.
// fn validate_calc_details(input: &IncomingCalculationDetails) -> Result<(), ValidationError> {
//     match (input.pace, input.distance, input.time) {
//         (None, None, None)
//         | (Some(_), None, None)
//         | (None, Some(_), None)
//         | (None, None, Some(_))
//         | (Some(_), Some(_), Some(_)) => {
//             Err(ValidationError::new("").with_message("Two parameters should be provided.".into()))
//         }
//         _ => Ok(()),
//     }
// }

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct CalculationResult {
    pub time: Seconds,
    pub distance: Meter,
    pub pace: Pace,
}
