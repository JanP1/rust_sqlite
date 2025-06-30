CREATE TABLE orders (
  id INTEGER PRIMARY KEY,
  order_id INTEGER NOT NULL,
  client_id INTEGER NOT NULL,
  dish_id INTEGER NOT NULL,
  FOREIGN KEY(client_id) REFERENCES clients(id) ON DELETE CASCADE,
  FOREIGN KEY(dish_id) REFERENCES dishes(id)
);
