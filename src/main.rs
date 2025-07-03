
use sql_connection::{restaurant_functions::{self, load_customers}};


fn main() {

    let connection = &mut restaurant_functions::establish_connection();
    match load_customers(connection) {
        Ok(custom) => {
            println!("Displaying {} customers", custom.len());
                for customer in custom {
                    println!("{}", customer.customer_name);
                    println!("{}", customer.id);
                    println!("-----------\n");
                }

        }
        Err(e) => {
            println!("Error loading customers {}", e);
        }
        
    }

}
