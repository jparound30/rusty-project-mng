UPDATE resources
SET name           = ?1
  , cost_per_month = ?2
WHERE resource_id = ?3
;