clean:
	sudo docker rm -v -f $(sudo docker ps -qa)
dev:
	sudo docker-compose up 