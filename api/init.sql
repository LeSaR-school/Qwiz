--
-- PostgreSQL database dump
--

-- Dumped from database version 14.9 (Ubuntu 14.9-0ubuntu0.22.04.1)
-- Dumped by pg_dump version 14.9 (Ubuntu 14.9-0ubuntu0.22.04.1)

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
-- Name: account_type; Type: TYPE; Schema: public; Owner: qwiz
--

CREATE TYPE public.account_type AS ENUM (
    'student',
    'teacher',
    'parent'
);


ALTER TYPE public.account_type OWNER TO qwiz;

--
-- Name: media_type; Type: TYPE; Schema: public; Owner: qwiz
--

CREATE TYPE public.media_type AS ENUM (
    'image',
    'video',
    'audio',
    'youtube'
);


ALTER TYPE public.media_type OWNER TO qwiz;

--
-- Name: check_student_class_func(); Type: FUNCTION; Schema: public; Owner: qwiz
--

CREATE FUNCTION public.check_student_class_func() RETURNS trigger
    LANGUAGE plpgsql
    AS $$

declare
type account_type;
class integer;
begin

select account_type into type from account where id=NEW."student_id";
select class_id into class from assignment where id=NEW."assignment_id";

if type!='student' then
raise exception 'User with id % is not a student', NEW."student_id";
elsif NEW."student_id" NOT IN (SELECT student_id FROM student WHERE class_id=class) then
raise exception 'User with id % is not in class id %', NEW."student_id", class;
end if;

return NEW;

end;

$$;


ALTER FUNCTION public.check_student_class_func() OWNER TO qwiz;

--
-- Name: check_student_func(); Type: FUNCTION; Schema: public; Owner: qwiz
--

CREATE FUNCTION public.check_student_func() RETURNS trigger
    LANGUAGE plpgsql
    AS $$

declare
type account_type;
begin

select account_type into type from account where id=NEW."student_id";
if type!='student' then
raise exception 'User with id % is not a student', NEW."student_id";
end if;

return NEW;

end;

$$;


ALTER FUNCTION public.check_student_func() OWNER TO qwiz;

--
-- Name: check_teacher_func(); Type: FUNCTION; Schema: public; Owner: qwiz
--

CREATE FUNCTION public.check_teacher_func() RETURNS trigger
    LANGUAGE plpgsql
    AS $$

declare
type account_type;
begin

select account_type into type from account where id=NEW."teacher_id";
if type!='teacher' then
raise exception 'User with id % is not a teacher', NEW."teacher_id";
end if;

return NEW;

end;

$$;


ALTER FUNCTION public.check_teacher_func() OWNER TO qwiz;

--
-- Name: delete_embed_func(); Type: FUNCTION; Schema: public; Owner: qwiz
--

CREATE FUNCTION public.delete_embed_func() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
	delete from media where uuid=OLD."embed_uuid";
	return null;
end;
$$;


ALTER FUNCTION public.delete_embed_func() OWNER TO qwiz;

--
-- Name: delete_profile_picture_func(); Type: FUNCTION; Schema: public; Owner: qwiz
--

CREATE FUNCTION public.delete_profile_picture_func() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
delete from media where uuid=OLD."profile_picture_uuid";
return null;
end;
$$;


ALTER FUNCTION public.delete_profile_picture_func() OWNER TO qwiz;

--
-- Name: delete_thumbnail_func(); Type: FUNCTION; Schema: public; Owner: qwiz
--

CREATE FUNCTION public.delete_thumbnail_func() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
	delete from media where uuid=OLD."thumbnail_uuid";
	return null;
end;
$$;


ALTER FUNCTION public.delete_thumbnail_func() OWNER TO qwiz;

--
-- Name: update_account_type_func(); Type: FUNCTION; Schema: public; Owner: qwiz
--

CREATE FUNCTION public.update_account_type_func() RETURNS trigger
    LANGUAGE plpgsql
    AS $$

begin

if OLD."account_type"='teacher' AND NEW."account_type"!='teacher' then
DELETE FROM class WHERE teacher_id=NEW."id";
elsif OLD."account_type"='student' AND NEW."account_type"!='student' then
DELETE FROM student WHERE student_id=NEW."id";
end if;

return null;

end;

$$;


ALTER FUNCTION public.update_account_type_func() OWNER TO qwiz;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: account; Type: TABLE; Schema: public; Owner: qwiz
--

CREATE TABLE public.account (
    username character varying(20) NOT NULL,
    password_hash character(128) NOT NULL,
    profile_picture_uuid uuid,
    account_type public.account_type NOT NULL,
    id integer NOT NULL
);


ALTER TABLE public.account OWNER TO qwiz;

