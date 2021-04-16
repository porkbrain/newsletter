import { expect } from "chai";
import { parseEmailFromEmlHtml } from "../src/parse";
import { readFileSync } from "fs";

describe("parse.ts", () => {
  it("should parse eml file", async () => {
    const sampleEml = readFileSync("test/parse.in").toString();
    const sampleHtml = readFileSync("test/parse.out").toString();

    const id = "test-id";
    const {
      html,
      recipientAddress,
      senderAddress,
      senderName,
      subject,
      receivedAt,
    } = await parseEmailFromEmlHtml(id, sampleEml);

    expect(recipientAddress).to.eq("test@prizeprofile.com");
    expect(senderAddress).to.eq("email@emailnastygal.com");
    expect(senderName).to.eq("Nasty Gal");
    expect(html).to.eq(sampleHtml);
    expect(subject).to.eq("RE: Your Winter Wardrobe");
    expect(receivedAt.toDateString()).to.eq("Sun Dec 13 2020");
  });
});
