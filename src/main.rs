use sql_connection::{ restaurant_functions::database_functionality::{self}};

fn main() {

    let connection = &mut database_functionality::establish_connection();

    // let _ = match database_functionality::add_ingredient(connection, "Ingredient1", 90){
    //     Ok(_) => {}
    //     Err(e) => {println!("Error while creating ingredient. {}", e);}
    //
    // };
    let ingredient_name : &str = "Ingredient1";
    let _ = match  database_functionality::get_ingredient_by_name(connection, ingredient_name){
        Ok(ingredient) => {
            if let Some(ing_name) = ingredient.get(0) {
                println!("Ingredient name {}", ing_name.name);
                println!("Ingredient kcal {}", ing_name.kcal);
            } else {
                println!("No ingredient with name: {}", ingredient_name);
            }

        }
        Err(e) => {
            println!("{}", e);
        }
    };

}


