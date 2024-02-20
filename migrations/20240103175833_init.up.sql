CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    "posts" (
        id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
        title VARCHAR(255) NOT NULL,
        description VARCHAR(4095) NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );


CREATE TYPE person_role AS ENUM ('admin', 'user');

CREATE TABLE
    "people" (
        id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
        username VARCHAR(255) NOT NULL UNIQUE,
        firstname VARCHAR(100) NOT NULL,
        lastname VARCHAR(100) NOT NULL,
        password VARCHAR(255) NOT NULL,
        role person_role NOT NULL DEFAULT 'user',
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );


CREATE TABLE
    "emails" (
        id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
        person_id UUID NOT NULL,
        address VARCHAR(255) NOT NULL,
        CONSTRAINT fk_person FOREIGN KEY(person_id) REFERENCES people(id) ON DELETE CASCADE
    );
