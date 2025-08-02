-- Active: 1738854740125@@127.0.0.1@5432@postgres
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL
);

INSERT INTO users (name, email) VALUES ('Alice', 'alice@example.com');
INSERT INTO users (name, email) VALUES ('Hermann', 'info@wennhmann.com');
INSERT INTO users (name, email) VALUES ('Paula', 'paula@paula.com');