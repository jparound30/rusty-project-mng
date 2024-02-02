SELECT task_id,
       title,
       description,
       assignee_resource_id,
       parent_task_id,
       start_date,
       due_date,
       estimated_time,
       actual_time,
       planed_value,
       task_status_id,
       progress_rate
FROM tasks
ORDER BY task_id DESC;
