/*
BEGIN;
DO
  $do$
  BEGIN

    EXECUTE
     (
       SELECT 'DROP TABLE ' || string_agg(format('%I', tablename), ', ') || ' CASCADE'
       FROM   pg_tables
       WHERE  schemaname = 'public'  -- убедитесь, что вы удаляете только из схемы public
     );
  END
  $do$;
COMMIT;
*/
/*
drop table users cascade;
drop table commands cascade;
drop table access cascade;
*/

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
  guid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  tg_id BIGINT NOT NULL UNIQUE,
  username VARCHAR(255) NOT NULL UNIQUE,
  password_hash VARCHAR(255) NOT NULL,
  fio VARCHAR(255) NOT NULL,
  groups VARCHAR(255) NOT NULL,
  description VARCHAR(255) NULL,
  active BOOLEAN NOT NULL,
  token VARCHAR(255) NULL
);

INSERT INTO users values(
  uuid_generate_v4(), 200712816, 'MedovikovOE',
  '$2b$12$W4pxNBAhbY9iHDf/XLndUOufVngbPERmFGbhzLLpbKOd/zLGHYxCO',
  'Медовиков О.Е.', 'admin', 'Автор', true, NULL
) ON CONFLICT DO NOTHING;

CREATE TABLE IF NOT EXISTS commands (
  guid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  category VARCHAR(255) NOT NULL,
  name VARCHAR(255) NOT NULL UNIQUE,
  func VARCHAR(255) NOT NULL,
  arg VARCHAR(255) NULL,
  return_file BOOLEAN NOT NULL,
  ask_day BOOLEAN NOT NULL,
  description TEXT NULL,
  active BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS access (
  client UUID, 
  command UUID,
  coment VARCHAR(255) NULL,
  FOREIGN KEY (client) REFERENCES users(guid),
  FOREIGN KEY (command) REFERENCES commands(guid)
);

CREATE TABLE IF NOT EXISTS dirs (
  guid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name VARCHAR(255) NOT NULL,
  directory TEXT NOT NULL,
  description TEXT NOT NULL,
  active BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS binarys(
  guid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  file_hash TEXT NOT NULL,
  file_data BYTEA NOT NULL
);

CREATE TABLE IF NOT EXISTS files (
  guid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  time_create TIMESTAMP NOT NULL DEFAULT NOW(),
  name VARCHAR(255),
  file_bin UUID NOT NULL,
  FOREIGN KEY (file_bin) REFERENCES binarys(guid)
);

CREATE TABLE IF NOT EXISTS messages (
  guid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  message TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS tasks (
  guid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  time_create TIMESTAMP NOT NULL DEFAULT NOW(),
  client UUID NOT NULL,
  scheduler BOOLEAN NOT NULL,
  command UUID NOT NULL,
  func VARCHAR(255) NOT NULL,
  arg VARCHAR(255) NULL,
  users_list BIGINT[],
  time_start TIMESTAMP NULL,
  time_stop  TIMESTAMP NULL,
  message UUID NULL,
  FOREIGN KEY (client) REFERENCES users(guid),
  FOREIGN KEY (command) REFERENCES commands(guid),
  FOREIGN KEY (message) REFERENCES messages(guid)
);

CREATE TABLE IF NOT EXISTS tasks_x_files (
  task UUID NOT NULL, 
  file UUID NOT NULL,
  FOREIGN KEY (task) REFERENCES tasks(guid),
  FOREIGN KEY (file) REFERENCES files(guid)
);

CREATE TABLE IF NOT EXISTS users_x_files (
  user_guid UUID NOT NULL, 
  file UUID NOT NULL,
  FOREIGN KEY (user_guid) REFERENCES users(guid),
  FOREIGN KEY (file) REFERENCES files(guid)
);
