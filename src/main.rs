use sql_connection::{ restaurant_functions::database_functionality::{self, get_ingredient_by_name}};

fn main() {

    let connection = &mut database_functionality::establish_connection();

    let ingredient_name : &str = "Tomato";
    let _ = match  get_ingredient_by_name(connection, ingredient_name){
        Ok(ingredient) => {
            if let Some(ing_name) = ingredient.get(0) {
                println!("Ingredient name {}", ing_name.name);
            } else {
                println!("No ingredient with name: {}", ingredient_name);
            }

        }
        Err(e) => {
            println!("{}", e);
        }
    };

}
