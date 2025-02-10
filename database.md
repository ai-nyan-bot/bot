## Database Management

This project uses `sqlx-cli` for managing the database. `sqlx-cli` is a command-line tool for working with databases,
allowing you to create and manage migrations and handle other database-related tasks efficiently.

The migration tool requires a metadata table which will be located at `traffic._sqlx_migrations`.
Note: `sqlx` does not support to configure the metadata table location - to work around this we create a user with
default schema (`search_path`).

## Setup

To get started with the database, follow these steps:

### Install `sqlx-cli`

First, you'll need to install `sqlx-cli`. You can find more details in the official
documentation [here](https://crates.io/crates/sqlx-cli).

To install `sqlx-cli`, run:

```shell
cargo install --version 0.8.3 sqlx-cli
sqlx --version
```

### 3. Configure Database connection; create a .env File

Create a .env file in the root of your project and set the DATABASE_URL environment variable to match your database
connection string:

```plaintext
DATABASE_URL=postgres://user:password@host:port/database
```

### 4. Run Migrations

To bring your database up to date with the latest schema, run the migrations:

```shell
sqlx migrate run
```

## Adding a New Migration

To add a new migration, use the following command:

```shell
sqlx migrate add -r <name>
```

Make sure to implement the *.up.sql and *.down.sql files accordingly to handle the migration logic.

## Schema Changes (development)

Offline mode allows macros like query!() to run their magic without requiring an active database connection. To prepare
your project for offline mode, run:

```shell
SQLX_OFFLINE=true
```

```shell
cargo sqlx prepare --workspace
# commit the changes in the .sqlx/ folder
```
