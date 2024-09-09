use diesel::connection::SimpleConnection;
use diesel::r2d2::CustomizeConnection;
use diesel::r2d2::Error as R2d2Error;
use diesel::SqliteConnection;

#[derive(Debug)]
pub struct SqliteConnectionCustomizer;

impl CustomizeConnection<SqliteConnection, R2d2Error> for SqliteConnectionCustomizer {
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), R2d2Error> {
        conn.batch_execute("PRAGMA busy_timeout = 10000;")
            .map_err(|e| R2d2Error::QueryError(e)) // Convert Diesel's error to r2d2::Error
    }
}
