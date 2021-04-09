import aws from "aws-sdk";

/**
 * Message published by S3 when a new object is created. Sent into `new_email`
 * queue.
 *
 * https://docs.aws.amazon.com/AmazonS3/latest/dev/notification-content-structure.html
 */
export interface S3CreateEvent {
  Records: Array<{
    eventVersion: string;
    s3: {
      bucket: { name: string };
      object: { key: string; size: number };
    };
  }>;
}

export interface S3Object {
  id: string;
  body: string;
}

/**
 * @param bucketName Bucket where emails are stored by SES
 * @param sqsMessageBody Stringified https://docs.aws.amazon.com/AmazonS3/latest/dev/notification-content-structure.html
 */
export async function fetchFromS3(
  bucketName: string,
  event: S3CreateEvent
): Promise<S3Object> {
  const { eventVersion, s3 } = event.Records.pop()!;

  console.log(
    `Received an S3 event version ${eventVersion} of size ${s3.object.size}.`
  );

  if (s3.bucket.name !== bucketName) {
    throw new Error(
      `Received an event from bucket ${s3.bucket.name}, but expected ${bucketName}.`
    );
  }

  const objectName = decodeURI(s3.object.key); // a.k.a. email id

  const params: aws.S3.Types.GetObjectRequest = {
    Bucket: bucketName,
    Key: objectName,
  };
  const { Body } = await new aws.S3().getObject(params).promise();

  return {
    id: objectName,
    body: Body!.toString(),
  };
}

/**
 * @param bucketName
 * @param s3Key
 * @param html
 */
export async function uploadHtmlToS3(
  bucketName: string,
  s3Key: string,
  html: aws.S3.Types.Body
) {
  const params: aws.S3.Types.PutObjectRequest = {
    Body: html,
    ACL: "public-read", // users can access email html previews
    Bucket: bucketName,
    CacheControl: "public, immutable",
    ContentType: "text/html",
    StorageClass: "REDUCED_REDUNDANCY",
    Key: `html/${s3Key}.html`,
  };

  await new aws.S3().putObject(params).promise();
}
