-- 集計結果記録用テーブル
CREATE TABLE evm_histories
(
    -- 集計日
    date         TEXT PRIMARY KEY,
    -- 集計日での実コスト
    actual_cost  INTEGER NOT NULL,
    -- 集計日での出来高
    earned_value INTEGER NOT NULL
);

INSERT INTO tasks
(task_id, title, description, assignee_resource_id, parent_task_id, start_date, due_date, estimated_time, actual_time,
 planned_value, task_status_id, progress_rate)
VALUES (1, 'a', 'b', 0, null, '2024-02-10', '2024-02-17', 10, 20, 6250, 0, 10),
       (2, 'aa', 'bb', 0, null, '2024-02-10', '2024-02-17', 10, 20, 1000, 1, 100),
       (3, 'aaa', 'bbb', 0, null, '2024-02-10', '2024-02-17', 10, 20, 6250, 1, 20),
       (4, 'aaaa', 'bbbb', null, null, '2024-02-10', '2024-02-15', 10, 20, 6250, 5, 20),
       (5, 'a', '', null, null, null, null, null, null, null, 0, 0),
       (6, 'asdf', 'asdf', 2, 4, '2024-02-19', '2024-02-23', 30, null, 56250, 0, 0),
       (7, 'くぇｒ', 'くぇｒ', 5, 6, '2024-02-22', '2024-02-29', 100, null, 37500, 0, 0)
;

INSERT INTO evm_histories
    (date, actual_cost, earned_value)
VALUES ('2024-02-10', 0, 0),
       ('2024-02-11', 2000, 1000),
       ('2024-02-12', 4000, 2000),
       ('2024-02-13', 15000, 10000),
       ('2024-02-14', 25000, 22000),
       ('2024-02-17', 28000, 27000),
       ('2024-02-19', 30000, 29000)
;