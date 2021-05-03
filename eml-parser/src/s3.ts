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

export async function fetchFromS3(event: S3CreateEvent): Promise<S3Object> {
  const { eventVersion, s3 } = event.Records.pop()!;

  console.log(
    `Received an S3 event version ${eventVersion} of size ${s3.object.size}.`
  );

  const bucketName = s3.bucket.name;
  const objectName = decodeURI(s3.object.key); // a.k.a. email id

  const params: aws.S3.Types.GetObjectRequest = {
    Bucket: bucketName,
    Key: objectName,
  };
  const { Body } = await new aws.S3().getObject(params).promise();

  return {
    id: objectName,
    body: (Body || "").toString(),
  };
}

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
    // https://stackoverflow.com/a/2505733/5093093
    ContentType: "text/html; charset=UTF-8",
    StorageClass: "REDUCED_REDUNDANCY",
    Key: s3Key,
  };

  await new aws.S3().putObject(params).promise();
}
