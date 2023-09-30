CREATE TABLE universities (
    id serial,
    rank int,
    name varchar(200) NOT NULL,
    url text NOT NULL,
    lng float NOT NULL,
    lat float NOT NULL,
    address varchar(100) NOT NULL,
    number_students int NOT NULL,
    subjects text NOT NULL
);
