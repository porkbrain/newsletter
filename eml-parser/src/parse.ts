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
  let {
    textAsHtml,
    html: htmlContent,
    from,
    subject,
    to: recipients,
    date,
  } = await simpleParser(body, emailParserOptions);

  if (!htmlContent && !textAsHtml) {
    throw new Error(`Missing email html in ${s3Key}.`);
  }

  let html = htmlContent || (textAsHtml as string);

  const recipient = Array.isArray(recipients) ? recipients.shift() : recipients;
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

  // some clients forward us emails from gmail, which means we have to go
  // through some extra steps to figure out who the email originated from
  if (isGmailForwarded(subject, html)) {
    subject = (subject || "").replace("Fwd: ", "");

    const {
      html: updatedHtml,
      senderName,
      senderAddress,
    } = extractAndRemoveFwdInfoFromGmailFwd(html);
    html = updatedHtml;

    if (senderAddress && senderName) {
      from = {
        value: [{ name: senderName, address: senderAddress }],
        html: "",
        text: "",
      };
    }
  }

  if (!from || !Array.isArray(from.value) || from.value.length === 0) {
    throw new Error(`Missing from header in ${s3Key}.`);
  }

  const { address, name } = from.value[0];
  if (!address) {
    throw new Error(`Missing from address in ${s3Key}.`);
  }

  return {
    recipientAddress,
    senderName: name || null,
    senderAddress: address,
    subject: subject || null,
    receivedAt: date || new Date(),
    html,
  };
}

/**
 * Some clients forward us emails from gmail to scan them.
 */
function isGmailForwarded(subject: string = "", html: string): boolean {
  return (
    /Fwd:\s/.test(subject) &&
    /---------- Forwarded message ---------/.test(html) &&
    /gmail_attr/.test(html)
  );
}

/**
 * Given an email forwarded from Gmail, strip the fwd information from html and
 * find who was the original sender
 */
function extractAndRemoveFwdInfoFromGmailFwd(
  html: string
): { html: string; senderAddress?: string; senderName?: string } {
  const MATCH_FWD_INFO = /<div (dir="\w+")? class="gmail_attr">.*?<\/div>/gi;
  const MATCH_SENDER_INFO = /gmail_sendername.*?>(.*?)<.*?mailto:(.*?)">/gi;

  const fwdInfoMatches = MATCH_FWD_INFO.exec(html);
  if (!fwdInfoMatches) {
    return { html, senderAddress: undefined, senderName: undefined };
  }

  // get rid of the needless fwd info
  const fwdInfo = fwdInfoMatches[0];
  html = html.replace(fwdInfo, "");

  const senderInfoMatches = MATCH_SENDER_INFO.exec(fwdInfo);
  if (!senderInfoMatches) {
    return { html, senderName: undefined, senderAddress: undefined };
  }

  return {
    html,
    senderName: senderInfoMatches[1],
    senderAddress: senderInfoMatches[2],
  };
}
