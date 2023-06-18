CREATE TABLE threads (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  forum_id VARCHAR NOT NULL,
  FOREIGN KEY (forum_id) REFERENCES forums(id)
);
