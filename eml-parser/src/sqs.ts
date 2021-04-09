/**
 * Everything related to message passing via SQS is included in this module.
 *
 * # TODO
 * * [#10][issue-10] Allow multiple concurrent messages.
 *
 * [issue-10]: https://github.com/bausano/mailmevouchers.com/issues/10
 */

import { SQS as AwsSqs } from "aws-sdk";
import { S3CreateEvent } from "./s3";

export interface SqsConf {
  url: string;
}

export class SqsMessage {
  constructor(
    public receipt: string,
    private origin: Sqs,
    private stringifiedBody: string
  ) {
    //
  }

  public body(): S3CreateEvent {
    // TBD: should we validate the JSON?
    return JSON.parse(this.stringifiedBody);
  }

  /**
   * Deletes the message from the origin SQS. Call this method after message has
   * been successfully processed.
   *
   * If this method isn't called, message will be redelivered based on the SQS
   * settings.
   */
  public delete(): Promise<void> {
    return this.origin.delete(this.receipt);
  }
}

/**
 * Polymorphism used for tests.
 */
export interface SqsProvider {
  /**
   * Blocks until a message is available.
   */
  receive(): Promise<SqsMessage>;

  /**
   * Deletes a message. Call this after the email has been successfully
   * processed.
   *
   * @param receipt A receipt handle of the message
   */
  delete(receipt: string): Promise<void>;
}

export class Sqs implements SqsProvider {
  constructor(private inner: AwsSqs, private conf: SqsConf) {
    //
  }

  public static new(conf: SqsConf): Sqs {
    return new Sqs(new AwsSqs(), conf);
  }

  public async receive(): Promise<SqsMessage> {
    const params: AwsSqs.Types.ReceiveMessageRequest = {
      QueueUrl: this.conf.url,
      MaxNumberOfMessages: 1,
      WaitTimeSeconds: 20,
    };

    // https://github.com/bbc/sqs-consumer/issues/247
    console.log(`[${new Date()}] Waiting for message in sqs client...`);
    const { Messages } = await this.inner.receiveMessage(params).promise();

    if (!Messages || !Array.isArray(Messages) || Messages.length === 0) {
      console.log(`[${new Date()}] No messages arrived from sqs...`);
      // this shouldn't lead to stack overflow due to the wait time enforced
      // by SQS, but just to be sure we wait 1s here
      await new Promise((resolve) => setTimeout(resolve, 1000));
      return this.receive();
    }

    const { ReceiptHandle, Body } = Messages.pop()!;

    if (!ReceiptHandle || !Body) {
      throw new Error(`Invalid message received: ${ReceiptHandle}: ${Body}`);
    }

    return new SqsMessage(ReceiptHandle, this, Body);
  }

  public async delete(receipt: string) {
    await this.inner
      .deleteMessage({
        QueueUrl: this.conf.url,
        ReceiptHandle: receipt,
      })
      .promise();
  }
}
