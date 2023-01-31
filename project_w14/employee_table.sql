--
-- PostgreSQL database dump
--

-- Dumped from database version 15.1
-- Dumped by pg_dump version 15.1

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

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    eid integer NOT NULL,
    ename text NOT NULL,
    dno integer NOT NULL,
    esal real NOT NULL,
    age integer NOT NULL,
    mobile integer NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (eid, ename, dno, esal, age, mobile) FROM stdin;
107	Alokwe Martin	7	380000	48	70900828
97	Dankade Aminat	5	550000	40	90233395
101	Alade Joy	2	250000	33	90485733
\.


--
-- PostgreSQL database dump complete
--

