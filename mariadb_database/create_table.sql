CREATE TABLE requests (
    id INT AUTO_INCREMENT PRIMARY KEY,
    generated_value INT NOT NULL,
    response_body JSON,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
