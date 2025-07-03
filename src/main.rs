use sql_connection::model::Customer;

use sql_connection::restaurant_functions;
use diesel::prelude::*;


fn main() {
    use sql_connection::schema::customers::dsl::*;

    let connection = &mut restaurant_functions::establish_connection();
    let results = customers
        .limit(5)
        .select(Customer::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} customers", results.len());
    for customer in results {
        println!("{}", customer.customer_name);
        println!("{}", customer.id);
        println!("-----------\n");
    }
}