--
-- Name: account_id_seq; Type: SEQUENCE; Schema: public; Owner: qwiz
--

ALTER TABLE public.account ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.account_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: assignment; Type: TABLE; Schema: public; Owner: qwiz
--

CREATE TABLE public.assignment (
    qwiz_id integer NOT NULL,
    class_id integer NOT NULL,
    open_time timestamp without time zone,
    close_time timestamp without time zone,
    id integer NOT NULL
);


ALTER TABLE public.assignment OWNER TO qwiz;

--
-- Name: assignment_id_seq; Type: SEQUENCE; Schema: public; Owner: qwiz
--

ALTER TABLE public.assignment ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.assignment_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: class; Type: TABLE; Schema: public; Owner: qwiz
--

CREATE TABLE public.class (
    teacher_id integer NOT NULL,
    name character varying(100) NOT NULL,
    id integer NOT NULL
);


ALTER TABLE public.class OWNER TO qwiz;

--
-- Name: class_id_seq; Type: SEQUENCE; Schema: public; Owner: qwiz
--

ALTER TABLE public.class ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.class_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: completed_assignment; Type: TABLE; Schema: public; Owner: qwiz
--

CREATE TABLE public.completed_assignment (
    assignment_id integer NOT NULL,
    student_id integer NOT NULL
);


ALTER TABLE public.completed_assignment OWNER TO qwiz;

--
-- Name: media; Type: TABLE; Schema: public; Owner: qwiz
--

CREATE TABLE public.media (
    uuid uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    uri character varying NOT NULL,
    media_type public.media_type NOT NULL
);


ALTER TABLE public.media OWNER TO qwiz;

--
-- Name: question; Type: TABLE; Schema: public; Owner: qwiz
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


ALTER TABLE public.question OWNER TO qwiz;

--
-- Name: qwiz; Type: TABLE; Schema: public; Owner: qwiz
--

CREATE TABLE public.qwiz (
    name character varying(100) NOT NULL,
    creator_id integer NOT NULL,
    thumbnail_uuid uuid,
    public boolean DEFAULT true NOT NULL,
    create_time timestamp without time zone DEFAULT (now() AT TIME ZONE 'UTC'::text) NOT NULL,
    id integer NOT NULL
);


ALTER TABLE public.qwiz OWNER TO qwiz;

--
-- Name: qwiz_id_seq; Type: SEQUENCE; Schema: public; Owner: qwiz
--

ALTER TABLE public.qwiz ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.qwiz_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: student; Type: TABLE; Schema: public; Owner: qwiz
--

CREATE TABLE public.student (
    student_id integer NOT NULL,
    class_id integer NOT NULL
);


ALTER TABLE public.student OWNER TO qwiz;

--
-- Name: vote; Type: TABLE; Schema: public; Owner: qwiz
--

CREATE TABLE public.vote (
    voter_id integer NOT NULL,
    qwiz_id integer NOT NULL
);


ALTER TABLE public.vote OWNER TO qwiz;

--
-- Name: account account_pkey; Type: CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.account
    ADD CONSTRAINT account_pkey PRIMARY KEY (id);


--
-- Name: account account_username_key; Type: CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.account
    ADD CONSTRAINT account_username_key UNIQUE (username);


--
-- Name: assignment assignment_pkey; Type: CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.assignment
    ADD CONSTRAINT assignment_pkey PRIMARY KEY (id);


--
-- Name: class class_pkey; Type: CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.class
    ADD CONSTRAINT class_pkey PRIMARY KEY (id);


--
-- Name: media media_pkey; Type: CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.media
    ADD CONSTRAINT media_pkey PRIMARY KEY (uuid);


--
-- Name: question question_pkey; Type: CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.question
    ADD CONSTRAINT question_pkey PRIMARY KEY (qwiz_id, index);


--
-- Name: qwiz qwiz_pkey; Type: CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.qwiz
    ADD CONSTRAINT qwiz_pkey PRIMARY KEY (id);


--
-- Name: student student_pkey; Type: CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.student
    ADD CONSTRAINT student_pkey PRIMARY KEY (student_id, class_id);


--
-- Name: vote vote_pkey; Type: CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.vote
    ADD CONSTRAINT vote_pkey PRIMARY KEY (voter_id, qwiz_id);


--
-- Name: completed_assignment check_student; Type: TRIGGER; Schema: public; Owner: qwiz
--

CREATE TRIGGER check_student BEFORE INSERT OR UPDATE ON public.completed_assignment FOR EACH ROW EXECUTE FUNCTION public.check_student_class_func();


--
-- Name: student check_student; Type: TRIGGER; Schema: public; Owner: qwiz
--

