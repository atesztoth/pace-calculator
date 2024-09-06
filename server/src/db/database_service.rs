use crate::db::errors::DbError;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::SqliteConnection;

pub trait DatabaseService: Send + Sync {
    fn new(db_url: &str) -> Self
    where
        Self: Sized;

    #[allow(dead_code)]
    fn get_pool(&self) -> Pool<ConnectionManager<SqliteConnection>>;

    fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, DbError>;
}

#[derive(Clone)]
pub(crate) struct DatabaseServiceImpl {
    pub connection_pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl DatabaseService for DatabaseServiceImpl {
    /// Creates and initializes db connection.
    /// Only the name of the database is asked for, because of the resolution logic.
    /// In a production application the db must be next to the executable, but in development mode,
    /// it should be in the cargo manifest directory.
    fn new(db_url: &str) -> Self {
        let manager = ConnectionManager::<SqliteConnection>::new(db_url);

        let connection_pool = Pool::builder()
            .build(manager)
            .expect("Failed to create DB pool!");

        DatabaseServiceImpl { connection_pool }
    }

    fn get_pool(&self) -> Pool<ConnectionManager<SqliteConnection>> {
        self.connection_pool.clone()
    }

    fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, DbError> {
        match self.connection_pool.clone().get() {
            Ok(c) => Ok(c),
            Err(e) => Err(DbError::CannotGetPooledConnection(e.to_string())),
        }
    }
}
