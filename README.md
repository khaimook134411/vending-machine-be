# Vending Machine Backend

This project serves as the backend for the vending machine application. It is built using Rust and connects to a PostgreSQL database. The backend provides APIs to manage vending machine operations such as cash inventory, orders, and products.

## Features
- Written in Rust for high performance and reliability
- Utilizes PostgreSQL as the database
- Exposed via Docker and Docker Compose for easy deployment
- Works seamlessly with the [frontend application](https://github.com/khaimook134411/vending-machine-fe)

## Prerequisites
Before running this project, ensure you have the following installed:
- [Docker](https://www.docker.com/)
- [Docker Compose](https://docs.docker.com/compose/)

## Getting Started

### 1. Clone the repository
```bash
git clone https://github.com/khaimook134411/vending-machine-be
cd vending-machine-be
```

### 2. Build and Run the Application

Using Docker Compose:
```bash
docker-compose up --build
```
This command will:
- Build the Rust application image
- Start the PostgreSQL container
- Start the Rust application container

### 3. Access the Application
- The backend will be accessible at `http://localhost:3000`
- The PostgreSQL database will be accessible on port `5433` with the following credentials:
  - **User:** postgres
  - **Password:** password
  - **Database:** vending_machine

## Project Structure
```
/app
├── Cargo.toml          # Rust project dependencies
├── Cargo.lock          # Dependency lock file
├── src/                # Rust source code
├── Dockerfile          # Dockerfile for the backend
├── docker-compose.yml  # Docker Compose configuration
```

## Environment Variables
The backend uses the following environment variables:
- **POSTGRES_USER:** PostgreSQL username (default: `postgres`)
- **POSTGRES_PASSWORD:** PostgreSQL password (default: `password`)
- **POSTGRES_DB:** PostgreSQL database name (default: `vending_machine`)

## Database Setup
The PostgreSQL database is initialized automatically when the container starts using the environment variables defined in `docker-compose.yml`.

To manually connect to the database, use the following command:
```bash
docker exec -it vending_postgres psql -U postgres -d vending_machine
```

## Frontend Integration
The backend works in tandem with the [frontend application](https://github.com/khaimook134411/vending-machine-fe). Make sure to set up and run the frontend to fully experience the vending machine application.

## Development Workflow

### Running Locally Without Docker
1. Ensure you have Rust and PostgreSQL installed.
2. Set up a `.env` file with the required database credentials.
3. Run the application:
   ```bash
   cargo run
   ```

## Deployment
For production deployments, ensure you:
- Use a secure password for the PostgreSQL database
- Configure environment variables appropriately
- Use tools like Docker Compose or Kubernetes for scaling and orchestration

