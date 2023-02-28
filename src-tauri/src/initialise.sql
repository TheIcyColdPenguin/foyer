CREATE TABLE IF NOT EXISTS photos (
    id INTEGER PRIMARY KEY,
    img_data BLOB NOT NULL,
    timestamp DATETIME DEFAULT NOW
);

CREATE TABLE IF NOT EXISTS labels (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    colour TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS photos_labels (
    id INTEGER PRIMARY KEY,
    photo_id INTEGER NOT NULL,
    label_id INTEGER NOT NULL,
    FOREIGN KEY (photo_id) REFERENCES photos(id),
    FOREIGN KEY (label_id) REFERENCES labels(id) 
);