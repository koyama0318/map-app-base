CREATE SCHEMA account;

CREATE TABLE account.Users (
    ID TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    longitude REAL,
    latitude REAL,
    PRIMARY KEY (id)
);

CREATE TABLE account.Drivers (
    ID TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    longitude REAL,
    latitude REAL,
    PRIMARY KEY (id)
);
