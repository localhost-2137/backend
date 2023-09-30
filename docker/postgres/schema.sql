CREATE TABLE universities (
    id serial,
    rank int NOT NULL,
    name varchar(200) NOT NULL,
    academic BOOLEAN NOT NULL,
    url text NOT NULL,
    lng float NOT NULL,
    lat float NOT NULL,
    address varchar(100) NOT NULL,
    city varchar(50) NOT NULL,
    number_students int NOT NULL,
    subjects text NOT NULL
);

CREATE EXTENSION IF NOT EXISTS postgis;
