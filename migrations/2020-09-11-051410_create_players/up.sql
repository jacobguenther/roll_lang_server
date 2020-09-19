-- Your SQL goes here

CREATE TABLE players (
  id SERIAL,

  name TEXT NOT NULL,
  email TEXT NOT NULL,
  password TEXT NOT NULL,
  hash_salt TEXT NOT NULL,

  is_admin BOOLEAN NOT NULL,

  UNIQUE (name, email),
  PRIMARY KEY (id)
);

/*
 one to one: User has one address
*/

CREATE TABLE settings (
  player_id INT NOT NULL,
  theme TEXT NOT NULL,
  PRIMARY KEY (player_id),
  CONSTRAINT fk_user_id FOREIGN KEY (player_id) REFERENCES players (id)
);