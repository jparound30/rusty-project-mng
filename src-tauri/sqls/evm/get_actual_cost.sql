SELECT IFNULL(SUM(t.actual_time * r.cost_per_month / 160), 0) AS actual_cost
FROM tasks t
         LEFT JOIN main.resources r ON t.assignee_resource_id = r.resource_id
;