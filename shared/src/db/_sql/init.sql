PRAGMA foreign_keys = ON;

CREATE TABLE backoffice_users
(
    id       INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    username TEXT UNIQUE                       NOT NULL,
    password BLOB                              NOT NULL,
    role     TEXT                              NOT NULL
);

CREATE TABLE user_login_tokens
(
    user_id      INTEGER     NOT NULL,
    token        TEXT UNIQUE NOT NULL,
    expire_after TEXT        NOT NULL,
    FOREIGN KEY (user_id) REFERENCES backoffice_users (id) ON DELETE CASCADE
);

CREATE TABLE url_redirect
(
    id                 INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    url_path           TEXT UNIQUE                       NOT NULL,
    url_redirect       TEXT UNIQUE                       NOT NULL,
    created_at         TEXT                              NOT NULL,
    created_by_user_id INTEGER                           NOT NULL,
    FOREIGN KEY (created_by_user_id) REFERENCES backoffice_users (id) ON DELETE CASCADE
);