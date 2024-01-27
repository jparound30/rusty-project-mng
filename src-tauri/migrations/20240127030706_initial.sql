-- Add migration script here
CREATE TABLE users
(
    user_id       INTEGER PRIMARY KEY AUTOINCREMENT,
    username      TEXT NOT NULL,
    password_hash TEXT,
    salt          TEXT
);

-- 初期ユーザ
-- ユーザ名:user, パスワード:pass
INSERT INTO users
    (user_id, username, password_hash, salt)
VALUES (1, 'user', '$argon2id$v=19$m=19456,t=2,p=1$qzr9nytOFNS+eiCTmRRPdA$oXjqSPTdaPirXQxz5lxAp+pp54IAIc9su2aepvqSx5E',
        'qzr9nytOFNS+eiCTmRRPdA')
     , (2, 'やまだ', '', '')
     , (3, 'すずき', '', '')
     , (4, 'はやし', '', '')
     , (5, 'もり', '', '')
;

CREATE TABLE task_status
(
    task_status_id INTEGER PRIMARY KEY,
    title          TEXT    NOT NULL,
    view_order     INTEGER NOT NULL UNIQUE
);

-- タスクステータス初期値
INSERT INTO task_status
    (task_status_id, title, view_order)
VALUES (1, '未着手', 1000)
     , (2, '対応中', 2000)
     , (3, 'レビュー中', 3000)
     , (4, 'レビュー対応中', 4000)
     , (5, '対応済', 5000)
     , (6, '完了', 6000)
;

CREATE TABLE tasks
(
    task_id          INTEGER PRIMARY KEY,
    title            TEXT    NOT NULL,
    description      TEXT,
    assignee_user_id INTEGER,
    parent_task_id   INTEGER,
    start_date       TEXT,
    due_date         TEXT,
    task_status_id   INTEGER NOT NULL,
    FOREIGN KEY (assignee_user_id) REFERENCES users (user_id),
    FOREIGN KEY (parent_task_id) REFERENCES tasks (task_id),
    FOREIGN KEY (task_status_id) REFERENCES task_status (task_status_id)
);

