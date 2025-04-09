
# 创建 down.sql 文件
@'
DROP TABLE IF EXISTS role_permissions;
DROP TABLE IF EXISTS user_roles;
DROP TABLE IF EXISTS permissions;
DROP TABLE IF EXISTS roles;
DROP TABLE IF EXISTS users;
'@ | Out-File -FilePath migrations\2025_04_08_000000_create_auth_tables\down.sql -Encoding utf8