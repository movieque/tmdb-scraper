## Local Development and Testing Tips

### Local Testing
For local development and testing without deploying to AWS:
- Use LocalStack to emulate AWS services
- Create mock TMDB API responses for testing
- Implement feature flags to control behavior during testing

### Optimization Tips
- Profile your Rust code to identify bottlenecks
- Use asynchronous processing where appropriate
- Optimize Protocol Buffer schemas for size and performance
- Consider compression for larger data objects

### Avoiding Common Pitfalls
- Be careful with TMDB API rate limits - they are strictly enforced
- Plan for data schema evolution as TMDB may change their API
- Implement robust error handling for network issues
- Consider regional availability and latency if deploying globally## Implementation Checklist

Below is a comprehensive list of tasks to complete for this project. Each task includes an explanation and subtasks where appropriate:

### 1. Project Setup
- [ ] **Create basic project structure**
  - Create repositories, directories, and initial Cargo.toml
  - Set up your Rust development environment
  - Initialize git repository
- [ ] **Set up Protocol Buffer schemas**
  - Create schema files for each data type (movies, TV shows, people)
  - Define message structures matching TMDB API responses
  - Set up protoc compilation process

### 2. AWS Infrastructure Setup
- [ ] **Define AWS resources**
  - Create an infrastructure-as-code template (SAM or Terraform)
  - Define IAM roles with proper permissions
  - Configure SQS queues with appropriate settings
  - Set up S3 buckets with lifecycle policies
  - Configure EventBridge schedule
- [ ] **Set up local development environment for AWS**
  - Configure AWS CLI credentials
  - Set up local testing for Lambda functions

### 3. Harvester Lambda Implementation
- [ ] **Implement TMDB API client**
  - Create authentication and request logic
  - Handle rate limiting and retries
  - Set up endpoint interactions
- [ ] **Develop ID harvesting logic**
  - Fetch daily export files or changes from TMDB
  - Extract and filter relevant IDs
  - Implement pagination for large datasets
- [ ] **Implement SQS integration**
  - Send IDs to queue with appropriate batching
  - Add metadata for processing
  - Implement error handling and retry logic

### 4. ID's Holder Queue Configuration
- [ ] **Configure SQS queue settings**
  - Set visibility timeout appropriate for processing time
  - Configure dead-letter queue for failed messages
  - Set up redrive policy
  - Optimize batch size settings
- [ ] **Implement monitoring**
  - Set up CloudWatch alarms for queue depth
  - Configure metrics for queue performance

### 5. Collector Lambda Implementation
- [ ] **Implement detailed TMDB API fetching**
  - Create functions for each media type (movies, TV shows, people)
  - Include all required parameters and options
  - Handle nested data relationships
- [ ] **Develop Protocol Buffer serialization**
  - Convert JSON responses to protobuf format
  - Implement efficient binary serialization
  - Handle optional fields and defaults
- [ ] **Implement S3 storage logic**
  - Create efficient object naming conventions
  - Organize data by type and other attributes
  - Implement data versioning if needed
- [ ] **Manage concurrency and rate limits**
  - Implement backoff strategies
  - Track and respect TMDB API rate limits
  - Optimize for performance while staying within limits

### 6. Storage Organization
- [ ] **Design object hierarchy**
  - Create naming conventions for S3 objects
  - Optimize for both storage and retrieval efficiency
  - Plan for future growth
- [ ] **Implement data indexing**
  - Create index files or metadata objects
  - Enable efficient querying and filtering
  - Track data versions

### 7. Monitoring and Logging
- [ ] **Set up CloudWatch logging**
  - Configure structured logging for Lambda functions
  - Set appropriate log retention periods
  - Create log filters for important events
- [ ] **Implement monitoring**
  - Create dashboards for system health
  - Set up alerts for failures or anomalies
  - Track API usage and quota consumption

### 8. Testing
- [ ] **Write unit tests**
  - Test each component in isolation
  - Mock external services
  - Cover error handling and edge cases
- [ ] **Perform integration testing**
  - Test the entire workflow
  - Verify data correctness
  - Measure performance
- [ ] **Load testing**
  - Verify system handles expected data volumes
  - Test with rate limit constraints
  - Ensure recovery from failures

### 9. Documentation and Maintenance
- [ ] **Complete documentation**
  - Update README with final details
  - Document API usage and configuration options
  - Create deployment guides
- [ ] **Create maintenance procedures**
  - Document backup and recovery procedures
  - Create update and migration process
  - Plan for TMDB API changes## Advanced Usage

