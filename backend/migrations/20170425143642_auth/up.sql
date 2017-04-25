-- Your SQL goes here
CREATE TABLE auth (
    id serial PRIMARY KEY,
    salt bytea NOT NULL,
    pwd bytea NOT NULL,
    date_created TIMESTAMPTZ DEFAULT NOW() NOT NULL, 
    date_modified TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

INSERT INTO auth (id, salt, pwd, date_created, date_modified) VALUES(201, decode('C6BE0D6DFC70198BF88E36520B8FC4A4DCB6353D', 'hex'), decode('12F3EA5F4E9CC2AA5C097851E7767682AAD1C06D', 'hex'), NOW(), NOW());
