CREATE TABLE IF NOT EXISTS inbound_emails (
    -- random string given to the email when SES stores it in S3
    s3_key VARCHAR (40) PRIMARY KEY,
    recipient_address TEXT NOT NULL,
    sender_address TEXT NOT NULL,
    sender_name TEXT,
    subject TEXT,
    -- UNIX time in seconds
    received_at INTEGER(4) NOT NULL,
    -- https://stackoverflow.com/a/26127039/5093093
    created_at INTEGER(4) NOT NULL DEFAULT (strftime('%s','now'))
);
