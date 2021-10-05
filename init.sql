/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

CREATE TABLE users (
  id bigint PRIMARY KEY,
  username varchar(32) NOT NULL,
  discriminator smallint NOT NULL,
  avatar varchar(34),
  permissions int NOT NULL DEFAULT 63,
  points int NOT NULL DEFAULT 0
);

CREATE TABLE memes (
  id int GENERATED ALWAYS AS IDENTITY,
  name varchar(24) NOT NULL UNIQUE,
  user_id bigint NOT NULL,
  timestamp bigint NOT NULL,
  views bigint NOT NULL DEFAULT 0,
  votes bigint NOT NULL DEFAULT 0,
  resolution smallint NOT NULL,
  explicit boolean NOT NULL DEFAULT false,
  PRIMARY KEY (id),
  CONSTRAINT memes_user_id FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE NO ACTION ON UPDATE NO ACTION
);

CREATE TABLE meme_votes (
  meme_id int,
  user_id bigint,
  vote smallint NOT NULL,
  PRIMARY KEY (meme_id, user_id),
  CONSTRAINT meme_votes_meme_id FOREIGN KEY (meme_id) REFERENCES memes (id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT meme_votes_user_id FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE tags (
  id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  name varchar(24) NOT NULL UNIQUE,
  explicit boolean NOT NULL DEFAULT false
);

CREATE TABLE meme_tags (
  meme_id int NOT NULL,
  tag_id int NOT NULL,
  CONSTRAINT meme_tags_meme_id FOREIGN KEY (meme_id) REFERENCES memes (id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT meme_tags_tag_id FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE comments (
  id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  content text NOT NULL,
  user_id bigint NOT NULL,
  timestamp bigint NOT NULL,
  parent_id int DEFAULT NULL,
  CONSTRAINT comments_user_id FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE NO ACTION ON UPDATE NO ACTION,
  CONSTRAINT comments_parent_id FOREIGN KEY (parent_id) REFERENCES comments (id) ON DELETE NO ACTION ON UPDATE NO ACTION
);

CREATE TABLE meme_comments (
  meme_id int NOT NULL,
  comment_id int NOT NULL,
  CONSTRAINT meme_comments_meme_id FOREIGN KEY (meme_id) REFERENCES memes (id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT meme_comments_comment_id FOREIGN KEY (comment_id) REFERENCES comments (id) ON DELETE CASCADE ON UPDATE CASCADE
);