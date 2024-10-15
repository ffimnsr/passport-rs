-- # Put the your SQL below migration seperator.
-- !UP

ALTER TABLE jobs
ADD COLUMN fts tsvector
GENERATED ALWAYS
AS (to_tsvector('english', description || ' ' || title)) STORED;

CREATE INDEX jobs_fts_idx ON jobs USING gin (fts);

-- !DOWN

DROP INDEX jobs_fts_idx;

ALTER TABLE jobs
DROP COLUMN fts;
