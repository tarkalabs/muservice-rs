# muservice-rs
muservice-rs is a template for creating microservices in Rust. This template comes with:
- Backend running an HTTP server with routing using `axum`
- Postgres database connected to the backend using `sqlx`
- Configuration with environment variables (12factor)
- Logging
- Example API endpoint handlers
- Integration tests
- CircleCI setup
- Docker containers

## Getting Started

### Using Docker
1. Clone the repository.
2. Install Docker Compose.
3. Build the service using `docker compose build`.
4. Start the containers using `docker compose up`.
5. View the logs using `docker compose logs muservice`.

### Without Docker
1. Clone the repository.
2. Start a Postgres database.
3. Set the database URL to point to a valid Postgres database in `settings/default.json`.
4. Run the service using `cargo run`.


## Environment variables
The environment variables are kept in the settings folder and are in JSON format. `settings/default.json` is always read and the values can be overwritten by having another JSON file whose name matches the environment variable `ENV`. You can also set environment variables through a `.env` file or the `environment` field of `docker-compose.yml`. Each file can having missing fields, as long as the combination of all settings sources contains all necessary variables.

## Tests
If you set up the project using Docker, the tests can be run by running `cargo test` within the `muservice` container. Otherwise, simply run `cargo test` from the project folder.
