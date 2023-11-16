set dotenv-load

# Define a command to build all Docker images
build:
    #!/usr/bin/env bash
    (cd actix_server && docker build -t jmfrank63/actix_server .)
    (cd hyper_service && docker build -t jmfrank63/hyper_service .)
    (cd mariadb_database && docker build -t jmfrank63/mariadb_database .)

# Define a command to run both Docker containers
run:
    #!/usr/bin/env bash
    server_addr_port=${SERVER_ADDR_PORT:-"0.0.0.0:3300"}
    service_addr_port=${SERVICE_ADDR_PORT:-"0.0.0.0:5500"}
    database_addr_port=${DATABASE_ADDR_PORT:-"0.0.0.0:3306"}
    docker network create adinmo-network || true
    docker stop server || true
    docker stop service || true
    docker stop database || true
    docker rm server || true
    docker rm service || true
    docker rm database || true
    docker run -d -p ${database_addr_port}:3306 --network adinmo-network --name database jmfrank63/mariadb_database
    docker run -d -p ${service_addr_port}:5500 --network adinmo-network --name service jmfrank63/hyper_service
    docker run -d -p ${server_addr_port}:3300 --env MARIADB_ROOT_PASSWORD=uuvv --network adinmo-network --name server jmfrank63/actix_server
