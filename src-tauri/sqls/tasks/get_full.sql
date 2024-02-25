SELECT t.task_id as task_id,
       t.title as title,
       t.description as description,
       t.assignee_resource_id as assignee_resource_id,
       r.name as assignee_resource_name,
       t.parent_task_id as parent_task_id,
       t2.title as parent_task_title,
       t.start_date as start_date,
       t.due_date as due_date,
       t.estimated_time as estimated_time,
       t.actual_time as actual_time,
       t.planned_value as planned_value,
       t.task_status_id as task_status_id,
       ts.title as task_status_name,
       t.progress_rate as progress_rate
FROM tasks t
JOIN task_status ts on t.task_status_id = ts.task_status_id
LEFT JOIN tasks t2 on t.parent_task_id = t2.task_id
LEFT JOIN resources r on t.assignee_resource_id = r.resource_id
WHERE t.task_id = ?1
;
