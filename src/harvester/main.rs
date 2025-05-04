use lambda_runtime::{run, LambdaEvent, service_fn};
use shared::*;


#[tokio::main]
async fn main() -> Result<()> {
    let handler = service_fn(handler);
    Ok(run(handler).await?)
}


async fn handler(event: LambdaEvent<Action>) -> Result<()> {
    println!("Received event: {:?}", event);
    let action = event.payload;
    match action {
        Action::Export(dataset) => {
            println!("Exporting dataset: {:?}", dataset);
            // Call the export function here
        }
        Action::Sync(dataset) => {
            println!("Syncing dataset: {:?}", dataset);
            // Call the sync function here
        }
    }
    Ok(())
}