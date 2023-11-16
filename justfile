set dotenv-load

# Define a command to build all Docker images
build:
    #!/usr/bin/env bash
    docker build -t jmfrank63/server ./server
    docker build -t jmfrank63/service -f service/Dockerfile .
    docker build -t jmfrank63/mariadb ./mariadb
    docker build -t jmfrank63/database ./database

# Define a command to run both Docker containers
run:
    #!/usr/bin/env bash
    server_addr_port=${SERVER_ADDR_PORT:-"0.0.0.0:3300"}
    service_addr_port=${SERVICE_ADDR_PORT:-"0.0.0.0:5500"}
    database_addr_port=${DATABASE_ADDR_PORT:-"0.0.0.0:3306"}
    docker network create adinmo-network || true
    docker stop server || true
    docker stop service || true
    docker stop mariadb || true
    docker rm server || true
    docker rm service || true
    docker rm mariadb || true
    docker run -d -p ${database_addr_port}:3306 --env MARIADB_ROOT_PASSWORD=uuvv --network adinmo-network --name mariadb jmfrank63/mariadb
    docker run -d -p ${service_addr_port}:5500 --network adinmo-network --name service jmfrank63/service
    docker run -d -p ${server_addr_port}:3300 --network adinmo-network --name server jmfrank63/server
