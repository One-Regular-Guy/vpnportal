CREATE TABLE users (
    id UUID PRIMARY KEY,
    name varchar(255) not null unique
    email varchar(255) not null unique,
    password varchar(255) not null
);
CREATE TABLE clients (
    uid varchar(49) PRIMARY KEY,
    ou varchar(26) not null,
    dc1 varchar(100) not null,
    dc2 varchar(100) not null,
    cn varchar(255) not null unique,
    sn varchar(255) not null unique,
    mail varchar(255) not null unique,
    actual_download_id varchar(50) unique,
    certificate text unique,
    totp_secret varchar(7) unique,
    password varchar(255) not null,
    is_ldap boolean not null,
    is_active boolean not null
);