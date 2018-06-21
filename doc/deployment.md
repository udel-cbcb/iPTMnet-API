[Home](/README.md) 


## Deployment - with docker

### Deployment with postgres backend

1. Start postgres docker container on port `5432` with user `iptmnet_user` and password `iptmnet_password`

	```
	sudo docker run --name postgres_iptmnet \
			-e POSTGRES_USER=iptmnet_user \
			-e POSTGRES_PASSWORD=iptment_password \
			-e POSTGRES_DB=iptmnet \
			-v iptmnet-vol:/var/lib/postgresql/data \
			-p 0.0.0.0:5432:5432 \
			-d postgres:10.3-alpine
	```

2. Import the data in postgres

	```
	# download the exported data
	wget http:://path/to/exported_data.zip

	# download the data import utility
	wget http://path/to/data_importer

	# change to executable permission
	chmod a+x data_importer

	# import the data
	./data_importer --host=localhost --port=5432 --user=iptmnet_user --pass=iptmnet_password 

	```

3. Create a `config.ini` file

	```
	[DEFAULT]
	driver = postgres

	[POSTGRES]
	host = postgres_iptmnet
	port = 5432
	database-name = iptmnet
	user = iptmnet_user
	password = iptmnet_password
	```

4. Start iptmnet docker container on port `8082`

	```
	sudo docker run --name iptmnet_api \
						-v $PWD/config.ini:/home/config.ini:ro \
						--link postgres_iptmnet:postgres_iptmnet \
						-p 8082:8088 \
						--restart always \
						-d \
						udelcbcb/iptmnet_api:1.1.3
	```

5. The api server will be available at `http://your-server:8082`

6. Configure a reverse proxy using nginx or apache according to your requirements.

### Deployment with oracle backend
1. Create a `config.ini` file

	```
	[DEFAULT]
	driver = oracle

	[POSTGRES]
	host = postgres_database_address
	port = 1234
	database-name = db_name
	user = username
	password = password
	```

2. Start iptmnet docker container on port `8082`

	```
	sudo docker run --name iptmnet_api \
						-v $PWD/config.ini:/home/config.ini:ro \
						-p 8082:8088 \
						--restart always \
						-d \
						udelcbcb/iptmnet_api:1.1.3
	```

3. The api server will be available at `http://your-server:8082`

4. Configure a reverse proxy using nginx or apache according to your requirements.

## Deployment - without docker

### Oracle backend
1. Create a `config.ini` file

	```
	[DEFAULT]
	driver = oracle

	[ORACLE]
	host = oracle_address
	port = oracle_port
	database-name = iptmnet
	user = oracle_user
	password = oracle_password
	```

2. Start the iptmnet server

	```
	# download the iptmnet server
	wget http://path/to/iptmnet_api

	# change to executabe permissions
	chmod a+x iptmnet_api

	# start the server
	./iptmnet_api
	```

3. The api server will be available at `http://your-server:8088`

4. Configure a reverse proxy using nginx or apache according to your requirements.