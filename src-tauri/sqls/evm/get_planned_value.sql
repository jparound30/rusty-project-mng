select IFNULL(SUM(planned_value), 0) as planned_value
FROM tasks
WHERE
    due_date <= DATE('now', '+9 hours');

