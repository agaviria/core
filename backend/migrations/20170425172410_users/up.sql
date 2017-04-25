-- required extension for uuid function
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users ( 
    id SERIAL PRIMARY KEY,
    auth_id INTEGER NOT NULL UNIQUE REFERENCES "auth" ("id") ON DELETE CASCADE, 
    email text UNIQUE NOT NULL, 
    uuid_ UUID UNIQUE NOT NULL,
    level_ INTEGER NOT NULL CHECK (level_ >= 0),
    date_created TIMESTAMPTZ DEFAULT NOW() NOT NULL, 
    date_modified TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

CREATE UNIQUE INDEX on users (uuid_);
CREATE UNIQUE INDEX on users (email);

INSERT INTO users (id, auth_id, email, uuid_, level_, date_created, date_modified) VALUES(101, 201, 'david@email.com', uuid_generate_v4(), 5, NOW(), NOW()); 
