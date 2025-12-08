CREATE TABLE user (
    id UUID PRIMARY KEY
    name varchar(255) not null unique,
    email varchar(255) not null unique,
    password varchar(255) not null
)