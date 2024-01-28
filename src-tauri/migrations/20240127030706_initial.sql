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
    view_order     INTEGER NOT NULL UNIQUE,
    progress_rate  INTEGER
);

-- タスクステータス初期値
INSERT INTO task_status
    (task_status_id, title, view_order, progress_rate)
VALUES (1, '未着手', 1000, 0)
     , (2, '対応中', 2000, 10)
     , (3, 'レビュー中', 3000, 80)
     , (4, 'レビュー対応中', 4000, 90)
     , (5, '対応済', 5000, 100)
     , (6, '完了', 6000, 100)
;

CREATE TABLE tasks
(
    task_id              INTEGER PRIMARY KEY,
    title                TEXT    NOT NULL,
    description          TEXT,
    assignee_resource_id INTEGER,
    parent_task_id       INTEGER,
    start_date           TEXT,
    due_date             TEXT,
    estimated_time       INTEGER,
    actual_time          INTEGER,
    planed_value         INTEGER,
    task_status_id       INTEGER NOT NULL,
    progress_rate        INTEGER NOT NULL,
    FOREIGN KEY (assignee_resource_id) REFERENCES resources (resource_id),
    FOREIGN KEY (parent_task_id) REFERENCES tasks (task_id),
    FOREIGN KEY (task_status_id) REFERENCES task_status (task_status_id)
);

CREATE TABLE resources
(
    resource_id    INTEGER PRIMARY KEY,
    name           TEXT    NOT NULL,
    cost_per_month INTEGER NOT NULL
)
