CREATE TABLE users (
    id UUID PRIMARY KEY,
    name varchar(255) not null unique not null unique,
    email varchar(255) not null unique not null unique,
    password varchar(255) not null unique not null
);
CREATE TABLE clients (
    uid varchar(255) not null unique PRIMARY KEY,
    ou varchar(255) not null,
    dc1 varchar(255) not null,
    dc2 varchar(255) not null,
    cn varchar(255) not null unique,
    sn varchar(255) not null unique,
    mail varchar(255) not null unique,
    password varchar(255) not null,
    is_ldap boolean not null,
    is_active boolean not null
);