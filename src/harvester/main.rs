use lambda_runtime::{run, LambdaEvent, service_fn};
use shared::*;

mod exporter;
mod syncer;
mod queue;
mod error;


#[tokio::main]
async fn main() -> Result<()> {
    let handler = service_fn(handler);
    Ok(run(handler).await?)
}


async fn handler(event: LambdaEvent<Action>) -> Result<()> {
    let action = event.payload;
    match action {
        Action::Export(dataset) => {
            let ids = exporter::harvest_export(dataset).await?.into_iter();
            let messages = create_messages(dataset, ids);
            queue::process_messages(messages).await?;
        }
        Action::Sync(dataset, days_interval) => {
            let ids = syncer::harvest_sync(dataset, days_interval).await?.into_iter();
            let messages = create_messages(dataset, ids);
            queue::process_messages(messages).await?;
        }
    }
    Ok(())
}


fn create_messages(dataset: Dataset, ids: impl Iterator<Item = u32>) -> Vec<Message> {
    let mut iter = ids.peekable();
    let mut messages = Vec::<Message>::new();
    while let Some(_) = iter.peek() {
        let mut ids = Vec::<u32>::new();
        for _ in 0..50 {
            if let Some(id) = iter.next() {
                ids.push(id);
            }
        }
        if !ids.is_empty() {
            let message = Message{dataset, ids};
            messages.push(message);
        }
    }
    messages
}