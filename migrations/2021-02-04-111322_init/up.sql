CREATE TABLE users (
  id BIGINT NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  alias TEXT NOT NULL UNIQUE,
  description TEXT,
  avatar TEXT,
  inserted_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL
);

CREATE TABLE posts (
  topic_id BIGINT NOT NULL,
  id UUID NOT NULL PRIMARY KEY,
  floor INTEGER NOT NULL,
  author_id BIGINT NOT NULL,
  author_name TEXT NOT NULL,
  content INTEGER NOT NULL,
  revision INTEGER NOT NULL DEFAULT 0,
  inserted_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  parent_id UUID REFERENCES posts (id)
);
