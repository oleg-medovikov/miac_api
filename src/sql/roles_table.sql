CREATE TABLE IF NOT EXISTS roles (
    user_guid VARCHAR(25) REFERENCES users(guid),
    name VARCHAR(30),
    token VARCHAR(25)
);
