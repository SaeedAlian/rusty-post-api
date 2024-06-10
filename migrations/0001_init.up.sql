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
CREATE TYPE gender AS ENUM ('male', 'female');

CREATE TABLE
    "people" (
        id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
        username VARCHAR(255) NOT NULL UNIQUE,
        firstname VARCHAR(100) NOT NULL,
        lastname VARCHAR(100) NOT NULL,
        password VARCHAR(255) NOT NULL,
        role person_role NOT NULL DEFAULT 'user',
        gender gender NOT NULL,
        birthdate DATE NOT NULL,
        biography VARCHAR(1024),
        is_profile_private BOOLEAN NOT NULL DEFAULT false,
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
        owner_id UUID NOT NULL,
        address VARCHAR(255) NOT NULL UNIQUE,
        is_primary BOOLEAN NOT NULL DEFAULT false,
        is_verified BOOLEAN NOT NULL DEFAULT false,
        is_private BOOLEAN NOT NULL DEFAULT false,
        updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),

        CONSTRAINT fk_owner FOREIGN KEY(owner_id) REFERENCES people(id) ON DELETE CASCADE
    );
