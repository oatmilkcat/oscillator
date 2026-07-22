CREATE TABLE IF NOT EXISTS images (
    id TEXT PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL,
    created_by TEXT REFERENCES users(id),
    updated_at TIMESTAMPTZ,
    updated_by TEXT REFERENCES users(id),
    description TEXT,
    width_px INTEGER NOT NULL,
    height_px INTEGER NOT NULL,   
    url TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS album_images (
    album_id TEXT NOT NULL REFERENCES albums(id),
    image_id TEXT NOT NULL REFERENCES images(id),
    PRIMARY KEY (album_id, image_id)
);

CREATE TABLE IF NOT EXISTS artist_images (
    artist_id TEXT NOT NULL REFERENCES artists(id),
    image_id TEXT NOT NULL REFERENCES images(id),
    PRIMARY KEY (artist_id, image_id)
);
