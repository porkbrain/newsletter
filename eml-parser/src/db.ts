/**
 * Postgres database follows schema in the source of truth directory of
 * `deployment/migrations`.
 *
 * We can write sql with interpolated variables with `${ ... }` because the lib
 * [escapes][postgres-escape] the variables.
 *
 * [postgres-escape]: https://github.com/porsager/postgres#tagged-template-function-sql
 */

import postgres from "postgres";
import { DEBUG } from "./conf";

export type Conn = postgres.Sql<{}>;
export type ConnOptions = postgres.Options<{}>;

export function newConn(opt: ConnOptions): Conn {
  return postgres({
    user: "newsletter",
    host: "localhost",
    port: 5432,
    db: "newsletter",
    debug: DEBUG,
    max: 1,
    ...opt,
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

/**
 * @param conn
 * @param email
 * @returns Id of the newly created email
 */
export async function insertInboundEmail(conn: Conn, email: InboundEmail) {
  const rows = await conn`
    insert into inbound_emails(
      s3_key,
      sender_address,
      sender_name,
      recipient_address,
      subject,
      received_at
    ) values (
      ${email.s3Key},
      ${email.senderAddress},
      ${email.senderName},
      ${email.recipientAddress},
      ${email.subject},
      ${email.receivedAt}
    )
  `;
}
