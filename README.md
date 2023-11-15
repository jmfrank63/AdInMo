# AdInMo Project

## Overview

AdInMo is a Rust-based project consisting of two main components: an Actix web server (`server`) and
a Hyper service (`service`). The project demonstrates the use of Rust for building web services,
along with Docker for containerization and inter-service communication.

### Components

- **Actix Server (`server`)**: A web server built using Actix that handles
incomingGET requests and forwards them to the Hyper service.
- **Hyper Service (`service`)**: A service built using Hyper that receives
requests from the Actix server and performs predefined operations (currently
set to return a simple success message).

## Getting Started

### Prerequisites

- Rust (1.73 or later)
- Docker
- `just` command runner

### Installation

1. **Clone the Repository**:

    ``` bash
    git clone [repository-url]
    cd AdInMo
    ```

2. **Build the Docker Images**:

    ``` bash
    just build
    ```

3. **Create a Docker Network**:

    ``` bash
    docker network create adinmo-network
    ```

4. **Run the Containers**:

    ``` bash
    just run
    ```

### Usage

- The Actix server is accessible at `http://localhost:3000/run`.
- The Hyper service runs internally within the Docker network and is
accessible to the Actix server at `http://service:5000`.

## Development

### Actix Server

- Located in the `actix_server` directory.
- Contains the Rust code for the Actix web server.

### Hyper Service

- Located in the `hyper_service` directory.
- Contains the Rust code for the Hyper service.

## Dockerization

- Each component is containerized using Docker.
- Dockerfiles are provided in the respective directories of each component.
- Containers communicate over a dedicated Docker network.

## Justfile

- A `justfile` is included for simplifying build and run processes.
- Use `just build` to build Docker images.
- Use `just run` to run the containers.

## Contributing

Contributions to the AdInMo project are welcome. Please follow the standard
procedures for contributing to a GitHub repository.

## License

Specify the license under which the project is available.

---

Replace `[repository-url]` with the actual URL of your Git repository. You
may also want to add more sections as needed, such as for testing,
environment setup, or advanced usage.