See the [Advanced Usage Guide](docs/advanced-usage.md) for more complex scenarios including:
- Collecting specific media types
- Setting custom date ranges
- Implementing differential updates
- Optimizing for large data sets
- Working with Protocol Buffer schemas# TMDB Data Collector

![TMDB Data Collector Architecture](https://path-to-your-architecture-diagram.png)

A robust, scalable Rust-based system for harvesting and storing data from The Movie Database (TMDB) API.

A robust, scalable Rust-based system for harvesting and storing data from The Movie Database (TMDB) API. This project efficiently collects information about movies, TV shows, seasons, episodes, people, and more through a series of serverless functions and queues, built with performance and reliability in mind.

## Table of Contents
- [Overview](#overview)
- [Architecture](#architecture)
- [Components](#components)
- [Setup and Installation](#setup-and-installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Implementation Checklist](#implementation-checklist)
- [API Reference](#api-reference)
- [Performance Considerations](#performance-considerations)
- [Local Development and Testing Tips](#local-development-and-testing-tips)
- [Monitoring and Logging](#monitoring-and-logging)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [License](#license)

## Overview

TMDB Data Collector is designed to systematically harvest data from TMDB's API, process it, and store it efficiently. The system leverages AWS serverless architecture to provide a scalable, maintainable solution that:

- Collects daily ID exports or changes from TMDB
- Processes IDs through a controlled queue system
- Retrieves detailed information for each media type
- Stores data in an organized object storage system
- Operates within TMDB's rate limits to avoid overloading their servers

Whether you need to maintain a complete mirror of TMDB's database or just collect specific data points, this system provides the infrastructure to do so reliably and efficiently.

## Architecture

The system consists of five main components working together:

1. **Scheduler** (Amazon EventBridge) - Triggers the data collection process at configured intervals
2. **Harvester** (Lambda Function) - Collects IDs from TMDB's daily exports or change feeds
3. **ID's Holder** (SQS Queue) - Stores and distributes IDs to be processed
4. **Collector** (Lambda Function) - Retrieves detailed information for each ID from TMDB API
5. **Storage** (S3 Bucket) - Stores the collected data in an organized, queryable format

Data flows through the system as follows:
- The Scheduler triggers the Harvester at defined intervals
- The Harvester fetches new or changed IDs from TMDB and sends them to ID's Holder
- The ID's Holder queue triggers the Collector functions concurrently but controlled
- Each Collector function fetches complete records from TMDB API for its assigned IDs
- Collected data is stored in Storage with appropriate organization by media type

## Components

### Scheduler (Amazon EventBridge)

- Triggers the initial data collection and subsequent updates
- Configurable scheduling patterns (daily, hourly, etc.)
- Passes parameters to define collection scope and behavior

### Harvester (Lambda Function)

- Connects to TMDB's daily export files or change feeds
- Extracts relevant IDs for movies, TV shows, people, etc.
- Handles pagination and rate limiting for large data sets
- Pushes extracted IDs to the ID's Holder queue
- Tracks already processed IDs to avoid duplication

### ID's Holder (SQS Queue)

- Maintains a queue of IDs to be processed
- Provides durability to ensure no IDs are lost
- Enables controlled concurrent processing
- Implements visibility timeout and dead-letter queue for error handling

### Collector (Lambda Function)

- Receives batches of IDs from the queue
- Makes authenticated requests to TMDB API for detailed information
- Processes and transforms the API responses as needed
- Respects TMDB's rate limits to avoid overloading their servers
- Writes collected data to appropriate storage locations

### Storage (S3 Bucket)

- Organizes data by media type (movies, TV shows, seasons, episodes, people)
- Uses efficient object naming conventions for easy retrieval
- Implements Protocol Buffers (protobuf) for serialization, providing:
  - Compact binary format for storage efficiency
  - Structured schema-based approach
  - Platform/language neutrality for future interoperability
  - Significantly reduced storage costs compared to JSON
- Enables data versioning and change tracking
- Optimizes for both storage efficiency and query performance

## Setup and Installation

### Prerequisites

- AWS Account with permissions to create:
  - Lambda Functions
  - SQS Queues
  - EventBridge Rules
  - S3 Buckets
  - IAM Roles and Policies
- TMDB API Key ([obtain here](https://www.themoviedb.org/documentation/api))
- Rust toolchain (rustc, cargo) for local development
- Protocol Buffers compiler (protoc) for schema compilation
- AWS CLI and AWS SAM or Terraform for deployment

### Deployment Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/tmdb-data-collector.git
   cd tmdb-data-collector
   ```

2. Build the Rust Lambda functions:
   ```bash
   # First compile the protobuf schemas
   protoc --rust_out=src/protos/ schemas/*.proto
   
   # Then build the lambda functions
   cargo build --release
   ```

3. Configure your TMDB API key and AWS settings:
   ```bash
   cp .env.example .env
   # Edit .env with your settings
   ```

4. Deploy the stack (using AWS SAM):
   ```bash
   sam build
   sam deploy --guided
   ```
   
   Or if using Terraform:
   ```bash
   terraform init
   terraform apply
   ```

5. Verify the deployment:
   ```bash
   cargo run --bin verify-deployment
   ```

## Configuration

### Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `TMDB_API_KEY` | Your TMDB API authentication key | - | Yes |
| `COLLECTION_INTERVAL` | How often to check for updates (in minutes) | `1440` (daily) | No |
| `COLLECTOR_CONCURRENCY` | Maximum concurrent collector functions | `5` | No |
| `MEDIA_TYPES` | Types of media to collect (comma-separated) | `movie,tv,person` | No |
| `S3_BUCKET_NAME` | Name of the S3 bucket for storage | - | Yes |
| `RATE_LIMIT_REQUESTS` | Maximum requests per second to TMDB API | `3` | No |

### Configuration Files

- `template.yaml` or `main.tf` - Main infrastructure configuration (SAM or Terraform)
- `config/collector.toml` - Collector function settings
- `config/harvester.toml` - Harvester function settings
- `config/storage.toml` - Storage organization settings

## Usage

### Starting a Collection

The system will automatically run based on the configured schedule. To manually trigger a collection:

```bash
aws events put-events --entries file://samples/trigger-event.json
```

### Monitoring Progress

Track the progress of collection jobs:

```bash
cargo run --bin status-checker
```

### Querying Collected Data

Access collected data directly from S3 or use the provided utilities:

```bash
cargo run --bin query-tool -- --type=movie --id=550
```



## API Reference

### TMDB API Integration

This project uses the following TMDB API endpoints:

- [Daily ID Exports](https://developers.themoviedb.org/3/getting-started/daily-file-exports)
- [Changes](https://developers.themoviedb.org/3/changes/get-movie-change-list)
- [Movies](https://developers.themoviedb.org/3/movies/get-movie-details)
- [TV Shows](https://developers.themoviedb.org/3/tv/get-tv-details)
- [People](https://developers.themoviedb.org/3/people/get-person-details)

For complete documentation, see [TMDB API Documentation](https://developers.themoviedb.org/3/getting-started/introduction).

### Internal Functions

Documentation for key functions can be found in the [API Reference](docs/api-reference.md).

## Performance Considerations

The system is designed to balance performance with API rate limit compliance, leveraging Rust's speed and efficiency:

- SQS queue manages the processing rate to stay within TMDB's limits
- Collector functions implement exponential backoff for rate limit handling
- Batch processing optimizes Lambda execution time
- Object storage pattern minimizes read/write operations
- Dead letter queues capture and allow replay of failed operations
- Rust's zero-cost abstractions and memory safety for optimal performance
- Asynchronous processing using Rust's async/await capabilities

For large-scale collections, consider:
- Increasing Lambda memory allocations for better performance
- Adjusting concurrency settings based on your TMDB API tier
- Implementing data compression for storage efficiency
- Using reserved concurrency to prevent resource starvation
- Tuning Rust compiler optimizations for smaller binary sizes

## Monitoring and Logging

The system provides comprehensive monitoring and logging:

- CloudWatch Logs for all Lambda functions
- CloudWatch Metrics for queue depths, function durations, and error rates
- Custom metrics for TMDB API usage and rate limiting
- Alerting for anomalies and failures

To set up alerts:

```bash
cargo run --bin setup-alerts
```

## Troubleshooting

### Common Issues

| Issue | Possible Cause | Solution |
|-------|----------------|----------|
| Harvester function timing out | Too many IDs to process | Increase Lambda timeout or implement pagination |
| Rate limit exceeded errors | Too high concurrency | Reduce `COLLECTOR_CONCURRENCY` setting |
| Missing data | API changes or errors | Check CloudWatch logs and TMDB API status |
| High latency | Insufficient Lambda resources | Increase Lambda memory allocation |

### Debug Mode

Enable detailed logging:

```bash
# For SAM
sam deploy --parameter-overrides "Stage=dev Debug=true"

# For Terraform
terraform apply -var="stage=dev" -var="debug=true"
```

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Acknowledgements

- [TMDB](https://www.themoviedb.org/) for providing their excellent API
- The AWS Serverless community for inspiration and best practices
- All contributors who help improve this project

---

*Note: This project is not affiliated with or endorsed by TMDB.*
