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