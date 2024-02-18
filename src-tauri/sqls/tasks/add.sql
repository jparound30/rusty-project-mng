INSERT INTO tasks
(title, description, assignee_resource_id, parent_task_id, start_date, due_date, estimated_time, actual_time,
 planned_value, task_status_id, progress_rate)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
;