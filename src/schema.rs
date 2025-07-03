// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Integer,
        customer_name -> Text,
    }
}

diesel::table! {
    dish_ingredient_conn (id) {
        id -> Integer,
        dish_id -> Integer,
        ingredient_id -> Integer,
        amount -> Nullable<Float>,
        unit -> Nullable<Text>,
    }
}

diesel::table! {
    dishes (id) {
        id -> Integer,
        name -> Text,
        preparation_time -> Integer,
    }
}

diesel::table! {
    ingredients (id) {
        id -> Integer,
        name -> Text,
        kcal -> Integer,
    }
}

diesel::table! {
    orders (id) {
        id -> Integer,
        order_id -> Integer,
        client_id -> Integer,
        dish_id -> Integer,
    }
}

diesel::joinable!(dish_ingredient_conn -> dishes (dish_id));
diesel::joinable!(dish_ingredient_conn -> ingredients (ingredient_id));
diesel::joinable!(orders -> dishes (dish_id));

diesel::allow_tables_to_appear_in_same_query!(
    customers,
    dish_ingredient_conn,
    dishes,
    ingredients,
    orders,
);
