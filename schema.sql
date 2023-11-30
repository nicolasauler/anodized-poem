--DROP TABLE IF EXISTS attendance;

CREATE TABLE IF NOT EXISTS attendance
(
    id          BIGSERIAL PRIMARY KEY NOT NULL,
    level       TEXT                NOT NULL,
    teacher     TEXT                NOT NULL,
    date        DATE NOT NULL
);
