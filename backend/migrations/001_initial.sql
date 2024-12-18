DROP SCHEMA IF EXISTS bidding CASCADE;
CREATE SCHEMA bidding;

CREATE TABLE bidding.users (
	id          BIGSERIAL PRIMARY KEY,
	email       VARCHAR(200) NOT NULL UNIQUE,
	password    VARCHAR(200) NOT NULL,
	username    VARCHAR(50)  NOT NULL,
	created_at  timestamp with time zone DEFAULT now() NOT NULL,
	updated_at  timestamp with time zone DEFAULT now() NOT NULL
);
