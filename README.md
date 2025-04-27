# Rust Microservices Architecture
A complete microservices ecosystem built with Rust, featuring high-performance web servers, load balancing, rate limiting, analytics, and comprehensive monitoring.

![Screenshot from 2025-04-28 01-01-39](https://github.com/user-attachments/assets/d387835d-f4c8-425b-b14f-2c90f6c1c5dc)


## Table of Contents
- System Overview
- Architecture
- Components
- Prerequisites
- Setup & Running
- API Documentation
- Development
- Monitoring
- Troubleshooting

## System Overview
This project demonstrates a complete microservices architecture using Rust with the following features:

- Multiple backend services with identical APIs
- Load balancing with nginx
- Rate limiting
- Analytics processing
- Prometheus & Grafana monitoring
- Containerized with Docker
- PostgreSQL database integration

## Architecture
> Directory structure:
```sh
.
├── .gitignore
├── analytics/              # Analytics service
├── docker-compose.yaml     # Main Docker Compose file
├── init-db/                # Database initialization scripts
├── load-balancer/          # Load balancing service
├── Makefile                # Main Makefile
├── monitoring/             # Prometheus and Grafana setup
│   ├── docker-compose.yml
│   ├── Makefile
│   ├── prometheus.yml
│   └── Setup.md
├── rate-limiter/           # Rate limiting service
├── Readme.md
├── server-1/               # First web server
│   ├── .dockerignore
│   ├── .env
│   ├── .gitignore
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── docker-compose.yml
│   ├── Dockerfile
│   ├── Makefile
│   ├── Setup.md
│   ├── src/
│   │   ├── main.rs
│   │   ├── services.rs
│   │   ├── header.rs
│   │   ├── kafka.rs
│   │   └── prom.rs
│   ├── target/
│   └── Testing.md
└── server-2/               # Second web server with identical API
```
> The system consists of multiple containerized services that communicate with each other:
```sh
                         ┌─────────────┐
                         │             │
                ┌────────▶   Monitor   │
                │        │             │
                │        └─────────────┘
                │
┌─────────┐     │        ┌─────────────┐         ┌─────────────┐
│         │     │        │             │         │             │
│ Client  ├─────┼───────▶│ LoadBalancer├────────▶│  Server 1   │
│         │     │        │             │         │             │
└─────────┘     │        └──────┬──────┘         └──────┬──────┘
                │               │                       │
                │               │                       │
                │        ┌──────▼──────┐         ┌──────▼──────┐
                │        │             │         │             │
                └────────┤ RateLimiter ├────────▶│  Server 2   │
                         │             │         │             │
                         └──────┬──────┘         └──────┬──────┘
                                │                       │
                                │                       │
                         ┌──────▼──────┐         ┌──────▼──────┐
                         │             │         │             │
                         │  Analytics  │◀────────┤  Database   │
                         │             │         │             │
                         └─────────────┘         └─────────────┘
```

## Components
### Server 1 & Server 2
Identical Rust servers built with Actix-web framework that handle the core business logic and expose the REST API. They connect to PostgreSQL for data persistence.

> Key Technologies:

- Actix-web framework
- SQLX for PostgreSQL connection
- Prometheus for metrics
- Kafka integration

### Load Balancer
A Rust-based service that distributes traffic between Server 1 and Server 2, ensuring high availability and performance.

> Key Features:

- Request routing
- Health checks
- Configurable through nginx.conf

### Rate Limiter
Controls the rate of requests to protect the system from being overwhelmed.

> Key Features:

- Request throttling
- Configurable limits
- Token bucket algorithm implementation

### Analytics
Processes data from other services and generates insights.

> Key Technologies:

- Kafka consumer/producer
- Data processing pipeline

### Monitoring
Provides comprehensive system monitoring using Prometheus and Grafana.

> Key Features:

- Real-time metrics
- Custom dashboards
- Alert configuration

## Prerequisites
- Docker and Docker Compose
- Rust (for development)
- Make utility
- PostgreSQL client (for schema setup)

## Setup & Running
### Database System Setup
To start all the databases and services:
```sh
make database
```

### Individual Component Setup (Open a terminal for each individual service and then run the following commands)
> Server 1 and Server 2

```sh
cd server-1
make
```
```sh
cd server-2
make
```

> Rate Limiter
```sh
cd rate-limiter
make
```

> Analytics
```sh
cd analytics
```

> Monitoring
```sh
cd monitoring
```
*Access Grafana dashboard*
*Open http://localhost:3000 in your browser*
*Default credentials: admin/admin*

## API Documentation

### Create a Todo Item
- URL: /todo
- Method: POST
- Request Body:
```json
{
  "title": "Finish the report",
  "description": "Complete the quarterly sales report",
  "status": "pending",
  "due_date": "2024-10-30"
}
```
- Response:
```json
{
  "id": 1,
  "title": "Finish the report",
  "description": "Complete the quarterly sales report",
  "status": "pending",
  "due_date": "2024-10-30",
  "created_at": "2024-10-23T13:34:15.763336",
  "updated_at": "2024-10-23T13:34:15.763336"
}
```

### Get All Todo Items
- URL: /todos
- Method: GET
- Response:
```json
[
  {
    "id": 1,
    "title": "Finish the report",
    "description": "Complete the quarterly sales report",
    "status": "pending",
    "due_date": "2024-10-30",
    "created_at": "2024-10-23T13:34:15.763336",
    "updated_at": "2024-10-23T13:34:15.763336"
  }
]
```

## Monitoring
### Metrics
The system exposes the following metrics via Prometheus:

- HTTP request counts
- Request duration
- Error rates
- System resource usage

### Grafana Dashboards
Access Grafana at `http://localhost:3000` and use the following dashboards:

1. System Overview - Core system metrics
2. API Performance - Request rates and latencies
3. Database Performance - Query performance and connection pools

### Adding Custom Metrics
To add new metrics:

1. Define the metric in the relevant service's `prom.rs` file
2. Register and expose the metric
3. Update the Prometheus scrape configuration if needed
4. Create or update Grafana dashboards

## Troubleshooting
### Common Issues
> Database Connection Errors
Error:
```sh
Database(PgDatabaseError { severity: Error, code: "42P01", message: "relation \"todo\" does not exist" })
```

Solution: Push the schema located in `todo.sql` to database:
```sh
psql -h localhost -U postgres -d postgres -f init-db/todo.sql
```

> Service Connection Issues
If services can't connect to each other:

1. Ensure necessary containers are running via:
```sh
docker-compose ps
```
2. Verify environment variables are correctly set in `.env` files

## Performance
> Tested using `oha`
> Command:
```sh
oha http://localhost:1234/todos -n 10000 -c 1000   
```
> Result:

![image](https://github.com/user-attachments/assets/92d5e2a8-6cd7-4f21-8348-ea6ef8673324)

