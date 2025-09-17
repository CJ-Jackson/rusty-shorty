SELECT u.id, u.username, u.role
FROM backoffice_users AS u
         INNER JOIN user_login_tokens ult on u.id = ult.user_id
WHERE ult.token = :token
  AND ult.expire_after > datetime('now')
LIMIT 1;