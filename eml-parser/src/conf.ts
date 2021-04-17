import aws from "aws-sdk";
import { config as dotenv } from "dotenv";
import { SqsConf } from "./sqs";

// see deployment script for the service
dotenv();

// Use specific API versions.
aws.config.apiVersions = {
  sqs: "2012-11-05",
};

export const defaultRegion = process.env.AWS_DEFAULT_REGION || "eu-west-1";
console.log(`Updating default region to ${defaultRegion}.`);
aws.config.update({ region: defaultRegion });

export interface Conf {
  inputNewMailQueue: SqsConf;
  emailsBucketName: string;
  htmlBucketName: string;
  dbName: string;
  maxFailedInRow: number;
}

/**
 * Creates a new configuration object from process environment variables or
 * throws an error if misconfigured.
 */
export function readEnv(): Conf {
  const inputQueueUrl = process.env.INPUT_QUEUE_URL;
  if (!inputQueueUrl) {
    throw new Error("INPUT_QUEUE_URL env var must be provided.");
  }

  const maxFailedInRow = parseInt(process.env.MAX_FAILED_IN_ROW || "10") || 10;

  const emailsBucketName = process.env.EMAILS_BUCKET_NAME || "mailmevouchers";
  const htmlBucketName =
    process.env.HTML_BUCKET_NAME || "assets.mailmevouchers.com";

  const dbName = process.env.DATABASE;
  if (!dbName) {
    throw new Error("DATABASE env var must be provided");
  }

  return {
    emailsBucketName,
    htmlBucketName,
    inputNewMailQueue: { url: inputQueueUrl },
    dbName,
    maxFailedInRow,
  };
}
