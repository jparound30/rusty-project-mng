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
VALUES (0, '未着手', 1000, 0)
     , (1, '作業中', 2000, 10)
     , (2, 'レビュー中', 3000, 80)
     , (3, 'レビュー指摘対応中', 4000, 90)
     , (4, '対応済', 5000, 100)
     , (5, '完了', 6000, 100)
;

CREATE TABLE tasks
(
    task_id              INTEGER PRIMARY KEY,
    -- タスク名
    title                TEXT    NOT NULL,
    -- 詳細
    description          TEXT,
    -- 担当者ID
    assignee_resource_id INTEGER,
    -- 親タスクID
    parent_task_id       INTEGER,
    -- 開始日
    start_date           TEXT,
    -- 期限日
    due_date             TEXT,
    -- 予定工数
    estimated_time       INTEGER,
    -- 実工数
    actual_time          INTEGER,
    -- 計画予算
    planed_value         INTEGER,
    -- タスクステータスID
    task_status_id       INTEGER NOT NULL,
    -- 作業進捗率
    progress_rate        INTEGER NOT NULL,
    FOREIGN KEY (assignee_resource_id) REFERENCES resources (resource_id),
    FOREIGN KEY (parent_task_id) REFERENCES tasks (task_id),
    FOREIGN KEY (task_status_id) REFERENCES task_status (task_status_id)
);

-- リソース
CREATE TABLE resources
(
    resource_id    INTEGER PRIMARY KEY,
    name           TEXT    NOT NULL,
    cost_per_month INTEGER NOT NULL
);

INSERT INTO resources
    (resource_id, name, cost_per_month)
VALUES (0, 'Aさん', 100000)
     , (1, 'Bさん', 200000)
     , (2, 'Cさん', 300000)
     , (3, 'Dさん', 400000)
     , (4, 'Eさん', 500000)
     , (5, 'Fさん', 600000)
;

-- タスク種別
CREATE TABLE task_types
(
    task_type_id INTEGER PRIMARY KEY,
    name         TEXT NOT NULL
);

INSERT INTO task_types
    (task_type_id, name)
VALUES (1, 'タスク'),
       (2, 'バグ'),
       (3, 'レビュー'),
       (4, '打合せ'),
       (5, 'QMS'),
       (99, 'その他')
;