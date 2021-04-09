/**
 * Gets the html from the eml file.
 */

import { simpleParser } from "mailparser";

export interface EmailData {
  recipientAddress: string;
  senderName: string | null;
  senderAddress: string;
  receivedAt: Date;
  subject: string | null;
  html: string;
}

const emailParserOptions = {};

export async function parseEmailFromEmlHtml(
  s3Key: string,
  body: string
): Promise<EmailData> {
  const {
    textAsHtml,
    html: htmlContent,
    from,
    subject,
    to: recipient,
    date,
  } = await simpleParser(body, emailParserOptions);

  if (!htmlContent && !textAsHtml) {
    throw new Error(`Missing email html in ${s3Key}.`);
  }

  if (!from || !Array.isArray(from.value) || from.value.length === 0) {
    throw new Error(`Missing from header in ${s3Key}.`);
  }

  const { address, name } = from.value[0];
  if (!address) {
    throw new Error(`Missing from address in ${s3Key}.`);
  }

  if (
    !recipient ||
    !Array.isArray(recipient.value) ||
    recipient.value.length === 0
  ) {
    throw new Error(`Missing recipient header in ${s3Key}.`);
  }

  const { address: recipientAddress } = recipient.value[0];
  if (!recipientAddress) {
    throw new Error(`Missing recipient address in ${s3Key}.`);
  }

  const html = htmlContent || (textAsHtml as string);

  return {
    recipientAddress,
    senderName: name || null,
    senderAddress: address,
    subject: subject || null,
    receivedAt: date || new Date(),
    html,
  };
}
