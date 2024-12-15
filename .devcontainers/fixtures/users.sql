-- Create table users
CREATE SEQUENCE users_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE CACHE 1;

CREATE TABLE person
(
    id           integer                DEFAULT nextval('users_id_seq'::regclass) NOT NULL,
    firstname    character varying(255) DEFAULT NULL:: character varying,
    lastname     character varying(255) DEFAULT NULL:: character varying,
		email        character varying(255) DEFAULT NULL:: character varying,
		username     character varying(255) DEFAULT NULL:: character varying,
		created_at   timestamp with time zone DEFAULT now() NOT NULL,
		updated_at   timestamp with time zone DEFAULT now() NOT NULL
);

ALTER TABLE ONLY person ADD CONSTRAINT person_pkey PRIMARY KEY (id);
