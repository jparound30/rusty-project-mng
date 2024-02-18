SELECT user_id
     , username AS 'username: String'
     , password_hash
     , salt
FROM users
WHERE username = ?1
;