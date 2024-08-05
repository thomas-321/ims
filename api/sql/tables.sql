USE database;

CREATE TABLE roles (
	roles_id INT AUTO_INCREMENT,
	role varchar(20) NOT NULL, -- (exm: engineer)
	createQuotations BOOL NOT NULL,
	createArticles BOOL NOT NULL,
	changeArticles BOOL NOT NULL,

	PRIMARY KEY (roles_id)
);

CREATE TABLE companys (
	companys_id INT AUTO_INCREMENT,
	name varchar(20),
	streetAndNumber varchar(30),
	postalcode varchar(10),
	city varchar(20),
	country varchar(20),
	phone varchar(), -- force format +XX X XXXX XXXX +31 6 1234 5678
);



--=========== TABLES WITH FOREIGN KEYS ===========

CREATE TABLE users (
	users_id INT AUTO_INCEMENT,
	firstname varchar(20),
	lastname varchar(20),
	email varchar(30),
    password_hash varchar(255),
	role INT,


	-- constraints 
	PRIMARY KEY (users_id),
	CONSTRAINT fk_role
	FOREIGN KEY (role) REFERENCES roles(role_id)
);

CREATE TABLE articles (
	article_id INT AUTO_INCREMENT,
	productnumber varchar (25) NOT NULL,
	ordernumber varchar (25) NOT NULL,
	supplier INT NOT NULL,        -- foreign key
	supplierProductNumber varchar(25) NOT NULL,
	mainsupplier INT NOT NULL,    -- foreign key
	orderAmount INT NOT NULL,
	orderUnit            -- foreign key?
	subInternalNumber INT NOT NULL, -- foreign key voor als ordernummer meerder items bevat zoals klemmen.
	inkoopprijs
	korting -- percentage datatype? int 0.00-100.00
	lastPriceChange DATE NOT NULL,
	


	-- constraint
	PRIMARY KEY (article_id),
	UNIQUE (productnumber),
	CONSTRAINT fk_supplier
	FOREIGN KEY (supplier) REFERENCES companys(company_id)
	CONSTRAINT fk_mainsupplier
	FOREIGN KEY (mainsupplier) REFERENCES companys(company_id)
)
