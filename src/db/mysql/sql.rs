pub(crate) const CREATE_TABLE: &str = "
CREATE TABLE IF NOT EXISTS users (
    id INT PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR (254) UNIQUE NOT NULL,
	password VARCHAR ( 255 ) NOT NULL,
    is_admin BOOLEAN DEFAULT FALSE
);
";

pub(crate) const INSERT_USER: &str = "
INSERT INTO users (username, password, is_admin) VALUES (?, ?, ?);
";

pub(crate) const UPDATE_USER: &str = "
UPDATE users SET 
    username = ?,
    password = ?,
    is_admin = ?
WHERE
    id = ?
";

pub(crate) const SELECT_BY_ID: &str = "
SELECT * FROM users WHERE id = ?;
";

pub(crate) const SELECT_BY_EMAIL: &str = "
SELECT * FROM users WHERE username = ?;
";

pub(crate) const REMOVE_BY_ID: &str = "
DELETE FROM users WHERE id = ?;
";
pub(crate) const REMOVE_BY_EMAIL: &str = "
DELETE FROM users WHERE username = ?;
";
