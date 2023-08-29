--
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

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: account; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.account (
    uuid uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    username character varying(20) NOT NULL,
    password_hash character(128) NOT NULL,
    profile_picture_uuid uuid
);


ALTER TABLE public.account OWNER TO postgres;

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
    qwiz_uuid uuid NOT NULL,
    body character varying(500) NOT NULL,
    answers character varying(100)[] DEFAULT '{}'::character varying[] NOT NULL,
    media_uuid uuid
);


ALTER TABLE public.question OWNER TO postgres;

--
-- Name: qwiz; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.qwiz (
    uuid uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    name character varying(100) NOT NULL,
    creator_uuid uuid NOT NULL,
    thumbnail_uuid uuid
);


ALTER TABLE public.qwiz OWNER TO postgres;

--
-- Data for Name: account; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.account (uuid, username, password_hash, profile_picture_uuid) FROM stdin;
\.


--
-- Data for Name: media; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.media (uuid, path) FROM stdin;
\.


--
-- Data for Name: question; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.question (qwiz_uuid, body, answers, media_uuid) FROM stdin;
\.


--
-- Data for Name: qwiz; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.qwiz (uuid, name, creator_uuid, thumbnail_uuid) FROM stdin;
\.


--
-- Name: account account_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.account
    ADD CONSTRAINT account_pkey PRIMARY KEY (uuid);


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
-- Name: question question_media_uuid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.question
    ADD CONSTRAINT question_media_uuid_key UNIQUE (media_uuid);


--
-- Name: qwiz qwiz_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qwiz
    ADD CONSTRAINT qwiz_pkey PRIMARY KEY (uuid);


--
-- Name: qwiz qwiz_thumbnail_uuid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qwiz
    ADD CONSTRAINT qwiz_thumbnail_uuid_key UNIQUE (thumbnail_uuid);


--
-- Name: SCHEMA public; Type: ACL; Schema: -; Owner: pg_database_owner
--

GRANT USAGE ON SCHEMA public TO qwiz;


--
-- Name: TABLE account; Type: ACL; Schema: public; Owner: postgres
--

GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE public.account TO qwiz;


--
-- Name: TABLE media; Type: ACL; Schema: public; Owner: postgres
--

GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE public.media TO qwiz;


--
-- Name: TABLE question; Type: ACL; Schema: public; Owner: postgres
--

GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE public.question TO qwiz;


--
-- Name: TABLE qwiz; Type: ACL; Schema: public; Owner: postgres
--

GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE public.qwiz TO qwiz;


--
-- PostgreSQL database dump complete
--

