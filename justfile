set dotenv-load

# Define a command to build all Docker images
build:
    #!/usr/bin/env bash
    (cd actix_server && docker build -t jmfrank63/actix_server .)
    (cd hyper_service && docker build -t jmfrank63/hyper_service .)

# Define a command to run both Docker containers
run:
    #!/usr/bin/env bash
    server_addr_port=${SERVER_ADDR_PORT:-"0.0.0.0:3300"}
    IFS=':' read -ra ADDR <<< "$server_addr_port"
    server_port=${ADDR[1]}
    service_addr_port=${SERVICE_ADDR_PORT:-"0.0.0.0:5500"}
    IFS=':' read -ra ADDR <<< "$service_addr_port"
    service_port=${ADDR[1]}
    docker network create adinmo-network || true
    docker stop server || true
    docker stop service || true
    docker rm server || true
    docker rm service || true
    docker run -d -p ${server_addr_port}:3300 --network adinmo-network --name server jmfrank63/actix_server
    docker run -d -p ${service_addr_port}:5500 --network adinmo-network --name service jmfrank63/hyper_service
