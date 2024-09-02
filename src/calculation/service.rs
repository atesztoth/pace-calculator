use crate::calculation::dto::{Meter, Seconds};

#[derive(Clone)]
pub struct CalculatorService {}

impl CalculatorService {
    pub fn new() -> Self {
        CalculatorService {}
    }

    // For test:
    pub fn calculate_pace(&self, time: Seconds, dist: Meter) -> f32 {
        1f32
    }

    // TODO: implement these
    pub fn calculate_time(&self, dist: Meter, pace: Seconds) -> f32 {
        todo!()
    }

    pub fn calculate_dist(&self, pace: Seconds, time: Seconds) -> f32 {
        todo!()
    }
}
