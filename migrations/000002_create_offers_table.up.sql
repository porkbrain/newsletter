CREATE TABLE IF NOT EXISTS offers (
    -- random string given to the email when SES stores it in S3
    s3_key VARCHAR (40) NOT NULL,
    deal TEXT NOT NULL,
    voucher TEXT,
    link TEXT,
    state TEXT NOT NULL DEFAULT 'new',
    -- https://stackoverflow.com/a/26127039/5093093
    created_at INTEGER(4) NOT NULL DEFAULT (strftime('%s','now'))
);

-- TODO: not idempotent
ALTER TABLE inbound_emails ADD COLUMN state TEXT DEFAULT 'new';
