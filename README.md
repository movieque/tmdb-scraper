# TMDB Data Pipeline

A Rust-based serverless data pipeline for efficiently collecting and distributing [TMDB](https://www.themoviedb.org/) data using AWS Lambda, SQS, and Bunny Edge Storage.

## Architecture Overview

This project implements a serverless data pipeline consisting of two main components:

1. **Harvester Lambda**: Triggered on schedule via EventBridge to either:
   - Process TMDB daily ID exports (export mode)
   - Check for updates since the last run (sync mode)
   - Organize IDs into batches and push to SQS

2. **Collector Lambda**: Triggered by SQS messages to:
   - Process batches of up to 50 IDs
   - Fetch detailed data from TMDB API in parallel
   - Convert JSON responses to Protobuf format
   - Store compressed data in Bunny Edge Storage

By storing the data directly in Edge Storage as Protobuf files, this solution:
- Eliminates traditional API server costs
- Delivers low-latency data access globally
- Reduces storage costs through efficient compression
- Maintains a completely serverless architecture



## Workflow

1. **Scheduled Trigger**:
   - EventBridge schedule triggers the Harvester Lambda
   - Includes parameters for dataset type and mode (export/sync)

2. **Harvester Function**:
   - In export mode: Downloads and processes daily ID exports from TMDB
   - In sync mode: Queries for updates since the last run
   - Sorts IDs and organizes them into batches of 50
   - Pushes batches to SQS with metadata (mode, dataset type)

3. **SQS Queue**:
   - Holds batched ID messages
   - Triggers Collector Lambda for each message

4. **Collector Function**:
   - Processes one SQS message (containing up to 50 IDs)
   - Makes parallel requests to TMDB API for detailed information
   - Uses HTTP pipelining/async requests for efficiency
   - Converts JSON responses to Protobuf format
   - Stores data in Bunny Edge Storage asynchronously

5. **Data Access**:
   - Clients access data directly from Edge Storage
   - Low latency due to edge distribution
   - No API server required

## Features

- **Compile-time Feature Flags**:
  - `harvester`: Builds only the Harvester Lambda
  - `collector`: Builds only the Collector Lambda

- **Operational Modes**:
  - `export`: Full dataset collection using TMDB exports
  - `sync`: Incremental updates since last collection

- **Performance Optimizations**:
  - Parallel processing of TMDB API requests
  - Asynchronous storage operations
  - Batched ID processing
  - Protobuf compression for storage efficiency

## Prerequisites

- Rust 1.65+
- AWS Account with access to Lambda, SQS, EventBridge
- Bunny Edge Storage account
- TMDB API access

## Setup

1. **AWS Resources**:
   ```bash
   # Create required AWS resources (Lambda, SQS, EventBridge)
   aws cloudformation deploy --template-file infrastructure/template.yaml --stack-name tmdb-pipeline
   ```

2. **Environment Variables**:
   ```
   TMDB_API_KEY=your_api_key
   AWS_REGION=your_region
   SQS_QUEUE_URL=your_queue_url
   BUNNY_STORAGE_API_KEY=your_bunny_key
   BUNNY_STORAGE_ENDPOINT=your_bunny_endpoint
   ```

3. **Build**:
   ```bash
   # Build the Harvester Lambda
   cargo build --release --features harvester
   
   # Build the Collector Lambda
   cargo build --release --features collector
   ```

4. **Deploy**:
   ```bash
   # Deploy both Lambdas
   make deploy
   ```

## Development

### Building Locally

```bash
# Build both functions
cargo build --all-features

# Build only the Harvester
cargo build --features harvester

# Build only the Collector
cargo build --features collector
```

### Testing

```bash
cargo test
```

### Local Execution

```bash
# Test Harvester with export mode and movie dataset
cargo run --features harvester -- --mode export --dataset movie

# Test Collector with a sample SQS message
cargo run --features collector -- --input-file test/sample_sqs_message.json
```

## Deployment

The project includes a Makefile for simple deployment:

```bash
# Deploy everything
make deploy

# Deploy only Harvester
make deploy-harvester

# Deploy only Collector
make deploy-collector
```

## Monitoring and Maintenance

- CloudWatch Logs for function execution logs
- CloudWatch Metrics for performance monitoring
- SQS Dead Letter Queue for failed processing

## License

[MIT](LICENSE)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.