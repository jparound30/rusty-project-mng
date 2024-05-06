SELECT resource_id AS 'resource_id: i64'
     , name
     , cost_per_month AS 'cost_per_month: i64'
FROM resources
WHERE
    resource_id = ?1
;