# Rust Kafka

# Rust Kafka Integration with Docker

This project demonstrates a simple integration of Apache Kafka with Rust using the `rdkafka` crate. It includes examples of a Kafka consumer and producer, showcasing how to send and receive messages. Kafka and Zookeeper are run using Docker for ease of setup and management.

## Features

- **Kafka Consumer**: Consumes messages from a Kafka topic and processes them.
- **Kafka Producer**: Sends messages to a Kafka topic.
- **Async Processing**: Utilizes Rust's async capabilities for non-blocking message handling.
- **Dockerized Kafka and Zookeeper**: Uses Docker to simplify the setup and management of Kafka and Zookeeper.

## Prerequisites

- Rust (latest stable version recommended)
- Docker and Docker Compose
- Cargo (Rust's package manager)

## Getting Started

### Step 1: Clone the Repository

```bash
git clone https://github.com/chinmayvivek/rust-kafka.git
cd rust-kafka
```



### Step 2: Setup Kafka and Zookeeper using Docker

1. **Ensure Docker is installed and running on your system**.
2. **Start Kafka and Zookeeper**:

   ```bash
   docker-compose up
   ```

   This command will pull the necessary Docker images and start the containers for Kafka and Zookeeper as defined in the `docker-compose.yml` file.

### Step 3: Run the project

```bash
cargo run cargo run --quiet
```


## Project Structure

- `src/consumer.rs`: Contains the implementation of the Kafka consumer.
- `src/producer.rs`: Contains the implementation of the Kafka producer.
- `docker-compose.yml`: Docker Compose configuration file for Kafka and Zookeeper.

## Configuration

Update the Kafka settings in the `src/consumer.rs` and `src/producer.rs` files if necessary:

```rust
// Consumer
.set("bootstrap.servers", "localhost:9092")
.set("group.id", "test-group")
.set("auto.offset.reset", "earliest")

// Producer
.set("bootstrap.servers", "localhost:9092")
```


## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Apache Kafka](https://kafka.apache.org/)
- [Rust rdkafka crate](https://docs.rs/rdkafka/)
- [Docker](https://www.docker.com/)

Feel free to open issues or pull requests if you have any suggestions or improvements!
