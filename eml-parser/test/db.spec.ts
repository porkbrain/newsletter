import { expect } from "chai";
import { newConn, insertInboundEmail } from "../src/db";

describe("db.ts", () => {
  it("should insert email", async () => {
    const conn = await newConn(":memory:");
    await conn.migrate({
      migrationsPath: "../migrations",
    });

    await insertInboundEmail(conn, {
      receivedAt: new Date(),
      recipientAddress: "testrecipientaddress",
      s3Key: "testkey",
      senderAddress: "testsenderaddress",
      senderName: "testsendername",
      subject: "testsubject",
    });

    const email = await conn.get("SELECT * FROM inbound_emails");
    expect(email.s3_key).to.eq("testkey");
    expect(email.recipient_address).to.eq("testrecipientaddress");
    expect(email.sender_address).to.eq("testsenderaddress");
    expect(email.sender_name).to.eq("testsendername");
    expect(email.subject).to.eq("testsubject");

    let tenSecAgo = Math.floor(Date.now() / 1000) - 10;
    let tenSecInFuture = Math.floor(Date.now() / 1000) + 10;
    expect(email.received_at).to.be.greaterThan(tenSecAgo);
    expect(email.received_at).to.be.greaterThan(tenSecAgo);
    expect(email.created_at).to.be.lessThan(tenSecInFuture);
    expect(email.created_at).to.be.lessThan(tenSecInFuture);
  });
});
