-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS refresh_tokens;

DROP TABLE IF EXISTS authorization_codes;

DROP TABLE IF EXISTS access_tokens;

DROP TABLE IF EXISTS clients;

DROP TABLE IF EXISTS users;

DROP EXTENSION IF EXISTS "uuid-ossp";