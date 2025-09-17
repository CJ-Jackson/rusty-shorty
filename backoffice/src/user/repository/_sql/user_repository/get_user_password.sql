SELECT id, password
FROM backoffice_users
WHERE username = :username
LIMIT 1;