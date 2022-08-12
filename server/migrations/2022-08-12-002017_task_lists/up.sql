-- Your SQL goes here
CREATE TABLE task_lists (
    tid SERIAL NOT NULL PRIMARY KEY,
    uid SERIAL NOT NULL,
    task_list JSONB NOT NULL,
    FOREIGN KEY (uid) REFERENCES users(uid)
);
