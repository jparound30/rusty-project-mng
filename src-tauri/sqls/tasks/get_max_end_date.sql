SELECT IFNULL(MAX(tasks.due_date), DATE('now', '+9 hours')) AS end_date
FROM tasks;