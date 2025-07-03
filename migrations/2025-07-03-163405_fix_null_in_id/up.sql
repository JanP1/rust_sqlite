DROP TABLE IF EXISTS customers;
CREATE TABLE customers (
  id INTEGER PRIMARY KEY NOT NULL,
  customer_name TEXT NOT NULL

);


DROP TABLE IF EXISTS dish_ingredient_conn;
CREATE TABLE dish_ingredient_conn (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    dish_id INTEGER NOT NULL,
    ingredient_id INTEGER NOT NULL,
    amount REAL,
    unit TEXT,
    FOREIGN KEY(dish_id) REFERENCES dishes(id) ON DELETE CASCADE,
    FOREIGN KEY(ingredient_id) REFERENCES ingredients(id) ON DELETE CASCADE,
    UNIQUE(dish_id, ingredient_id)
);


DROP TABLE if EXISTS orders;
CREATE TABLE orders (
  id INTEGER NOT NULL PRIMARY KEY,
  order_id INTEGER NOT NULL,
  client_id INTEGER NOT NULL,
  dish_id INTEGER NOT NULL,
  FOREIGN KEY(client_id) REFERENCES clients(id) ON DELETE CASCADE,
  FOREIGN KEY(dish_id) REFERENCES dishes(id)
);
