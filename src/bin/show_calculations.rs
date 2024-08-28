use diesel::QueryDsl;
use pace_calculator::establish_connection;
use pace_calculator::models::Calculation;

pub fn main() {
    use pace_calculator::schema::calculations::dsl::calculations;

    let connection = &mut establish_connection();

    let results = calculations
        .select(Calculation::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for calculation in results {
        println!("{:?}", calculation);
    }
}
