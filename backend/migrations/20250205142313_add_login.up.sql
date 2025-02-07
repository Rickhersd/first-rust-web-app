-- Add up migration script here
-- Add up migration script here
-- ALTER TABLE users DROP CONSTRAINT "users_password_key";
ALTER TABLE users ADD COLUMN login text not null unique;
ALTER TABLE users ALTER column name type text;
 
 
