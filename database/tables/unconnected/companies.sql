
CREATE TABLE IF NOT EXISTS companys (
    company_id INT AUTO_INCREMENT PRIMARY KEY,
    company_name varchar(255) NOT null,
    company_street varchar(255) NOT null,
    company_city varchar(255) NOT null,
    company_zipcode varchar(10) NOT null,
    company_country varchar(100) NOT null,
    company_phone varchar(50) NOT null,
    company_email varchar(255) NOT null,
    company_website varchar(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS company_contacts (
    contact_id INT AUTO_INCREMENT PRIMARY KEY,
    company INT NOT NULL,
    firstname varchar(20) NOT null,
    lastname varchar(20),
    contact_email varchar(255) NOT null,
    contact_phone varchar(50) NOT null,
    contact_mobile varchar(50),
    contact_titel varchar(50),

    CONSTRAINT FK_company_contacts_company FOREIGN KEY (company) REFERENCES companys(company_id)
);

CREATE TABLE IF NOT EXISTS user_roles (
    role_id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    role_name VARCHAR(50) NOT NULL UNIQUE,
    create_orders BOOLEAN NOT NULL DEFAULT FALSE,
    create_quotations BOOLEAN NOT NULL DEFAULT FALSE,
    create_purchase_orders BOOLEAN NOT NULL DEFAULT FALSE,
    create_articles BOOLEAN NOT NULL DEFAULT FALSE,
    create_customers BOOLEAN NOT NULL DEFAULT FALSE,
    create_suppliers BOOLEAN NOT NULL DEFAULT FALSE,
    create_users BOOLEAN NOT NULL DEFAULT FALSE,
    create_companys BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS user_status_types (
    status_id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    status_name VARCHAR(32) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS users (
    user_id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,             -- for now not secured
    email VARCHAR(64) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP NULL DEFAULT NULL,
    role_id INT NOT NULL,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    status int NOT NULL,

    CONSTRAINT FK_users_role FOREIGN KEY (role_id) REFERENCES user_roles(role_id)
    CONSTRAINT FK_users_status FOREIGN KEY (status) REFERENCES user_status_types(status_id)
);

INSERT INTO companys (
    company_name, company_street, company_city, company_zipcode, company_country,
    company_phone, company_email, company_website
) VALUES
('TechNed B.V.', 'Keizersgracht 123', 'Amsterdam', '1015CJ', 'Netherlands', '+31 20 123 4567', 'info@techned.nl', 'https://www.techned.nl'),
('GlobalSoft Ltd.', '221B Baker Street', 'London', 'NW1 6XE', 'United Kingdom', '+44 20 7946 0991', 'contact@globalsoft.co.uk', 'https://www.globalsoft.co.uk'),
('DigiWare GmbH', 'Musterstraße 45', 'Berlin', '10115', 'Germany', '+49 30 1234567', 'support@digiware.de', 'https://www.digiware.de'),
('InnoTech NV', 'Stationsstraat 89', 'Utrecht', '3511CE', 'Netherlands', '+31 30 456 7890', 'hello@innotech.nl', 'https://www.innotech.nl'),
('BlueSky Inc.', '500 Market St', 'San Francisco', '94105', 'United States', '+1 415 123 7890', 'info@bluesky.com', 'https://www.bluesky.com'),
('CloudOne S.A.', 'Rue de la Loi 155', 'Brussels', '1040', 'Belgium', '+32 2 289 1234', 'contact@cloudone.be', 'https://www.cloudone.be'),
('EcoLogix B.V.', 'Nieuwezijds Voorburgwal 182', 'Amsterdam', '1012SJ', 'Netherlands', '+31 20 567 1234', 'info@ecologix.nl', 'https://www.ecologix.nl'),
('NextWave S.L.', 'Calle de Alcalá 45', 'Madrid', '28014', 'Spain', '+34 91 123 4567', 'hello@nextwave.es', 'https://www.nextwave.es'),
('Nordic Solutions AB', 'Kungsgatan 12', 'Stockholm', '111 35', 'Sweden', '+46 8 123 456', 'info@nordicsolutions.se', 'https://www.nordicsolutions.se'),
('OrangeBits SARL', '5 Avenue Victor Hugo', 'Paris', '75016', 'France', '+33 1 44 55 66 77', 'support@orangebits.fr', 'https://www.orangebits.fr'),
('DeltaForce B.V.', 'Oudezijds Achterburgwal 12', 'Amsterdam', '1012DL', 'Netherlands', '+31 20 998 4455', 'sales@deltaforce.nl', 'https://www.deltaforce.nl'),
('InfoSphere Ltd.', '41 George Street', 'Edinburgh', 'EH2 2HT', 'United Kingdom', '+44 131 667 8900', 'info@infosphere.uk', 'https://www.infosphere.uk'),
('ZenTech Oy', 'Mannerheimintie 20', 'Helsinki', '00100', 'Finland', '+358 9 1234567', 'hello@zentech.fi', 'https://www.zentech.fi'),
('VisionWare B.V.', 'Grote Markt 1', 'Groningen', '9712HR', 'Netherlands', '+31 50 123 4567', 'info@visionware.nl', 'https://www.visionware.nl'),
('AlphaTech Pte Ltd', 'Orchard Road 100', 'Singapore', '238840', 'Singapore', '+65 1234 5678', 'contact@alphatech.sg', 'https://www.alphatech.sg'),
('CyberEdge BV', 'Vughterstraat 75', 'Den Bosch', '5211EK', 'Netherlands', '+31 73 512 1212', 'info@cyberedge.nl', 'https://www.cyberedge.nl'),
('TransGlobal Corp.', '200 King St W', 'Toronto', 'M5H 3T4', 'Canada', '+1 416 123 9999', 'contact@transglobal.ca', 'https://www.transglobal.ca'),
('ByteForge B.V.', 'Laan van NOI 22', 'The Hague', '2595AJ', 'Netherlands', '+31 70 123 4567', 'hello@byteforge.nl', 'https://www.byteforge.nl'),
('CodeLab LLC', '1st Ave 150', 'New York', '10009', 'United States', '+1 212 345 6789', 'support@codelab.com', 'https://www.codelab.com'),
('VeloSoft B.V.', 'Pleinweg 230', 'Rotterdam', '3083EX', 'Netherlands', '+31 10 987 6543', 'info@velosoft.nl', 'https://www.velosoft.nl');

INSERT INTO company_contacts (
    company, firstname, lastname, contact_email, contact_phone,
    contact_mobile, contact_titel
) VALUES
(1, 'Jan', 'de Vries', 'jan.vries@techned.nl', '+31 20 123 4567', '+31 6 1234 5678', 'CTO'),
(2, 'Alice', 'Smith', 'alice.smith@globalsoft.co.uk', '+44 20 7946 0991', '+44 7400 123456', 'Project Manager'),
(3, 'Lars', 'Müller', 'lars.mueller@digiware.de', '+49 30 1234567', '+49 170 1234567', 'Product Owner'),
(4, 'Sanne', 'Jansen', 's.jansen@innotech.nl', '+31 30 456 7890', NULL, 'CEO'),
(5, 'Michael', 'Johnson', 'm.johnson@bluesky.com', '+1 415 123 7890', '+1 650 555 7890', 'Sales Manager'),
(6, 'Elise', 'Dubois', 'elise.dubois@cloudone.be', '+32 2 289 1234', '+32 485 123456', 'Support Lead'),
(7, 'Peter', 'van Dijk', 'p.vandijk@ecologix.nl', '+31 20 567 1234', NULL, 'Engineer'),
(8, 'Carlos', 'García', 'carlos.garcia@nextwave.es', '+34 91 123 4567', '+34 600 123 456', 'Consultant'),
(9, 'Anna', 'Eriksson', 'anna.eriksson@nordicsolutions.se', '+46 8 123 456', NULL, 'HR Manager'),
(10, 'Luc', 'Moreau', 'luc.moreau@orangebits.fr', '+33 1 44 55 66 77', '+33 6 12 34 56 78', 'Marketing Director'),
(11, 'Jeroen', 'Koster', 'j.koster@deltaforce.nl', '+31 20 998 4455', NULL, 'CFO'),
(12, 'Rachel', 'Murray', 'r.murray@infosphere.uk', '+44 131 667 8900', '+44 7800 999888', 'Analyst'),
(13, 'Timo', 'Virtanen', 'timo.virtanen@zentech.fi', '+358 9 1234567', '+358 40 1234567', 'Engineer'),
(14, 'Lisa', 'van Leeuwen', 'lisa.leeuwen@visionware.nl', '+31 50 123 4567', NULL, 'Designer'),
(15, 'Aaron', 'Lim', 'aaron.lim@alphatech.sg', '+65 1234 5678', '+65 8123 4567', 'DevOps Engineer'),
(16, 'Wim', 'de Groot', 'wim.groot@cyberedge.nl', '+31 73 512 1212', '+31 6 9988 7766', 'Security Lead'),
(17, 'Natalie', 'Chen', 'n.chen@transglobal.ca', '+1 416 123 9999', '+1 647 555 1122', 'Operations Lead'),
(18, 'Mark', 'Boersma', 'm.boersma@byteforge.nl', '+31 70 123 4567', NULL, 'Technical Consultant'),
(19, 'Sophie', 'Adams', 'sophie.adams@codelab.com', '+1 212 345 6789', '+1 917 777 9999', 'Team Lead'),
(20, 'Tessa', 'Hendriks', 't.hendriks@velosoft.nl', '+31 10 987 6543', '+31 6 4321 1234', 'Scrum Master');

INSERT INTO user_roles (
    role_name,create_orders,create_quotations,create_purchase_orders,create_articles,
    create_customers,create_suppliers,create_users,create_companys
) VALUES
('Administrator', TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE),
('Sales Manager', TRUE, TRUE, FALSE, FALSE, TRUE, FALSE, FALSE, FALSE),
('Procurement Officer', FALSE, FALSE, TRUE, FALSE, FALSE, TRUE, FALSE, FALSE),
('Product Manager', FALSE, FALSE, FALSE, TRUE, FALSE, FALSE, FALSE, FALSE),
('Customer Support', FALSE, TRUE, FALSE, FALSE, TRUE, FALSE, FALSE, FALSE),
('Finance Admin', TRUE, FALSE, TRUE, FALSE, FALSE, FALSE, FALSE, FALSE),
('Warehouse Staff', FALSE, FALSE, FALSE, TRUE, FALSE, FALSE, FALSE, FALSE),
('HR Manager', FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, TRUE, FALSE),
('IT Admin', FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, TRUE, TRUE),
('Sales Representative', TRUE, TRUE, FALSE, FALSE, TRUE, FALSE, FALSE, FALSE),
('Junior Sales', TRUE, TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE),
('Purchasing Assistant', FALSE, FALSE, TRUE, FALSE, FALSE, FALSE, FALSE, FALSE),
('Marketing Manager', FALSE, TRUE, FALSE, TRUE, FALSE, FALSE, FALSE, FALSE),
('External Auditor', FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE),
('Logistics Coordinator', TRUE, FALSE, TRUE, TRUE, FALSE, FALSE, FALSE, FALSE),
('Customer Admin', FALSE, FALSE, FALSE, FALSE, TRUE, FALSE, FALSE, FALSE),
('Supplier Manager', FALSE, FALSE, FALSE, FALSE, FALSE, TRUE, FALSE, FALSE),
('Business Analyst', FALSE, TRUE, FALSE, TRUE, TRUE, TRUE, FALSE, FALSE),
('Regional Manager', TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, FALSE, FALSE),
('Company Admin', FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, TRUE, TRUE);

INSERT INTO user_status_types (status_name) VALUES
('Active'),
('Inactive'),
('Suspended'),
('Pending Verification'),
('Archived');

INSERT INTO users (
    username, password, email, role_id,
    first_name, last_name, user_status, last_login
) VALUES
('jvandermeer', 'password123', 'jan.vandermeer@example.com', 1, 'Jan', 'van der Meer', 1, '2025-07-20 08:15:00'),
('asmith', 'password123', 'alice.smith@globalsoft.co.uk', 2, 'Alice', 'Smith', 1, '2025-07-21 10:00:00'),
('lmueller', 'password123', 'lars.mueller@digiware.de', 3, 'Lars', 'Müller', 2, NULL),
('sjansen', 'password123', 'sanne.jansen@innotech.nl', 4, 'Sanne', 'Jansen', 1, '2025-07-22 09:45:00'),
('mjohnson', 'password123', 'michael.johnson@bluesky.com', 5, 'Michael', 'Johnson', 1, '2025-07-21 15:30:00'),
('edubois', 'password123', 'elise.dubois@cloudone.be', 6, 'Elise', 'Dubois', 3, NULL),
('pvandijk', 'password123', 'peter.vandijk@ecologix.nl', 7, 'Peter', 'van Dijk', 1, '2025-07-22 07:50:00'),
('cgarcia', 'password123', 'carlos.garcia@nextwave.es', 8, 'Carlos', 'García', 1, NULL),
('aeriksson', 'password123', 'anna.eriksson@nordicsolutions.se', 9, 'Anna', 'Eriksson', 2, NULL),
('lmoreau', 'password123', 'luc.moreau@orangebits.fr', 10, 'Luc', 'Moreau', 1, '2025-07-19 16:10:00'),
('jkoster', 'password123', 'jeroen.koster@deltaforce.nl', 11, 'Jeroen', 'Koster', 4, NULL),
('rmurray', 'password123', 'rachel.murray@infosphere.uk', 12, 'Rachel', 'Murray', 1, '2025-07-20 14:05:00'),
('tvirtanen', 'password123', 'timo.virtanen@zentech.fi', 13, 'Timo', 'Virtanen', 1, '2025-07-21 11:20:00'),
('lvleeuwen', 'password123', 'lisa.vleeuwen@visionware.nl', 14, 'Lisa', 'van Leeuwen', 5, NULL),
('alim', 'password123', 'aaron.lim@alphatech.sg', 15, 'Aaron', 'Lim', 1, '2025-07-22 08:30:00'),
('wdegroot', 'password123', 'wim.degroot@cyberedge.nl', 16, 'Wim', 'de Groot', 1, '2025-07-21 13:55:00'),
('nchen', 'password123', 'natalie.chen@transglobal.ca', 17, 'Natalie', 'Chen', 1, '2025-07-22 09:00:00'),
('mboersma', 'password123', 'mark.boersma@byteforge.nl', 18, 'Mark', 'Boersma', 2, NULL),
('sadams', 'password123', 'sophie.adams@codelab.com', 19, 'Sophie', 'Adams', 1, '2025-07-20 17:10:00'),
('thendriks', 'password123', 'tessa.hendriks@velosoft.nl', 20, 'Tessa', 'Hendriks', 1, '2025-07-21 08:05:00');
