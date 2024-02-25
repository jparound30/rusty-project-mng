UPDATE tasks
SET title       = ?2
  , description = ?3
  , assignee_resource_id = ?4
  , parent_task_id = ?5
  , start_date = ?6
  , due_date = ?7
  , estimated_time = ?8
  , actual_time = ?9
  , planned_value = ?10
  , task_status_id = ?11
  , progress_rate = ?12
WHERE task_id = ?1;
;