SELECT IFNULL(SUM(planned_value), 0) AS planned_value
FROM tasks
WHERE due_date <= DATE('now', '+9 hours');
