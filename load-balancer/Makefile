dev:
	cargo run
run:
	sudo docker build -t load_balancer . && sudo docker run --rm -it -p 1234:1234 load_balancer
nginx:
	sudo docker run --name nginx-load-balancer -p 1234:1234 \
	-v ./nginx.conf:/etc/nginx/nginx.conf:ro \
	nginx
