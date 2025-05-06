use aws_sdk_sqs::types::SendMessageBatchRequestEntry;
use aws_config::{load_defaults, BehaviorVersion};
use futures::future::try_join_all;
use static_init::dynamic;
use aws_sdk_sqs::Client;
use super::error::Error;
use shared::*;

#[dynamic]
static QUEUE_URL: String = std::env::var("QUEUE_URL").expect("QUEUE_URL not set");



pub async fn process_messages(messages: Vec<Message>) -> Result<()> {
    let config = load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);
    let mut messages = messages.into_iter().peekable();
    let mut futures = Vec::new();
    while let Some(_) = messages.peek() {
        // create the futures into chunks of maximum 500 to create a limit on how many requests are sent to the sqs queue.
        let mut iter = Vec::new();
        for _ in 0..500 {
            // create a batch of 10 messages for single http request on each batch.
            let mut batch = Vec::<Message>::new();
            for _ in 0..10 {
                if let Some(message) = messages.next() {
                    batch.push(message);
                }
            }
            if !batch.is_empty() {
                iter.push(queue(batch, &client));
            }
        }
        if !iter.is_empty() {
            futures.push(iter)
        }
    }
    for iter in futures {
        try_join_all(iter).await?;
    }
    Ok(())
}



async fn queue(batch: Vec<Message>, client: &Client) -> Result<()> {
    let url = queue_url();
    let mut entries = Vec::<SendMessageBatchRequestEntry>::new();
    for message in batch {
        let message = message.try_into()?;
        entries.push(message);
    }
    let output = client.send_message_batch().queue_url(url).set_entries(Some(entries)).send().await?;
    if !output.failed().is_empty() {
        return Err(Error::BatchResultErrorEntry(output.failed).into());
    }
    Ok(())
}


fn queue_url() -> &'static str {
    QUEUE_URL.as_str()
}