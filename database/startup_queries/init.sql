-- init.sql

USE test;

-- Creating the roles table
CREATE TABLE IF NOT EXISTS roles (
    role_id INT AUTO_INCREMENT PRIMARY KEY,
    role VARCHAR(20) NOT NULL, -- (e.g., engineer)
    create_articles BOOLEAN NOT NULL,
    delete_articles BOOLEAN NOT NULL,
    read_articles BOOLEAN NOT NULL,
    create_ quotations BOOLEAN NOT NULL,
    delete_quotations BOOLEAN NOT NULL,
    read_quotations BOOLEAN NOT NULL,
    edit_roles BOOLEAN NOT NULL
);

-- Creating the companys table
CREATE TABLE IF NOT EXISTS companys (
    company_id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(20) NOT NULL,
    streetAndNumber VARCHAR(30),
    postalcode VARCHAR(10),
    city VARCHAR(20),
    country VARCHAR(20),
    phone VARCHAR(20) -- force format +XX X XXXX XXXX +31 6 1234 5678
);

-- Creating the users table
CREATE TABLE IF NOT EXISTS users (
    user_id INT AUTO_INCREMENT PRIMARY KEY,
    firstname VARCHAR(20) NOT NULL,
    lastname VARCHAR(20) NOT NULL,
    role_id INT NOT NULL,
    email VARCHAR(100) NOT NULL,
    password VARCHAR(255) NOT NULL,   -- hashed password
    FOREIGN KEY (role_id) REFERENCES roles(role_id)
);

-- Creating the articles table
CREATE TABLE IF NOT EXISTS articles (
    article_id INT AUTO_INCREMENT PRIMARY KEY,
    article_number VARCHAR(100) NOT NULL,
    description VARCHAR(255),
    price DECIMAL(10,2) NOT NULL,
    stock INT NOT NULL,
    -- stock location
    manufacturer_id INT NOT NULL,
    main_supplier_id INT NOT NULL,
    discount DECIMAL(10,2) NOT NULL,            -- 0 if no discount is given

    FOREIGN KEY (manufacturer_id) REFERENCES companys(company_id),
    FOREIGN KEY (main_supplier_id) REFERENCES companys(company_id)
);

-- Inserting testdata in the tables
-- source /docker-entrypoint-initdb.d/testdata.sql;
