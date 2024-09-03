use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;

pub trait DatabaseService: Send + Sync {
    fn new(db_url: &str) -> impl DatabaseService
    where
        Self: Sized;
}

#[derive(Clone)]
pub(crate) struct DatabaseServiceImpl {
    pub connection_pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl DatabaseService for DatabaseServiceImpl {
    fn new(db_url: &str) -> Self {
        let manager = ConnectionManager::<SqliteConnection>::new(db_url);

        let connection_pool = Pool::builder()
            .build(manager)
            .expect("Failed to create DB pool!");

        DatabaseServiceImpl { connection_pool }
    }
}
