﻿--
-- PostgreSQL database dump
--

-- Dumped from database version 15.2
-- Dumped by pg_dump version 15.2

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: uuid-ossp; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;


--
-- Name: EXTENSION "uuid-ossp"; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION "uuid-ossp" IS 'generate universally unique identifiers (UUIDs)';


--
-- Name: account_type; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.account_type AS ENUM (
    'student',
    'teacher',
    'parent'
);


ALTER TYPE public.account_type OWNER TO postgres;

--
-- Name: delete_embed_func(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.delete_embed_func() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
	delete from media where uuid=OLD."embed_uuid";
	return null;
end;
$$;


ALTER FUNCTION public.delete_embed_func() OWNER TO postgres;

--
-- Name: delete_profile_picture_func(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.delete_profile_picture_func() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
delete from media where uuid=OLD."profile_picture_uuid";
return null;
end;
$$;


ALTER FUNCTION public.delete_profile_picture_func() OWNER TO postgres;

--
-- Name: delete_thumbnail_func(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.delete_thumbnail_func() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
	delete from media where uuid=OLD."thumbnail_uuid";
	return null;
end;
$$;


ALTER FUNCTION public.delete_thumbnail_func() OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: account; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.account (
    id integer NOT NULL,
    username character varying(20) NOT NULL,
    password_hash character(128) NOT NULL,
    profile_picture_uuid uuid,
    account_type public.account_type NOT NULL
);


ALTER TABLE public.account OWNER TO postgres;

--
-- Name: account_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.account_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.account_id_seq OWNER TO postgres;

--
-- Name: account_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.account_id_seq OWNED BY public.account.id;


--
-- Name: media; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.media (
    uuid uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    path character varying NOT NULL
);


ALTER TABLE public.media OWNER TO postgres;

--
-- Name: question; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.question (
    qwiz_id integer NOT NULL,
    index integer NOT NULL,
    body character varying(500) NOT NULL,
    answer1 character varying(200) NOT NULL,
    answer2 character varying(200) NOT NULL,
    answer3 character varying(200),
    answer4 character varying(200),
    correct smallint NOT NULL,
    embed_uuid uuid,
    CONSTRAINT correct_check CHECK (((correct >= 1) AND (((correct <= 4) AND (answer4 IS NOT NULL)) OR ((correct <= 3) AND (answer3 IS NOT NULL)) OR (correct <= 2)))),
    CONSTRAINT index_check CHECK ((index >= 0)),
    CONSTRAINT question4_check CHECK (((answer4 IS NULL) OR (answer3 IS NOT NULL)))
);


ALTER TABLE public.question OWNER TO postgres;

--
-- Name: qwiz; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.qwiz (
    id integer NOT NULL,
    name character varying(100) NOT NULL,
    creator_id integer NOT NULL,
    thumbnail_uuid uuid
);


ALTER TABLE public.qwiz OWNER TO postgres;

--
-- Name: qwiz_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.qwiz_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.qwiz_id_seq OWNER TO postgres;

--
-- Name: qwiz_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.qwiz_id_seq OWNED BY public.qwiz.id;


--
-- Name: account id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.account ALTER COLUMN id SET DEFAULT nextval('public.account_id_seq'::regclass);


--
-- Name: qwiz id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qwiz ALTER COLUMN id SET DEFAULT nextval('public.qwiz_id_seq'::regclass);


--
-- Data for Name: account; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.account (username, password_hash, profile_picture_uuid, account_type, id) FROM stdin;
\.


--
-- Data for Name: media; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.media (uuid, path) FROM stdin;
\.


--
-- Data for Name: question; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.question (qwiz_id, index, body, answer1, answer2, answer3, answer4, correct, embed_uuid) FROM stdin;
\.


--
-- Data for Name: qwiz; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.qwiz (id, name, creator_id, thumbnail_uuid) FROM stdin;
\.


--
-- Name: account account_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.account
    ADD CONSTRAINT account_pkey PRIMARY KEY (id);


--
-- Name: account account_username_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.account
    ADD CONSTRAINT account_username_key UNIQUE (username);


--
-- Name: media media_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.media
    ADD CONSTRAINT media_pkey PRIMARY KEY (uuid);


--
-- Name: question question_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.question
    ADD CONSTRAINT question_pkey PRIMARY KEY (qwiz_id, index);


--
-- Name: qwiz qwiz_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qwiz
    ADD CONSTRAINT qwiz_pkey PRIMARY KEY (id);


--
-- Name: question delete_embed; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER delete_embed AFTER DELETE ON public.question FOR EACH ROW EXECUTE FUNCTION public.delete_embed_func();


--
-- Name: account delete_profile_picture; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER delete_profile_picture AFTER DELETE ON public.account FOR EACH ROW EXECUTE FUNCTION public.delete_profile_picture_func();


--
-- Name: qwiz delete_thumbnail; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER delete_thumbnail AFTER DELETE ON public.qwiz FOR EACH ROW EXECUTE FUNCTION public.delete_thumbnail_func();


--
-- Name: account account_profile_picture_uuid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.account
    ADD CONSTRAINT account_profile_picture_uuid_fkey FOREIGN KEY (profile_picture_uuid) REFERENCES public.media(uuid) ON DELETE SET NULL;


--
-- Name: question question_embed_uuid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.question
    ADD CONSTRAINT question_embed_uuid_fkey FOREIGN KEY (embed_uuid) REFERENCES public.media(uuid) ON DELETE SET NULL;


--
-- Name: question question_qwiz_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.question
    ADD CONSTRAINT question_qwiz_id_fkey FOREIGN KEY (qwiz_id) REFERENCES public.qwiz(id) ON DELETE CASCADE;


--
-- Name: qwiz qwiz_creator_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qwiz
    ADD CONSTRAINT qwiz_creator_id_fkey FOREIGN KEY (creator_id) REFERENCES public.account(id) ON DELETE CASCADE;


--
-- Name: qwiz qwiz_thumbnail_uuid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qwiz
    ADD CONSTRAINT qwiz_thumbnail_uuid_fkey FOREIGN KEY (thumbnail_uuid) REFERENCES public.media(uuid) ON DELETE SET NULL;


--
-- Name: SCHEMA public; Type: ACL; Schema: -; Owner: pg_database_owner
--

GRANT USAGE ON SCHEMA public TO qwiz;


--
-- Name: TABLE account; Type: ACL; Schema: public; Owner: postgres
--

GRANT ALL ON TABLE public.account TO qwiz;


--
-- Name: TABLE media; Type: ACL; Schema: public; Owner: postgres
--

GRANT ALL ON TABLE public.media TO qwiz;


--
-- Name: TABLE question; Type: ACL; Schema: public; Owner: postgres
--

GRANT ALL ON TABLE public.question TO qwiz;


--
-- Name: TABLE qwiz; Type: ACL; Schema: public; Owner: postgres
--

GRANT ALL ON TABLE public.qwiz TO qwiz;


--
-- PostgreSQL database dump complete
--

