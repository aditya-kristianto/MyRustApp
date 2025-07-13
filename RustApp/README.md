<a name="readme-top"></a>

# Rust

Rust project

## Build with

* [![Actix][Actix.rs]][Actix-url]
* [![JQuery][JQuery.com]][JQuery-url]
* [![MongoDB][Mongodb.com]][Mongodb-url]
* [![Postgresql][Postgresql.org]][Postgresql-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## The Guides

- [Installation](docs/readme/installation.md)
- [Usage](docs/readme/usage.md)
- [Advanced](docs/readme/advanced.md)
- [API](docs/readme/api.md)
- [ERD](docs/readme/erd.md)
- [Diagram](docs/readme/diagram.md)
- [Environment Variables (.env)](docs/readme/env.md)

## Project Structure
my_project/
|-- src/
|   |-- main.rs
|   |-- lib.rs
|   |-- handlers/
|       |-- mod.rs
|       |-- user_handler.rs
|   |-- models/
|       |-- mod.rs
|       |-- user.rs
|   |-- routes/
|       |-- mod.rs
|       |-- user_routes.rs
|   |-- services/
|       |-- mod.rs
|       |-- user_service.rs
|   |-- utils/
|       |-- mod.rs
|       |-- helpers.rs
|-- migrations/
|-- static/
|-- templates/
|-- tests/
|-- .gitignore
|-- Cargo.toml
|-- diesel.toml

Explanation of the structure:

- src/: The main source code directory.
    - main.rs: The entry point for the application.
    - lib.rs: Optionally, if you want to structure your project as a library crate (useful for testing).
- handlers/: Modules for request handlers.
  - mod.rs: Module file to re-export modules in the handlers directory.
  - user_handler.rs: Module containing request handlers related to user operations.
- models/: Database models and related code.
  - mod.rs: Module file to re-export modules in the models directory.
  - user.rs: Module containing the User struct or any other database models.
- routes/: Modules for defining routes.
  - mod.rs: Module file to re-export modules in the routes directory.
  - user_routes.rs: Module defining routes related to user operations.
- services/: Business logic and services.
  - mod.rs: Module file to re-export modules in the services directory.
  - user_service.rs: Module containing business logic for user operations.
- utils/: Utility modules.
  - mod.rs: Module file to re-export modules in the utils directory.
  - helpers.rs: Module containing utility functions/helpers.
- migrations/: Database migration files if you're using a database.
- static/: Static files (CSS, JavaScript, etc.) that will be served by the web server.
- templates/: HTML templates if your web framework uses them.
- tests/: Directory for tests.
- .gitignore: Git ignore file to specify which files and directories to ignore.
- Cargo.toml: Rust package configuration file.
- diesel.toml: Diesel configuration file if you are using Diesel as an ORM.

## Documentation

### How to Install MongoDB Database using Docker 

```
docker run -d -p 27017:27017 --name mongodb \
    -e MONGO_INITDB_ROOT_USERNAME=root \
    -e MONGO_INITDB_ROOT_PASSWORD=password \
    mongo:latest
```

### How to run Rust project

```
cargo run
```

### How to test Rust project

```
cargo test
```

### Setup the environment variables

```
APP_NAME=rust
APP_VERSION=1.0
APP_HOST=127.0.0.1
APP_PORT=8080
APP_ENV=local

MONGODB_URI=mongodb://root:password@127.0.0.1:27017/?retryWrites=true&w=majority
MONGODB_USERNAME=root
MONGODB_PASSWORD=password
MONGODB_DATABASE=rust

RUST_BACKTRACE=1
```

### Rust optimization
```
https://github.com/johnthagen/min-sized-rust
```

###
``` Install Kafka
https://developer.confluent.io/quickstart/kafka-docker/?utm_medium=sem&utm_source=google&utm_campaign=ch.sem_br.nonbrand_tp.prs_tgt.dsa_mt.dsa_rgn.apac_lng.eng_dv.all_con.confluent-developer&utm_term=&creative=&device=c&placement=&gad=1&gclid=CjwKCAjwvdajBhBEEiwAeMh1U-40c50_RxWpc0Lq4NRKSU4_4MKlMBrmZHH8JbDDADei7c0Ytgz8uxoCtDQQAvD_BwE
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Docker Image
![alt text](./docs/readme/Screen%20Shot%202023-08-14%20at%2021.33.25.png)

Explanation :
- v1.0-development-debug : using rust alpine docker image v1.70.0-alpine3.18 and debug target
- v2.0-development-debug : using alpine docker image v3.18.3
- v3.0-development-debug : using rust alpine docker image v1.71.1-alpine3.18
- v3.0-debug : using target debug
- v3.0-release : using target release
- 

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Authors

- [@aditya.kristianto](https://github.com/aditya-kristianto)

## 

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[Actix.rs]: https://img.shields.io/badge/actix.rs-000000?style=for-the-badge&logo=rust&logoColor=white
[Actix-url]: https://actix.rs/
[JQuery.com]: https://img.shields.io/badge/jQuery-0769AD?style=for-the-badge&logo=jquery&logoColor=white
[JQuery-url]: https://jquery.com
[Mongodb.com]: https://img.shields.io/badge/mongodb-12614a?style=for-the-badge&logo=mongodb&logoColor=white
[Mongodb-url]: https://mongodb.com
[Postgresql.org]: https://img.shields.io/badge/postgresql-336791?style=for-the-badge&logo=postgresql&logoColor=white
[Postgresql-url]: https://www.postgresql.org