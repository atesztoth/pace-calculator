use self::models::*;
use diesel::prelude::*;
use pace_calculator::*;

pub fn main() {
    use self::schema::calculations::dsl::*;

    let connection = &mut establish_connection();

    let results = calculations
        .select(Calculation::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} calculations", results.len());
    for calculation in results {
        println!("{:?}", calculation);
    }
}
