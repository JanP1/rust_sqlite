use crate::model::Customer;
use dotenvy::dotenv;
use diesel::prelude::*;
use std::env;
use diesel::result::Error;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn load_customers(connection: &mut SqliteConnection) -> Result<Vec<Customer>, Error> {
    use crate::schema::customers::dsl::*;

    let results = customers
        .limit(5)
        .select(Customer::as_select())
        .load(connection)?; 

    Ok(results)
}
