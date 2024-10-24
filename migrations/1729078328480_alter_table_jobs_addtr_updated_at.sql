-- # Put the your SQL below migration seperator.
-- !UP

CREATE EXTENSION IF NOT EXISTS moddatetime;

CREATE TRIGGER tr_mod_updated_at
BEFORE UPDATE ON jobs
FOR EACH ROW
EXECUTE FUNCTION moddatetime(updated_at);

-- !DOWN

DROP TRIGGER IF EXISTS tr_mod_updated_at ON jobs;
DROP EXTENSION moddatetime;
