
-- Creating the roles table
CREATE TABLE IF NOT EXISTS roles (
    role_id INT AUTO_INCREMENT PRIMARY KEY,
    role VARCHAR(20) NOT NULL, -- (e.g., engineer)
    create_articles BOOLEAN NOT NULL,
    delete_articles BOOLEAN NOT NULL,
    read_articles BOOLEAN NOT NULL,
    create_quotations BOOLEAN NOT NULL,
    delete_quotations BOOLEAN NOT NULL,
    read_quotations BOOLEAN NOT NULL,
    can_edit_roles BOOLEAN NOT NULL,
    can_edit_users BOOLEAN NOT NULL
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
