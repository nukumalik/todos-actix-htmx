-- Add migration script here
CREATE TABLE todos (
  id VARCHAR(100) PRIMARY KEY,
  title TEXT NOT NULL,
  is_complete BOOLEAN NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP
);