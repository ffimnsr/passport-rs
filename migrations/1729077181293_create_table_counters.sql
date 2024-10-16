-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS counters (
    name TEXT PRIMARY KEY,
    total_count BIGINT NOT NULL
);

CREATE OR REPLACE FUNCTION fn_inc_job_count()
RETURNS TRIGGER
LANGUAGE PLPGSQL
AS $$
BEGIN
    INSERT INTO counters (name, total_count)
    VALUES ('job_count', 1)
    ON CONFLICT(name)
    DO UPDATE SET total_count = counters.total_count + 1;
    RETURN NEW;
END;
$$;

CREATE OR REPLACE FUNCTION fn_dec_job_count()
RETURNS TRIGGER
LANGUAGE PLPGSQL
AS $$
BEGIN
    INSERT INTO counters (name, total_count)
    VALUES ('job_count', 0)
    ON CONFLICT(name)
    DO UPDATE SET total_count = counters.total_count - 1;
    RETURN OLD;
END;
$$;

CREATE TRIGGER tr_inc_job_count
AFTER INSERT ON jobs
FOR EACH ROW
EXECUTE FUNCTION fn_inc_job_count();

CREATE TRIGGER tr_dec_job_count
AFTER DELETE ON jobs
FOR EACH ROW
EXECUTE FUNCTION fn_dec_job_count();

-- !DOWN

DROP TRIGGER tr_dec_job_count ON jobs;
DROP TRIGGER tr_inc_job_count ON jobs;
DROP FUNCTION fn_dec_job_count();
DROP FUNCTION fn_inc_job_count();
DROP TABLE counters;
