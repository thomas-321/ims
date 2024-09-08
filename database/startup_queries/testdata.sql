USE test;

-- Inserting test data into roles table
INSERT INTO test.roles (role, create_articles, delete_articles, read_articles, create_quotations, delete_quotations, read_quotations, can_edit_roles, can_edit_users) VALUES
('Admin', TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE),
('New_User', FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE),
('Editor', TRUE, TRUE, TRUE, FALSE, FALSE, TRUE, FALSE, FALSE),
('Viewer', FALSE, FALSE, TRUE, FALSE, FALSE, TRUE, FALSE, FALSE),
('Sales', FALSE, FALSE, TRUE, TRUE, TRUE, TRUE, FALSE, FALSE),
('Manager', TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, FALSE),
('Guest', FALSE, FALSE, TRUE, FALSE, FALSE, TRUE, FALSE, FALSE),
('Support', FALSE, FALSE, TRUE, FALSE, FALSE, TRUE, FALSE, FALSE),
('Content Creator', TRUE, FALSE, TRUE, FALSE, FALSE, TRUE, FALSE, FALSE),
('Quality Assurance', TRUE, TRUE, TRUE, FALSE, FALSE, TRUE, FALSE, FALSE),
('HR', FALSE, FALSE, TRUE, FALSE, FALSE, TRUE, TRUE, True);


INSERT INTO test.users (firstname, lastname, role_id, email, password) VALUES
('John', 'Doe', 1, 'john.doe@example.com', 'hashed_password_1'),
('Jane', 'Smith', 2, 'jane.smith@example.com', 'hashed_password_2'),
('Alice', 'Johnson', 3, 'alice.johnson@example.com', 'hashed_password_3'),
('Bob', 'Brown', 4, 'bob.brown@example.com', 'hashed_password_4'),
('Carol', 'Davis', 5, 'carol.davis@example.com', 'hashed_password_5'),
('Eve', 'Miller', 6, 'eve.miller@example.com', 'hashed_password_6'),
('Frank', 'Wilson', 7, 'frank.wilson@example.com', 'hashed_password_7'),
('Grace', 'Taylor', 8, 'grace.taylor@example.com', 'hashed_password_8'),
('Hank', 'Anderson', 9, 'hank.anderson@example.com', 'hashed_password_9'),
('Ivy', 'Thomas', 10, 'ivy.thomas@example.com', 'hashed_password_10');

