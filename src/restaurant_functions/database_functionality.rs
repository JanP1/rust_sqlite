use crate::{model::{Customer, Dish, FullOrder, Order}, schema::{ orders::dsl::*}};
use dotenvy::{dotenv};
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
        .order(id.desc())
        .limit(20)
        .select(Customer::as_select())
        .load(connection)?; 

    Ok(results)
}

pub fn load_orders(connection: &mut SqliteConnection) -> Result<Vec<Order>, Error> {

    let results = orders
        .limit(5)
        .select(Order::as_select())
        .load(connection)?; 

    Ok(results)
}

pub fn get_dish_by_id(connection: &mut SqliteConnection, searched_dish_id: i32) -> Result<Vec<Dish>, Error> {
    use crate::schema::dishes::dsl::*;

    let results = dishes
        .filter(id.eq(searched_dish_id))
        .limit(1)
        .select(Dish::as_select())
        .load(connection)?; 

    Ok(results)
}

pub fn get_order_by_id(connection: &mut SqliteConnection, searched_order_id: i32) -> Result<FullOrder, Error> {

    let results: Vec<Order> = orders
        .filter(order_id.eq(searched_order_id))
        .select(Order::as_select())
        .load(connection)?; 

    for ord in results {
        println!("Dish {}", ord.dish_id);
        println!("-----------\n");
    }

    let dish = Dish {

        id: 12,
        name: "Dish1".to_string(),
        preparation_time: 23,
    };

    let full_order = FullOrder{
        customer_name: "John Doe".to_string(),
        dishes: vec![dish],
        order_id: 5,
        

    };
    
    Ok(full_order)
}




