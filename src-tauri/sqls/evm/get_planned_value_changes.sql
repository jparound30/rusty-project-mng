WITH RECURSIVE date_list AS (SELECT DATE(?1) AS generated_date
                             UNION ALL
                             SELECT DATE(generated_date, '+1 day')
                             FROM date_list
                             WHERE generated_date < ?2)
SELECT generated_date,
       (SELECT IFNULL(SUM(planned_value), 0)
        FROM tasks
        WHERE due_date <= DATE(generated_date, '+9 hours')) AS planned_value
FROM date_list
;