use lambda_runtime::{run, LambdaEvent, service_fn};
use shared::*;

mod harvester;
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
            let messages = harvester::harvest(dataset).await?;
            queue::process_messages(messages).await?;
        }
        Action::Sync(dataset) => {
            println!("Syncing dataset: {:?}", dataset);
        }
    }
    Ok(())
}