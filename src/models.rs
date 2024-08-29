use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::calculations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Calculation {
    pub id: String,
    // if more precision is needed, store them as text because Diesel does not support
    // f64 due to potential precision issues. I doubt more precision would be needed tho.
    pub time: f32,
    pub distance: f32,
    pub pace: f32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::calculations)]
pub struct NewCalculation<'a> {
    pub id: &'a str,
    pub time: f32,
    pub distance: f32,
    pub pace: f32,
}
