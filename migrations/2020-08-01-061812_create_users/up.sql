CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username TEXT NOT NULL,
    email VARCHAR(255) NOT NULL,
    UNIQUE (username, email),
    password TEXT NOT NULL,
    first_name TEXT,
    last_name TEXT,
    phone_number VARCHAR(15),
    dob TIMESTAMP NOT NULL,
    role VARCHAR(20),
    roles TEXT,
    avatar TEXT,
    time_zone TEXT,
    created_by TEXT,
    created_time_dt TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_by TEXT,
    updated_time_dt TIMESTAMP NOT NULL DEFAULT NOW(),
    status smallint NOT NULL DEFAULT '0',
    confirm_code VARCHAR(6),
    confirm_code_created_time_dt TIMESTAMP NOT NULL
)