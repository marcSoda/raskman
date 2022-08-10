-- Your SQL goes here
CREATE TABLE users (
       uid SERIAL NOT NULL PRIMARY KEY,
       name VARCHAR(128) NOT NULL,
       login VARCHAR(256) NOT NULL,
       hashword VARCHAR(256) NOT NULL,
);
