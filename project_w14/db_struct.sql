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
-- Name: customer_table; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer_table (
    c_id integer NOT NULL,
    c_name text NOT NULL,
    c_email text NOT NULL,
    c_mobile character varying(50),
    eid integer NOT NULL,
    data_id integer NOT NULL,
    c_age integer
);


ALTER TABLE public.customer_table OWNER TO postgres;

--
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size character varying(8) NOT NULL,
    data_duration_days integer NOT NULL,
    data_price_naira integer NOT NULL
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Name: departments; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.departments (
    dept_managerid integer NOT NULL,
    dno integer NOT NULL,
    dname text NOT NULL,
    dlocation text,
    pno integer
);


ALTER TABLE public.departments OWNER TO postgres;

--
-- Name: projects; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.projects (
    pno integer NOT NULL,
    pname text NOT NULL,
    pduration text NOT NULL,
    project_mangerid integer
);


ALTER TABLE public.projects OWNER TO postgres;

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
-- Data for Name: customer_table; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer_table (c_id, c_name, c_email, c_mobile, eid, data_id, c_age) FROM stdin;
110	Musta Karim	m_karim@gmail.com	08055089112	102	5	35
111	Lilian Jaiya	l_jaiya@gmail.com	08055185341	100	3	43
112	Arthur Musa	a_musa@gmail.com	07055288713	107	10	50
113	Philip Akonjo	p_akonjo@gmail.com	09052356772	100	2	41
114	Marylene Mapa	m_mapa@gmail.com	08053333551	120	5	33
115	Oghenero Agor	o_ogor@gmail.com	07055566774	117	11	50
116	Adams Bree	a_bree@gmail.com	08056765424	102	1	33
117	Okafor Mathias	o_mathias@gmail.com	08056763367	120	10	45
118	Samson Adeleke	s_adeleke@gmail.com	07056774423	117	11	65
119	Lawal Tamire	l_tamire@gmail.com	09053111101	107	5	35
120	James Job	j_job@gmail.com	08059693919	100	8	44
121	Matthew Jakande	m_jakande@gmail.com	07051232144	120	2	21
122	Jamila Adegboye	j_adegboye@gmail.com	08054921923	107	5	20
\.


--
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (data_id, data_size, data_duration_days, data_price_naira) FROM stdin;
1	350MB	2	200
2	1.8GB	14	500
3	3.9GB	30	1000
4	7.5GB	30	1500
5	9.2GB	30	2000
6	10.8GB	30	2500
7	14GB	30	3000
8	18GB	30	4000
9	24GB	30	5000
10	29.9GB	30	8000
11	50GB	30	10000
\.


--
-- Data for Name: departments; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.departments (dept_managerid, dno, dname, dlocation, pno) FROM stdin;
\.


--
-- Data for Name: projects; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.projects (pno, pname, pduration, project_mangerid) FROM stdin;
\.


--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (eid, ename, dno, esal, age, mobile) FROM stdin;
107	Alokwe Martin	7	380000	48	70900828
97	Dankade Aminat	5	550000	40	90233395
101	Alade Joy	2	250000	33	90485733
\.


--
-- Name: customer_table customer_table_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer_table
    ADD CONSTRAINT customer_table_pkey PRIMARY KEY (c_id);


--
-- Name: dataplan dataplan_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_pkey PRIMARY KEY (data_id);


--
-- Name: projects projects_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_pkey PRIMARY KEY (pname);


--
-- PostgreSQL database dump complete
--