CREATE TRIGGER check_student BEFORE INSERT OR UPDATE ON public.student FOR EACH ROW EXECUTE FUNCTION public.check_student_func();


--
-- Name: class check_teacher; Type: TRIGGER; Schema: public; Owner: qwiz
--

CREATE TRIGGER check_teacher BEFORE INSERT OR UPDATE ON public.class FOR EACH ROW EXECUTE FUNCTION public.check_teacher_func();


--
-- Name: question delete_embed; Type: TRIGGER; Schema: public; Owner: qwiz
--

CREATE TRIGGER delete_embed AFTER DELETE ON public.question FOR EACH ROW EXECUTE FUNCTION public.delete_embed_func();


--
-- Name: account delete_profile_picture; Type: TRIGGER; Schema: public; Owner: qwiz
--

CREATE TRIGGER delete_profile_picture AFTER DELETE ON public.account FOR EACH ROW EXECUTE FUNCTION public.delete_profile_picture_func();


--
-- Name: qwiz delete_thumbnail; Type: TRIGGER; Schema: public; Owner: qwiz
--

CREATE TRIGGER delete_thumbnail AFTER DELETE ON public.qwiz FOR EACH ROW EXECUTE FUNCTION public.delete_thumbnail_func();


--
-- Name: account update_account_type; Type: TRIGGER; Schema: public; Owner: qwiz
--

CREATE TRIGGER update_account_type AFTER UPDATE ON public.account FOR EACH ROW EXECUTE FUNCTION public.update_account_type_func();


--
-- Name: account account_profile_picture_uuid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.account
    ADD CONSTRAINT account_profile_picture_uuid_fkey FOREIGN KEY (profile_picture_uuid) REFERENCES public.media(uuid) ON DELETE SET NULL;


--
-- Name: assignment assignment_class_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.assignment
    ADD CONSTRAINT assignment_class_id_fkey FOREIGN KEY (class_id) REFERENCES public.class(id) ON DELETE CASCADE;


--
-- Name: assignment assignment_qwiz_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.assignment
    ADD CONSTRAINT assignment_qwiz_id_fkey FOREIGN KEY (qwiz_id) REFERENCES public.qwiz(id) ON DELETE CASCADE;


--
-- Name: class class_teacher_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.class
    ADD CONSTRAINT class_teacher_id_fkey FOREIGN KEY (teacher_id) REFERENCES public.account(id) ON DELETE CASCADE;


--
-- Name: completed_assignment completed_assignment_assignment_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.completed_assignment
    ADD CONSTRAINT completed_assignment_assignment_id_fkey FOREIGN KEY (assignment_id) REFERENCES public.assignment(id) ON DELETE CASCADE;


--
-- Name: completed_assignment completed_assignment_student_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.completed_assignment
    ADD CONSTRAINT completed_assignment_student_id_fkey FOREIGN KEY (student_id) REFERENCES public.account(id) ON DELETE CASCADE;


--
-- Name: question question_embed_uuid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.question
    ADD CONSTRAINT question_embed_uuid_fkey FOREIGN KEY (embed_uuid) REFERENCES public.media(uuid) ON DELETE SET NULL;


--
-- Name: question question_qwiz_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.question
    ADD CONSTRAINT question_qwiz_id_fkey FOREIGN KEY (qwiz_id) REFERENCES public.qwiz(id) ON DELETE CASCADE;


--
-- Name: qwiz qwiz_creator_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.qwiz
    ADD CONSTRAINT qwiz_creator_id_fkey FOREIGN KEY (creator_id) REFERENCES public.account(id) ON DELETE CASCADE;


--
-- Name: qwiz qwiz_thumbnail_uuid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.qwiz
    ADD CONSTRAINT qwiz_thumbnail_uuid_fkey FOREIGN KEY (thumbnail_uuid) REFERENCES public.media(uuid) ON DELETE SET NULL;


--
-- Name: student student_class_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.student
    ADD CONSTRAINT student_class_id_fkey FOREIGN KEY (class_id) REFERENCES public.class(id) ON DELETE CASCADE;


--
-- Name: student student_student_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.student
    ADD CONSTRAINT student_student_id_fkey FOREIGN KEY (student_id) REFERENCES public.account(id) ON DELETE CASCADE;


--
-- Name: vote vote_qwiz_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.vote
    ADD CONSTRAINT vote_qwiz_id_fkey FOREIGN KEY (qwiz_id) REFERENCES public.qwiz(id) ON DELETE CASCADE;


--
-- Name: vote vote_voter_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: qwiz
--

ALTER TABLE ONLY public.vote
    ADD CONSTRAINT vote_voter_id_fkey FOREIGN KEY (voter_id) REFERENCES public.account(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

