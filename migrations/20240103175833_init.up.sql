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

