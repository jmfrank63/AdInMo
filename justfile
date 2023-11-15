# Define a command to build all Docker images

build:
    #!/usr/bin/env bash
    (cd actix_server && docker build -t jmfrank63/actix_server .)
    (cd hyper_service && docker build -t jmfrank63/hyper_service .)

# Define a command to run both Docker containers
run:
    #!/usr/bin/env bash
    docker network create adinmo-network || true
    docker stop server || true
    docker stop service || true
    docker rm server || true
    docker rm service || true
    docker run -d -p 3000:3000 --network adinmo-network --name server jmfrank63/actix_server
    docker run -d -p 5000:5000 --network adinmo-network --name service jmfrank63/hyper_service
