CREATE TABLE dish_ingredient_conn (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    dish_id INTEGER NOT NULL,
    ingredient_id INTEGER NOT NULL,
    amount REAL,             -- amount as a number, e.g., 1.5
    unit TEXT,               -- optional, e.g., "grams", "cups"
    FOREIGN KEY(dish_id) REFERENCES dishes(id) ON DELETE CASCADE,
    FOREIGN KEY(ingredient_id) REFERENCES ingredients(id) ON DELETE CASCADE,
    UNIQUE(dish_id, ingredient_id)
);
