# AdInMo Project

## Overview

AdInMo is a Rust-based project consisting of two main components: an Actix web server (`server`) and
a Hyper service (`service`). The project demonstrates the use of Rust for building web services,
along with Docker for containerization and inter-service communication.

### Components

-   **Actix Server (`server`)**: A web server built using Actix that handles
    incomingGET requests and forwards them to the Hyper service.
-   **Hyper Service (`service`)**: A service built using Hyper that receives
    requests from the Actix server and performs predefined operations (currently
    set to return a simple success message).

## Getting Started

### Prerequisites

-   Rust (1.73 or later)
-   Docker
-   `just` command runner

### Installation

1. **Clone the Repository**:

    ```bash
    git clone https://github.com/jmfrank63/AdInMo.git
    cd AdInMo
    ```

2. **Build the Docker Images**:

    ```bash
    just build
    ```

3. **Create a Docker Network**:

    ```bash
    docker network create adinmo-network
    ```

4. **Run the Containers**:

    ```bash
    just run
    ```

### Usage

-   The Actix server is accessible at `http://127.0.0.1:3300/run`.
-   The Hyper service runs internally within the Docker network and is
    accessible to the Actix server at `http://service:5500`.

## Development

### Actix Server

-   Located in the `server` directory.
-   Contains the Rust code for the Actix web server.

### Hyper Service

-   Located in the `service` directory.
-   Contains the Rust code for the Hyper service.

## Dockerization

-   Each component is containerized using Docker.
-   Dockerfiles are provided in the respective directories of each component.
-   Containers communicate over a dedicated Docker network.

## Justfile

-   A `justfile` is included for simplifying build and run processes.
-   Use `just build` to build Docker images.
-   Use `just run` to run the containers.

## Contributing

Contributions to the AdInMo project are welcome. Please follow the standard
procedures for contributing to a GitHub repository.

## License

This project is licensed under the MIT License - see the LICENSE.md file for details.
