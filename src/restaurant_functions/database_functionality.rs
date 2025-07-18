use crate::{model::{Customer, Dish, FullOrder, Ingredient, Order}, schema::orders::dsl::*};
use dotenvy::{dotenv};
use diesel::{insert_into, prelude::*};
use std::env;
use diesel::result::Error;

// ========================================================================
// ====================== Connecting to the database ======================

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// ========================================================================
// ========================================================================

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

    let mut dishes_in_order: Vec<Dish> = vec![];

    for ord in results {
        match  get_dish_by_id(connection, ord.dish_id)
        {
            Ok(dish_exists) =>{
                if let Some(first_dish) = dish_exists.get(0) {
                    dishes_in_order.push(first_dish.clone());
                } else {
                    println!("No dish found with id: {}", ord.dish_id);
                }
            }
            Err(e) => {println!("{}", e);}
            
        }

    }

    let full_order = FullOrder{
        customer_name: "John Doe".to_string(),
        dishes: dishes_in_order,
        order_id: 5,
        

    };

    for dish_in_ord in &full_order.dishes {
        println!("Dish {}", dish_in_ord.name);
    }
    
    Ok(full_order)
}



pub fn get_ingredient_by_id(connection: &mut SqliteConnection, searched_ingredient_id: i32) -> Result<Vec<Ingredient>, Error> {
    use crate::schema::ingredients::dsl::*;
    let results = ingredients
        .filter(id.eq(searched_ingredient_id))
        .limit(1)
        .select(Ingredient::as_select())
        .load(connection)?; 

    Ok(results)
}


pub fn get_ingredient_by_name(connection: &mut SqliteConnection, searched_name: &str) -> Result<Vec<Ingredient>, Error> {
    use crate::schema::ingredients::dsl::*;
    let results = ingredients
        .filter(name.eq(searched_name))
        .limit(1)
        .select(Ingredient::as_select())
        .load(connection)?; 

    Ok(results)
}


pub fn add_ingredient(connection: &mut SqliteConnection, ingredient_name: &str, ingredient_kcal: i32) -> Result<String, String> {
    use crate::schema::ingredients::dsl::*;

    let _ = insert_into(ingredients)
        .values((
            name.eq(ingredient_name),
            kcal.eq(ingredient_kcal)

        ))
        .execute(connection);

    Ok("Added ingredient".to_string())
}


// Getting length of table


pub fn get_num_of_rows(connection: &mut SqliteConnection){

}

