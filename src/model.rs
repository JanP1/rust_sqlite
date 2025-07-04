use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::customers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Customer {
    pub id: i32,
    pub customer_name: String,
}


#[derive(Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::dishes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Dish {
    pub id: i32,
    pub name: String,
    pub preparation_time: i32,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::ingredients)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub kcal: i32,
}




#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::orders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Order {
    pub id: i32,
    pub order_id: i32,
    pub customer_id: i32,
    pub dish_id: i32,
}


// ====================================================


pub struct FullOrder {
    pub order_id: i32,
    pub customer_name: String,
    pub dishes: Vec<Dish>,
}
