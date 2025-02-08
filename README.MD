# Simple CRUD using Actix Web and SQLx MySQL

This project demonstrates a simple CRUD (Create, Read, Update, Delete) application using Actix Web and SQLx with a MySQL database.

## Prerequisites

- Rust (latest stable version)
- MySQL database
- Cargo (Rust package manager)

## Getting Started

### Clone the repository

```sh
git clone https://github.com/singgihpr/actix_simple_crud_mysql.git
cd actix_simple_crud_mysql
```

### Set up the MySQL database

1. Create a new MySQL database.
2. Update the `.env` file with your database credentials.

```env
DATABASE_URL=mysql://username:password@localhost/database_name
```

### Install dependencies

```sh
cargo install sqlx-cli
cargo build
```

### Run database migrations

```sh
sqlx migrate run
```

### Run the application

```sh
cargo run
```

## API Endpoints

- **Create a new item**
    - `POST /users`
    - Request body: JSON object with user details

- **Get all users**
    - `GET /users`

- **Get an item by ID**
    - `GET /users/{id}`

- **Update an item**
    - `PATCH /users/{id}`
    - Request body: JSON object with updated user details

- **Delete an item**
    - `DELETE /users/{id}`

## Project Structure

- `src/main.rs`: Entry point of the application.
- `src/models.rs`: Defines the data models.
- `src/errors.rs` : Define errors function.
- `src/routes.rs`: Defines the API routes and handlers.
- `src/db.rs`: Database connection and query functions.

## License

This project is licensed under the MIT License.
