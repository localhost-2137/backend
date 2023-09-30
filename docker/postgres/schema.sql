CREATE TABLE universities (
    id int primary key,
    rank int NOT NULL,
    name varchar(200) NOT NULL,
    academic BOOLEAN NOT NULL,
    url text NOT NULL,
    lng float NOT NULL,
    lat float NOT NULL,
    address varchar(100) NOT NULL,
    city varchar(50) NOT NULL,
    number_students int NOT NULL
);

CREATE TABLE universities_subjects (
    u_id int references universities(id),
    subject varchar(50) not null
);

CREATE EXTENSION IF NOT EXISTS postgis;
