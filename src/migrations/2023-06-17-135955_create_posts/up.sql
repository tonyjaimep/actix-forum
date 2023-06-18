CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  content TEXT NOT NULL,
  thread_id INTEGER NOT NULL,
  FOREIGN KEY (thread_id) REFERENCES threads(id)
);
