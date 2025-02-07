-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS feedbacks (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        rating INTEGER NOT NULL,
        text TEXT NOT NULL UNIQUE,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );

CREATE TABLE
    IF NOT EXISTS roles (
        id SERIAL PRIMARY KEY NOT NULL,
        code TEXT NOT NULL UNIQUE
    );

INSERT INTO roles (id, code) values 
    (1, 'ADMIN'), 
    (2, 'USER');

CREATE TABLE
    IF NOT EXISTS users (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        name INTEGER NOT NULL,
        password TEXT NOT NULL UNIQUE,
        role_id INT NOT NULL REFERENCES roles(id),
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
    );

