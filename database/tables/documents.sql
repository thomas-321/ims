
CREATE TABLE IF NOT EXISTS order_status_types (
    status_id INT AUTO_INCREMENT PRIMARY KEY,
    status_name VARCHAR(50) NOT NULL
);

CREATE TABLE IF NOT EXISTS document_types (
    document_type_id INT AUTO_INCREMENT PRIMARY KEY,
    document_type_name VARCHAR(50) NOT NULL
    document_prefix VARCHAR(2) NOT NULL
);


create table if not exists orders (
    order_id int auto_increment primary key
    ordernumber varchar(50) NOT null,
    customer_id int NOT null,
    customer_refrence varchar(100) NOT NULL,
    order_creation_date datetime NOT NULL,
    order_text TEXT NOT NULL,
    order_status int NOT NULL,
    order_lead int NOT NULL, -- person who is responsible for the order

    CONSTRAINT FK_orders_customer FOREIGN KEY (customer_id) REFERENCES companys(company_id),
    CONSTRAINT FK_orders_status FOREIGN KEY (order_status) REFERENCES order_status_types(status_id)
    CONSTRAINT FK_orders_lead FOREIGN KEY (order_lead) REFERENCES users(user_id)
);



CREATE TABLE IF NOT EXISTS quotations (
    quotation_id INT AUTO_INCREMENT PRIMARY KEY,
    quotation_number VARCHAR(50) NOT NULL,
    customer_id INT NOT NULL,
    customer_reference VARCHAR(100) NOT NULL,
    quotation_creation_date DATETIME NOT NULL,
    quotation_text TEXT NOT NULL,

    CONSTRAINT FK_company_id_quotation FOREIGN KEY (customer_id) REFERENCES companys(company_id)
);


CREATE TABLE IF NOT EXISTS quotation_items (
    quotation_id INT NOT NULL PRIMARY KEY,
    article_number int NOT NULL,
    article_quantity INT NOT NULL,
    article_price DECIMAL(10, 2) NOT NULL,
    profit_margin DECIMAL(4, 2) NOT NULL,
    total_price DECIMAL(12, 2) NOT NULL,

    CONSTRAINT FK_quotation_items_quotation FOREIGN KEY (quotation_id) REFERENCES quotations_data(quotation_id)
    CONSTRAINT FK_quotation_items_article FOREIGN KEY (article_number) REFERENCES articles(article_id)
);

CREATE TABLE IF NOT EXISTS purchase_orders (
    purchase_order_id INT AUTO_INCREMENT PRIMARY KEY,
    purchase_order_number VARCHAR(25) NOT NULL,

    supplier_id INT NOT NULL,
    supplier_reference VARCHAR(100), -- given by the supplier when the order is placed
    po_creation_date DATETIME NOT NULL,
    po_text TEXT NOT NULL,


    CONSTRAINT FK_purchase_orders_supplier FOREIGN KEY (supplier_id) REFERENCES companys(company_id)
);

-- 
CREATE TABLE IF NOT EXISTS purchase_order_items (
    purchase_order_id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    supplier_article_number INT NOT NULL,
    article_quantity INT NOT NULL,
    article_price DECIMAL(10, 2) NOT NULL,
    total_price DECIMAL(12, 2) NOT NULL,


    CONSTRAINT FK_purchase_order_items_purchase_order FOREIGN KEY (purchase_order_id) REFERENCES purchase_orders(purchase_order_id)
    CONSTRAINT FK_purchase_order_items_article FOREIGN KEY (article_number) REFERENCES articles(article_id)
);

CREATE TABLE IF NOT EXISTS purchase_requests (
    purchase_request_id INT AUTO_INCREMENT PRIMARY KEY,
    request_number VARCHAR(25) NOT NULL,
    requester_id INT NOT NULL, -- the person who requests the purchase
    request_date DATETIME NOT NULL,
    request_text TEXT NOT NULL,

    CONSTRAINT FK_purchase_requests_requester FOREIGN KEY (requester_id) REFERENCES company_contacts(contact_id)
);

insert into order_status_types (status_name) values
('open'),
('in progress'),
('on hold'),
('closed'),
('cancelled');

insert into document_types (document_type_name, document_prefix) values
('invoice', 'IV'),
('quotation', 'QT'),
('purchase order', 'PO'),
('order'), 'OR'),
('purchase request', 'PR');

