import sqlite3 from "sqlite3";
import { open, Database } from "sqlite";

export type Conn = Database<sqlite3.Database, sqlite3.Statement>;

export function newConn(dbFile: string): Promise<Conn> {
  return open({
    filename: dbFile,
    driver: sqlite3.Database,
  });
}

export interface InboundEmail {
  // random string given to the email when SES stores it in S3
  s3Key: string;
  senderAddress: string;
  recipientAddress: string;
  senderName: string | null;
  subject: string | null;
  receivedAt: Date;
}

export async function insertInboundEmail(conn: Conn, email: InboundEmail) {
  await conn.run(
    `
  insert into inbound_emails (
      s3_key,
      sender_address,
      sender_name,
      recipient_address,
      subject,
      received_at
  ) values (?, ?, ?, ?, ?, ?)
  `,
    email.s3Key,
    email.senderAddress,
    email.senderName,
    email.recipientAddress,
    email.subject,
    // we store time as unix time in seconds
    Math.floor(email.receivedAt.getTime() / 1000)
  );
}
