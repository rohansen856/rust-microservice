PROJECT_NAME = monitoring

up:
	@echo "🚀 Starting Prometheus and Grafana..."
	docker-compose up

prometheus-shell:
	@echo "🔧 Opening a shell inside Prometheus container..."
	docker exec -it prometheus /bin/sh

grafana-shell:
	@echo "🔧 Opening a shell inside Grafana container..."
	docker exec -it grafana /bin/bash

help:
	@echo ""
	@echo "✨ Available commands:"
	@echo "  make up              # Start Prometheus and Grafana"
	@echo "  make prometheus-shell# Shell into Prometheus container"
	@echo "  make grafana-shell   # Shell into Grafana container"
	@echo "  make help            # Show this help"
	@echo ""
