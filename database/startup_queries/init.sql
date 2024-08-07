-- init.sql

USE test;

-- Creating the roles table
CREATE TABLE IF NOT EXISTS roles (
    roles_id INT AUTO_INCREMENT PRIMARY KEY,
    role VARCHAR(20) NOT NULL, -- (e.g., engineer)
    createQuotations BOOLEAN NOT NULL,
    createArticles BOOLEAN NOT NULL,
    changeArticles BOOLEAN NOT NULL
);

-- Creating the companys table
CREATE TABLE IF NOT EXISTS companys (
    companys_id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(20),
    streetAndNumber VARCHAR(30),
    postalcode VARCHAR(10),
    city VARCHAR(20),
    country VARCHAR(20),
    phone VARCHAR(20) -- force format +XX X XXXX XXXX +31 6 1234 5678
);

-- Inserting testdata in the tables
source /docker-entrypoint-initdb.d/testdata.sql;
