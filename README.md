# Rustacean API
Rustacean API is a web service built with Rocket, Diesel, and SQLite. This service allows you to perform CRUD operations on Rustaceans (a representation of Rust programmers) via HTTP endpoints.

## Features
`Get all Rustaceans`: Retrieve a list of all Rustaceans.

`View a Rustacean`: Get detailed information about a specific Rustacean.
`Create a Rustacean`: Add a new Rustacean to the database.
`Update a Rustacean`: Update the information of an existing Rustacean.
`Delete a Rustacean`: Remove a Rustacean from the database.

## Prerequisites
Before running this project, ensure you have the following installed:

`Rust`
`SQLite`

## Getting Started
### Clone the Repository

`git clone https://github.com/yourusername/rustacean-api.git`
`cd rustacean-api`

## Set Up the Database
This project uses Diesel for ORM and database migrations. Initialize your database by running the following commands:

`cargo install diesel_cli --no-default-features --features sqlite`
`diesel setup`
`diesel migration run`

## Environment Configuration
Create a .env file in the root directory of the project and add the following configuration:

`ROCKET_DATABASES='{sqlite_db={url="db.sqlite"}}'`

### Running the Application
Build and run the application with Cargo:

`The API will be available at http://localhost:8000.`

## API Endpoints
1. `GET /rustaceans`: Get All Rustaceans
2. `View a Rustacean`: GET /rustaceans/<id>
3. `Create a Rustacean`: POST /rustaceans
4. `Update a Rustacean`: PUT /rustaceans/<id>
5. `Delete a Rustacean`: DELETE /rustaceans/<id>


## Contributing
Contributions are welcome! Please create a pull request with a clear description of your changes.

## License
This project is licensed under the MIT License. See the LICENSE file for details.

