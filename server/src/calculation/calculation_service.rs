use crate::{
    calculation::dto::{CalculationResult, Meter, Pace, Seconds},
    calculation::errors::CalculationError,
    db::database_service::DatabaseService,
    error::api_error::ApiError,
    models::{Calculation, NewCalculation},
    schema::calculations,
};
use diesel::{RunQueryDsl, SelectableHelper};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct CalculatorService {
    db: Arc<dyn DatabaseService>,
}

impl CalculatorService {
    pub fn new<T: DatabaseService + 'static>(db: Arc<T>) -> Self {
        CalculatorService { db }
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

    pub fn store_calculation(&self, calc: &CalculationResult) -> Result<Calculation, ApiError> {
        let conn = &mut self.db.get_connection()?;

        let idx = Uuid::new_v4();
        let new_calculation = NewCalculation {
            id: &idx.to_string(),
            pace: calc.pace,
            distance: calc.distance,
            time: calc.distance,
        };

        let save_result = diesel::insert_into(calculations::table)
            .values(&new_calculation)
            .returning(Calculation::as_returning())
            .get_result(conn);

        match save_result {
            Ok(r) => Ok(r),
            // TODO: Fix this madness!
            Err(e) => Err(ApiError::CalculationError(CalculationError::SaveFailed(
                e.to_string(),
            ))),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::calculation::calculation_service::CalculatorService;
    use crate::db::database_service::DatabaseService;
    use crate::db::errors::DbError;
    use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
    use diesel::SqliteConnection;
    use std::sync::Arc;

    struct MockDatabaseService;

    impl DatabaseService for MockDatabaseService {
        fn new(db_url: &str) -> impl DatabaseService
        where
            Self: Sized,
        {
            todo!()
        }

        fn get_pool(&self) -> Pool<ConnectionManager<SqliteConnection>> {
            todo!()
        }

        fn get_connection(
            &self,
        ) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, DbError> {
            todo!()
        }
    }

    #[test]
    fn test_calculate_pace_si() {
        let db = Arc::new(MockDatabaseService);
        let res = CalculatorService::new(db).calculate_pace(2640f32, 10_000f32);
        assert_eq!(res, 264f32);
    }

    #[test]
    fn test_calculate_time_si() {
        let db = Arc::new(MockDatabaseService);
        let res = CalculatorService::new(db).calculate_time(10_000f32, 264f32);
        assert_eq!(res, 2640f32);
    }

    #[test]
    fn test_calculate_dist_si() {
        let db = Arc::new(MockDatabaseService);
        let res = CalculatorService::new(db).calculate_dist(264f32, 2640f32);
        assert_eq!(res, 10_000f32);
    }
}
