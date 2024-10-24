-- # Put the your SQL below migration seperator.
-- !UP

CREATE TYPE pubsub_message_status AS ENUM (
  'pending',
  'delivered'
);

CREATE TABLE IF NOT EXISTS pubsub_job_announcements (
    job_id VARCHAR(24) PRIMARY KEY REFERENCES jobs(id) ON DELETE CASCADE,
    message_id INT NULL,
    message_status pubsub_message_status NOT NULL DEFAULT 'pending',
    scheduled_delivery_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    delivery_attemps INT DEFAULT 0,
    delivered_at TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER tr_mod_updated_at
BEFORE UPDATE ON pubsub_job_announcements
FOR EACH ROW
EXECUTE FUNCTION moddatetime(updated_at);

CREATE OR REPLACE FUNCTION fn_notify_job_announcer()
RETURNS TRIGGER
LANGUAGE PLPGSQL
AS $$
BEGIN
  INSERT INTO pubsub_job_announcements (job_id)
  VALUES (NEW.id);

  PERFORM pg_notify('job_announcements', NEW.id::TEXT);

  RETURN NEW;
END;
$$;

CREATE TRIGGER tr_notify_job_announcer
AFTER INSERT ON jobs
FOR EACH ROW
EXECUTE FUNCTION fn_notify_job_announcer();

-- !DOWN

DROP TRIGGER IF EXISTS tr_notify_job_announcer ON jobs;
DROP FUNCTION IF EXISTS fn_notify_job_announcer();
DROP TRIGGER IF EXISTS tr_mod_updated_at ON jobs;
DROP TABLE pubsub_job_announcements;
DROP TYPE pubsub_message_status;
