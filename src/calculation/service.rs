use crate::calculation::dto::{Meter, Pace, Seconds};

#[derive(Clone)]
pub struct CalculatorService {}

impl CalculatorService {
    pub fn new() -> Self {
        CalculatorService {}
    }

    /// Calculates how many seconds it takes to run a kilometer.
    #[inline(always)]
    pub fn calculate_pace(&self, time: Seconds, dist: Meter) -> Pace {
        1000f32 / (dist / time)
    }

    /// Calculates how much time it took to run the distance.
    #[inline(always)]
    pub fn calculate_time(&self, dist: Meter, pace: Pace) -> Seconds {
        pace / 1000f32 * dist
    }

    /// Calculates how much distance will pass for keeping the given pace for a given time.
    #[inline(always)]
    pub fn calculate_dist(&self, pace: Pace, time: Seconds) -> Meter {
        time / (pace / 1000f32)
    }
}

#[cfg(test)]
mod test {
    use crate::calculation::service::CalculatorService;

    #[test]
    fn test_calculate_pace_si() {
        let res = CalculatorService::new().calculate_pace(2640f32, 10_000f32);
        assert_eq!(res, 264f32);
    }

    #[test]
    fn test_calculate_time_si() {
        let res = CalculatorService::new().calculate_time(10_000f32, 264f32);
        assert_eq!(res, 2640f32);
    }

    #[test]
    fn test_calculate_dist_si() {
        let res = CalculatorService::new().calculate_dist(264f32, 2640f32);
        assert_eq!(res, 10_000f32);
    }
}
