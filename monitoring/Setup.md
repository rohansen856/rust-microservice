# 📊 Monitoring Setup (Prometheus + Grafana)

## Project Structure

- `docker-compose.yml`: Docker Compose configuration for Prometheus and Grafana services.
- `prometheus.yml`: Prometheus configuration file to scrape metrics.
- `Makefile`: Makefile to manage starting, stopping, and managing the services easily.

## Configuration

Update the scrape target settings inside `prometheus.yml` if necessary:
