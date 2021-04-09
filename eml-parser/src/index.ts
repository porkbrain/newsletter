import { readEnv } from "./conf";
import { parseEmailFromEmlHtml } from "./parse";
import { fetchFromS3, uploadHtmlToS3 } from "./s3";
import { Sqs } from "./sqs";
import { newConn, insertInboundEmail } from "./db";

async function main() {
  const conf = readEnv();
  const inputSqs = Sqs.new(conf.inputNewMailQueue);
  const conn = newConn(conf.db);

  let failedInRow = 0;
  while (true) {
    try {
      const message = await inputSqs.receive();
      const { id: s3Key, body } = await fetchFromS3(
        conf.emailsBucketName,
        message.body()
      );

      const {
        senderAddress,
        senderName,
        recipientAddress,
        receivedAt,
        subject,
        html,
      } = await parseEmailFromEmlHtml(s3Key, body);

      console.log(`[${new Date()}] Inserting email in ${s3Key}...`);
      await insertInboundEmail(conn, {
        recipientAddress,
        s3Key,
        senderAddress,
        senderName,
        receivedAt,
        subject,
      });
      await uploadHtmlToS3(conf.htmlBucketName, s3Key, html);
      // TODO: insert new job
      await message.delete();
      failedInRow = 0;
    } catch (err) {
      failedInRow++;
      console.log(`[${new Date()}] Error: `, err);
      if (failedInRow > conf.maxFailedInRow) {
        throw new Error(
          `Failed to process ${conf.maxFailedInRow} subsequent messages`
        );
      }
    }
  }
}

main()
  .catch((e) => console.error(`[${new Date()}] Main threw error:`, e))
  .then(() => console.log(`[${new Date()}] Exiting...`));
