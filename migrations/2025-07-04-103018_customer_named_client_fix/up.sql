DROP TABLE orders;


CREATE TABLE orders (
  id INTEGER NOT NULL PRIMARY KEY,
  order_id INTEGER NOT NULL,
  customer_id INTEGER NOT NULL,
  dish_id INTEGER NOT NULL,
  FOREIGN KEY(customer_id) REFERENCES customers(id) ON DELETE CASCADE,
  FOREIGN KEY(dish_id) REFERENCES dishes(id)
);
