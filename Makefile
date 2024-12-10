clean:
	sudo docker rm -v -f $(sudo docker ps -qa)
dev:
	sudo docker-compose up
database:
	sudo docker-compose --profile database up
kill:
	sudo kill -9 $(sudo lsof -t -i :8888) 