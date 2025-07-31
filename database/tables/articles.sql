
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
