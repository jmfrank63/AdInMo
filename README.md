# AdInMo Project

## Overview

AdInMo is a Rust-based project consisting of three main components: an Actix web server (`server`) and
a Hyper service (`service`) and the database (`mariadb`). A test container (`client`) is provided for
the basic API calls. The project demonstrates the use of Rust for building web services,
along with Docker for containerization, inter-service communication and database access.

### Components

- **Actix Server (`server`)**: A web server built using Actix that handles
    incomingGET requests and forwards them to the Hyper service.
- **Hyper Service (`service`)**: A service built using Hyper that receives
    requests from the Actix server and performs predefined operations (currently
    set to return a simple success message).
- **Database (`mariadb`)**: A MariaDB database that stores the data for the
    Hyper service.
- **Test Client (`client`)**: A test container that can be used to test the
    Actix server.

## Getting Started

### Prerequisites

- Rust (1.73 or later)
- Docker
- `just` command runner https://github.com/casey/just

### Installation

1. **Clone the Repository**:

    ```bash
    git clone https://github.com/jmfrank63/AdInMo.git
    cd AdInMo
    ```

2. **Build the Docker Images**:

    ```bash
    cp env.example .env
    ```

3. **Create a Docker Network**:

     ```bash
    just build
    ```

4. **Run the Containers**:

    ```bash
    just run
    ```

5. **(Wait a little ~ 15 seconds) Run the Integration Tests**:

    ```bash
    just test
    ```

### Components

- The Actix server is accessible at `http://server:3300/run`.
- The Hyper service runs internally within the Docker network and is
    accessible to the Actix server at `http://service:5500`.
- The database runs internally within the Docker network and is accessible
    to the Actix server at `http://mariadb:3306`.
- To run the integration tests, use `just test`. Please give about 15 seconds for
    everything else to startup.

## Development

- I think I ticked all the boxes. The whole development took about 30 hours. This includes
    about 15 hours of troubleshooting things that should work but didn't. So the actual coding
    time was about 15 hours, roughly 1/3 of the time goes to docker. The devops stuff can be
    quite time consuming.

- Rust code: Everything is async. The relevant crates have been used, sea-orm seemed to be
    overkill and thus I chose sqlx. Sqlx has however a nasty bug not detecting mariadb and
    thus poluting rust-analyzer with errors when using the macros. Thus the macros had not
    been used which adds a little more lines to the code.
    There are also no doc comments. For a project that is looked at for a couple of days
    maximum this would be a waste of resources. The code should be written in a way that
    it explains itself. Where i think a comment is useful I added one.
    The abstraction of the database was a nice experiment. It has certainly some advantages,
    but also some drawbacks. The abstraction is not fully complete. You will find a
    few sqlx entries in the logs. This can however be easily fixed, I just didn't do it.
    There is always a better to the good and I think the abstraction is good enough for
    demonstrating how it could be done.

- I had no prior experience with a hexagonal architecture. Hyper and Sqlx were new to me as well.
    New was also trying to completely abstracting the database away from the server. I might have
    overdone it a little, at least the username should be parameterized. But for the purpose of the
    demo just using one user is fine. Once I figured out where the pool should be created, it was
    quite straight forward.

- One major roadblocks was docker and the reference to another workspace. It seems like my solution
    is the minimum possible. Building from within the crate does not seem to work. I am happy to
    learn, should there be another solution apart from building the whole workspace.

- The middleware was also not especially easy. The async nature of the call made from there
    provided a major headache. Initially I had the service directly as a member of the struct,
    but that did not work, for now I couldn't clone the service. Once wrapped into an Arc, it worked.

- The project is by far not polished. But for the purpose of demonstrating my current skill level, it
    should be sufficient. Four container and 5 workspaces might be a little bit over-engineered
    but it was certainly fun to do.

- The docker containers are not optimized for size. They are also not pushed to my repo. You have to
    build the project. It should however build on Linux and Mac both Intel and Arm. Linux Intel
    and Mac Arm are tested.

### Actix Server

- Located in the `server` directory.
- Contains the Rust code for the Actix web server.

### Hyper Service

- Located in the `service` directory.
- Contains the Rust code for the Hyper service.

### Database

- Located in the `database` directory.
- Contains the Dockerfile for the MariaDB database.

## Networking

- All containers are part of the adinmo-network Docker network.

## Justfile

- A `justfile` is included for simplifying build and run processes.
- Use `just build` to build Docker images.
- Use `just run` to run the containers.
- Use `just test` to run the integration tests.

Just is a good choice for development, though if there are more containers either
  docker-compose or ansible would be a better choice.

## Configuration

- The addresses and ports are configurable via environment variables.
    The project should run out of the box, but if you want to change the ports
    or addresses, you can do so by setting environment variables in .env
    env.example contains all the necessary values.
- Not everything works directly by changing the variables. To shorten times, sometimes usernames
    and passwords are defined in multiple places. But this can be changed so a single place
    can be the source of truth. I just didn't want to spend too much time on it.
- Simply copy over env.example to .env and everything should work.

## Challenge

- A minor bug is in the project, it is very unlikely you will encounter it. I left it in
  so should there be a technical discussion we can talk about it.

## License

This project is licensed under the MIT License - see the LICENSE.md file for details.
