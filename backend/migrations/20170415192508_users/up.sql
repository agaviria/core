CREATE TABLE users ( 
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email text UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    modified_on TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

CREATE UNIQUE INDEX on users (email);
