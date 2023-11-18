-- init-db.sql
CREATE USER IF NOT EXISTS 'httpbin-user' @'%' IDENTIFIED BY 'aabb';

GRANT ALL PRIVILEGES ON `httpbin-db`.* TO 'httpbin-user' @'%';

FLUSH PRIVILEGES;

CREATE DATABASE IF NOT EXISTS `httpbin-db`;

USE `httpbin-db`;

CREATE TABLE requests (
    id INT AUTO_INCREMENT PRIMARY KEY,
    value INT NOT NULL,
    response_body JSON,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
