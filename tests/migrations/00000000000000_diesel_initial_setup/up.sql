CREATE TABLE users
(
    id INT IDENTITY(1,1),
    name NVARCHAR(MAX) NOT NULL,
    hair_color NVARCHAR(MAX) NULL,
    CONSTRAINT PK_users PRIMARY KEY CLUSTERED (id)
);

CREATE TABLE posts
(
    id INT IDENTITY(1,1),
    user_id INT NOT NULL,
    title VARCHAR(200) NOT NULL,
    body NVARCHAR(MAX) NULL,
    CONSTRAINT PK_posts PRIMARY KEY CLUSTERED (id)
)


CREATE TABLE comments
(
    id INT IDENTITY(1,1),
    post_id INT NOT NULL,
    [text] NVARCHAR(MAX) NOT NULL,
    CONSTRAINT PK_comments PRIMARY KEY CLUSTERED (id)
)

CREATE TABLE special_posts
(
    id INT IDENTITY(1,1),
    user_id INT NOT NULL,
    title NVARCHAR(MAX) NOT NULL,
    CONSTRAINT PK_special_posts PRIMARY KEY CLUSTERED (id)
)

CREATE TABLE special_comments
(
    id INT IDENTITY(1,1),
    special_post_id INT NOT NULL,
    CONSTRAINT PK_special_comments PRIMARY KEY CLUSTERED (id)
)

CREATE TABLE followings (
    user_id INT NOT NULL,
    post_id INT NOT NULL,
    email_notifications BIT NOT NULL DEFAULT 0,
    CONSTRAINT PK_followings PRIMARY KEY (user_id, post_id)
)

CREATE TABLE likes (
    comment_id INT NOT NULL,
    user_id INT NOT NULL,
    CONSTRAINT PK_likes PRIMARY KEY (user_id, comment_id)
)

CREATE TABLE trees (
    id INT PRIMARY KEY,
    parent_id INT
)