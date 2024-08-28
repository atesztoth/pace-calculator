use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::calculations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Calculation {
    pub id: i32,
    pub time: i32,
    pub distance: i32,
    pub pace: i32,
}
