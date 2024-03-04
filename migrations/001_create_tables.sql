CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  tg_id BIGINT NOT NULL UNIQUE,
  username VARCHAR(255) NOT NULL UNIQUE,
  password_hash VARCHAR(255) NOT NULL,
  fio VARCHAR(255) NOT NULL,
  groups VARCHAR(255) NOT NULL,
  description VARCHAR(255) NULL,
  token VARCHAR(255) NULL,
);

INSERT INTO users values(
  1, 200712816, 'MedovikovOE',
  '$2b$12$W4pxNBAhbY9iHDf/XLndUOufVngbPERmFGbhzLLpbKOd/zLGHYxCO',
  'Медовиков О.Е.', 'admin', 'Автор', NULL
);

CREATE TABLE IF NOT EXISTS commands (
  id SERIAL PRIMARY KEY,
  category VARCHAR(255) NOT NULL,
  name VARCHAR(255) NOT NULL,
  func VARCHAR(255) NOT NULL,
  arg VARCHAR(255) NULL,
  return_file BOOLEAN NOT NULL,
  ask_day BOOLEAN NOT NULL,
  active BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS access (
  client SERIAL FOREIGN KEY client REFERENCES users(id),
  command SERIAL FOREIGN KEY command REFERENCES commands(id),
  coment VARCHAR(255) NULL 
);

CREATE TABLE IF NOT EXISTS dirs (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  active BOOLEAN NOT NULL

);

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS binarys(
  guid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  file_hash TEXT NOT NULL,
  file_data BYTEA NOT NULL
);

CREATE TABLE IF NOT EXISTS files (
  guid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  time_create TIMESTAMP NOT NULL DEFAULT NOW(),
  name VARCHAR(255),
  binary UUID FOREIGN KEY (binary) REFERENCES binarys(guid)
);

CREATE TABLE IF NOT EXISTS messages (
  guid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  message TEXT NOT NULL UNIQUE
);


CREATE TABLE IF NOT EXISTS tasks (
  guid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  time_create TIMESTAMP NOT NULL DEFAULT NOW(),
  client SERIAL FOREIGN KEY (client) REFERENCES users(id),
  scheduler BOOLEAN NOT NULL,
  command SERIAL FOREIGN KEY (command) REFERENCES commands(id),
  func VARCHAR(255) NOT NULL,
  arg VARCHAR(255) NULL,
  users_list BIGINT[],
  time_start TIMESTAMP NULL,
  time_stop  TIMESTAMP NULL,
  files UUID[],
  message UUID FOREIGN KEY (message) REFERENCES messages(guid)
);


ALTER TABLE tasks
  ADD CONSTRAINT files FOREIGN KEY (files) REFERENCES files(guid);
