ALTER TABLE users ADD COLUMN password_hash VARCHAR(255) NOT NULL DEFAULT '';

UPDATE users SET password_hash = '' WHERE password_hash = '';

ALTER TABLE users ALTER COLUMN password_hash DROP DEFAULT;
