use sql_connection::restaurant_functions::database_functionality::{self, get_order_by_id};


fn main() {

    let connection = &mut database_functionality::establish_connection();
    let _ = match  get_order_by_id(connection, 3002){
        Ok(_) => {

        }
        Err(e) => {
            println!("{}", e);
        }       
    };

}
