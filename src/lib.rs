pub mod models;
pub mod schema;

use crate::models::Calculation;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use uuid::Uuid;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_calculation(
    conn: &mut SqliteConnection,
    time: f32,
    distance: f32,
    pace: f32,
) -> Calculation {
    use self::models::NewCalculation;
    use crate::schema::calculations;

    let id = Uuid::new_v4();
    let new_calculation = NewCalculation {
        id: &id.to_string(),
        pace,
        distance,
        time,
    };

    diesel::insert_into(calculations::table)
        .values(&new_calculation)
        .returning(Calculation::as_returning())
        .get_result(conn)
        .expect("Error saving calculation")
}
