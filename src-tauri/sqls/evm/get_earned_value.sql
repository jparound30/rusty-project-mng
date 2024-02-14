SELECT IFNULL(SUM(earned_value), 0) AS earned_value
FROM (SELECT t.task_id                                             AS task_id,
             CASE
                 WHEN t.task_status_id = 0 THEN 0
                 WHEN t.task_status_id = 1 THEN
                     t.planned_value * (ts.progress_rate + t.progress_rate * (?1 - ?2) / 100) / 100
                 ELSE t.planned_value * ts.progress_rate / 100 END AS earned_value
      FROM tasks t
               JOIN task_status ts ON t.task_status_id = ts.task_status_id

      WHERE planned_value IS NOT NULL
        AND t.task_status_id != 0)
;