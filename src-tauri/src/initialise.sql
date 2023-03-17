CREATE TABLE IF NOT EXISTS photos (
    id INTEGER PRIMARY KEY,
    img_data BLOB NOT NULL,
    extension TEXT,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_photos_timestamp on photos(timestamp);

CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    colour TEXT DEFAULT "#bb86db"
);
CREATE INDEX IF NOT EXISTS idx_tags_name on tags(name);

CREATE TABLE IF NOT EXISTS photos_tags (
    photo_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    FOREIGN KEY (photo_id) REFERENCES photos(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id),
    PRIMARY KEY (photo_id, tag_id)
);
CREATE INDEX IF NOT EXISTS idx_photos_tags_photo_id on photos_tags(photo_id);
CREATE INDEX IF NOT EXISTS idx_photos_tags_tag_id on photos_tags(tag_id);