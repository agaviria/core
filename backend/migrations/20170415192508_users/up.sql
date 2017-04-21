CREATE TABLE auth (
    id serial PRIMARY KEY,
    salt bytea NOT NULL,
    pwd bytea NOT NULL,
    date_created TIMESTAMPTZ DEFAULT NOW() NOT NULL, 
    date_modified TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

CREATE TABLE users ( 
    id SERIAL PRIMARY KEY,
    auth_id INTEGER NOT NULL UNIQUE REFERENCES "auth" ("id") ON DELETE CASCADE, 
    email text UNIQUE NOT NULL, 
    uuid_ uuid UNIQUE NOT NULL,
    level_ INTEGER NOT NULL CHECK (level_ >= 0),
    date_created TIMESTAMPTZ DEFAULT NOW() NOT NULL, 
    date_modified TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

CREATE UNIQUE INDEX on users (uuid_);
CREATE UNIQUE INDEX on users (email);
