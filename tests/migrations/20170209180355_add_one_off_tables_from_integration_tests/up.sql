CREATE TABLE users_with_name_pk(
    name NVARCHAR(200) PRIMARY KEY
)

CREATE TABLE points (
    x INT NOT NULL,
    y INT NOT NULL,
    PRIMARY KEY(x, y)
)