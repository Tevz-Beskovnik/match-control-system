-- create table countries
CREATE TABLE IF NOT EXISTS displays(
    id BIGSERIAL NOT NULL PRIMARY KEY,
    display_number INT NOT NULL,
)

CREATE TABLE IF NOT EXISTS countries(
    id BIGSERIAL NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    short_code TEXT NOT NULL,
    flag TEXT NOT NULL
);

-- create table for players
CREATE TABLE IF NOT EXISTS players(
    id BIGSERIAL NOT NULL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    country_id INT NOT NULL REFERENCES countries(id),
    club TEXT
);

CREATE TABLE IF NOT EXISTS umpires(
    id BIGSERIAL NOT NULL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    country_id INT NOT NULL REFERENCES countries(id)
);

CREATE TABLE IF NOT EXISTS "sets"(
    id BIGSERIAL NOT NULL PRIMARY KEY,
    team1_score SMALLINT NOT NULL,
    team2_score SMALLINT NOT NULL
);

CREATE TABLE IF NOT EXISTS statuses(
    "name" TEXT NOT NULL PRIMARY KEY,
    "description" TEXT
);

CREATE TABLE IF NOT EXISTS match_rounds(
    prefix TEXT NOT NULL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS match_types(
    prefix TEXT NOT NULL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS playing_fields(
    id BIGSERIAL NOT NULL PRIMARY KEY,
    field_number INT NOT NULL,
    display_id INT NOT NULL REFERENCES displays(id)
)

CREATE TABLE IF NOT EXISTS matches(
    id BIGSERIAL NOT NULL PRIMARY KEY,
    team1_player1_id INT NOT NULL REFERENCES players(id),
    team1_player2_id INT NOT NULL REFERENCES players(id),
    team2_player1_id INT NOT NULL REFERENCES players(id),
    team2_player2_id INT NOT NULL REFERENCES players(id),
    court_number SMALLINT,
    time_start TIMESTAMP DEFAULT now(),
    time_end TIMESTAMP,
    duration BIGINT DEFAULT 0,
    shuttles_used SMALLINT DEFAULT 0,
    umpire_id INT NOT NULL REFERENCES umpires(id),
    service_judge_id INT NOT NULL REFERENCES umpires(id),
    team1_games SMALLINT,
    team2_games SMALLINT,
    set1_id INT NOT NULL REFERENCES "sets"(id),
    set2_id INT NOT NULL REFERENCES "sets"(id),
    set3_id INT NOT NULL REFERENCES "sets"(id),
    set4_id INT NOT NULL REFERENCES "sets"(id),
    set5_id INT NOT NULL REFERENCES "sets"(id),
    set6_id INT NOT NULL REFERENCES "sets"(id),
    set7_id INT NOT NULL REFERENCES "sets"(id),
    "status" TEXT NOT NULL REFERENCES statuses("name"),
    match_type TEXT NOT NULL REFERENCES match_types(prefix),
    match_round TEXT NOT NULL REFERENCES match_rounds(prefix),
    playing_field INT NOT NULL REFERENCES playing_fields(id)
);