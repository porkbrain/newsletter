const sqlite3 = require("sqlite3");
const { open, Database } = require("sqlite");
const {
  GoogleSpreadsheet,
  GoogleSpreadsheetWorksheet,
} = require("google-spreadsheet");

/**
 * Env setup.
 */

const receiver = process.env.RECEIVER_EMAIL;
if (!receiver) {
  throw new Error("Missing receiver email");
}

const htmlUrl = process.env.HTML_URL;
if (!htmlUrl) {
  throw new Error("Missing html url");
}

const googleClientEmail = process.env.GOOGLE_SERVICE_ACCOUNT_EMAIL;
if (!googleClientEmail) {
  throw new Error("Missing google client email.");
}

const googlePrivateKey = process.env.GOOGLE_PRIVATE_KEY;
if (!googlePrivateKey) {
  // https://theoephraim.github.io/node-google-spreadsheet/#/getting-started/authentication?id=service-account
  throw new Error("Missing google private key.");
}

const dbPath = process.env.DATABASE_PATH;
if (!dbPath) {
  throw new Error("Missing sqlite3 db path");
}

const sheetId = process.env.SHEET_ID;
if (!sheetId) {
  throw new Error("Missing sheet id");
}

/**
 * Returns ids of emails for GSG which haven't been synced yet with the google
 * sheet of results.
 *
 * @return {Promise<Array<{
 *  id: string,
 *  subject?: string,
 *  senderName?: string
 * }>>}
 */
async function selectNewGsgEmails(conn) {
  const sql = `SELECT s3_key, subject, sender_name FROM inbound_emails WHERE recipient_address = '${receiver}' AND state != 'synced'`;
  const ids = [];
  await conn.each(sql, (err, row) => {
    if (err) {
      console.error("Cannot get row due to error:", err);
      throw new Error(err.toString());
    }

    ids.push({
      id: row.s3_key,
      subject: row.subject,
      senderName: row.sender_name,
    });
  });

  return ids;
}

/**
 * Returns all offers gathered for given email id.
 *
 * @return {Promise<{
 *  deal: string,
 *  voucher?: string,
 *  link?: string,
 * }>}
 */
async function selectOffersForEmail(conn, id) {
  const sql = `SELECT deal, voucher, link FROM offers WHERE s3_key = '${id}'`;
  const offers = [];
  await conn.each(sql, (err, row) => {
    if (err) {
      console.error("Cannot get row due to error:", err);
      throw new Error(err.toString());
    }

    offers.push({
      deal: row.deal,
      voucher: row.voucher,
      link: row.link,
    });
  });

  return offers;
}

/**
 * Sets offers for given newsletters as synced.
 */
async function markOffersAsSynced(conn, ids) {
  const idsCsv = ids.map((s) => `'${s}'`).join(",");
  await conn.run(
    `UPDATE inbound_emails SET state = 'synced' WHERE s3_key IN (${idsCsv})`
  );
}

/**
 * Given a sheet handle and list of offers associated with email ids, inserts
 * them as new rows.
 *
 * @param sheet {GoogleSpreadsheetWorksheet}
 * @param emails {{
 *  id: string,
 *  offers: Array<{
 *  deal: string,
 *  voucher?: string,
 *  link?: string
 *  }
 * }}
 */
async function insertOffersIntoGoogleSheet(sheet, emails) {
  const newRows = [];

  for (const { email, offers } of emails) {
    for (const offer of offers) {
      const offerRow = [];

      offerRow.push(new Date().toJSON()); // Date
      offerRow.push(email.subject || ""); // Subject
      offerRow.push(email.senderName || ""); // Sender
      offerRow.push(offer.deal || ""); // Short Text
      offerRow.push(offer.voucher ? "Code" : "Deal"); // Code or Deal
      offerRow.push(offer.voucher || ""); // Code Value
      offerRow.push(""); // TODO: Unique code estimation
      offerRow.push(offer.link || ""); // Deeplink / links
      offerRow.push(`${htmlUrl}/${email.id}`); // Email with links

      newRows.push(offerRow);
    }
  }

  console.log(`Inserting ${newRows.length} new rows to Google Sheet`);
  await sheet.addRows(newRows);
}

async function main() {
  const conn = await open({
    filename: dbPath,
    driver: sqlite3.Database,
  });

  const doc = new GoogleSpreadsheet(sheetId);
  await doc.useServiceAccountAuth({
    client_email: googleClientEmail,
    private_key: googlePrivateKey,
  });
  await doc.loadInfo();
  const sheet = doc.sheetsByIndex[0];

  const emails = await selectNewGsgEmails(conn);
  console.log(`Found ${emails.length} new GSG emails`);
  const emailsWithOffers = (
    await Promise.all(
      emails.map(async (email) => {
        try {
          const offers = await selectOffersForEmail(conn, email.id);
          return { email, offers };
        } catch (err) {
          console.error(`Error selecting offers for email ${id} due to`, err);
        }
      })
    )
  ).filter(Boolean);

  console.log("Inserting into Google Sheets");
  await insertOffersIntoGoogleSheet(sheet, emailsWithOffers);

  console.log("Marking as synced");
  await markOffersAsSynced(
    conn,
    emails.map((e) => e.id)
  );
}

main()
  .then(() => console.log("Done"))
  .catch(console.error);
