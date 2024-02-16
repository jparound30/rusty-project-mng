SELECT IFNULL(MIN(tasks.start_date), DATE('now', '+9 hours')) AS start_date
FROM tasks;