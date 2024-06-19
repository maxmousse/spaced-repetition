# Spaced Repetition

This project aims to facilitate knowledge retention for users through the implementation of the 'spaced repetition' technique.

## Prerequisites

To utilize this repository and execute the project, ensure you have the following:

- Node.js 20 and npm
- Rust and Cargo
- Docker and docker-compose

## Project Setup

Follow these steps to set up the project:

- Clone the repository to your local machine.
- Navigate to the root of the repository and execute `npm i`.
- Generate project code by running `npx nx run-many --target generate --all`. For detailed instructions on code generation, refer to the 'Code Generation' section.
- Create a `.env` file containing necessary environment variables in the root directory of the repository.
- As the project relies on external software (such as a Postgres database), you can launch them using Docker Compose with `docker-compose up -d`.
- During initial installation, initialize the database by pushing its schema with `npx prisma db push` (refer to the database section for further details).
- Run the seed script to populate the database with dummy data using `npx prisma db seed`. The seed script is located in `apps/api/src/seed.rs`.
- Start the API with `npx nx serve api`. It will be accessible at `http://localhost:8080`. Note that the project heavily relies on Nx. See the 'Project Structure' section for more information.
- Launch the frontend with `npx nx serve pwa`. Access it at `http://localhost:4200`. Note that a development proxy is in place, making the API available at `http://localhost:4200/api` to mitigate CORS issues.
- You're now all set to begin!

## Database Management

This project employs Prisma ORM (check <https://www.prisma.io/> for documentation). Prisma offers numerous features. Here are some commonly used ones:

- Utilize Prisma Studio, a visual interface provided by Prisma, for database interaction. Launch it with `npx prisma studio`.
- Other common operations include:
  - `npx prisma db push`: Pushes the SQL schema to the database.
  - `npx prisma db seed`: Seeds the database.
  - `npx prisma db migrate reset`: Resets the database to an empty state using migrations.
  - `npx prisma migrate dev`: Creates a new migration after modifying the database schema.

Refer to Prisma's documentation for comprehensive information on available commands and workflow for updating the database schema.

## Code Generation

This project utilizes various code generation tools to automate repetitive and error-prone tasks.

- Prisma employs code generation to produce a custom client that aligns with the database schema. Generate the Prisma client with `npx nx run prisma_client:generate`. This should be done whenever the Prisma schema is updated.
- The API/frontend communication relies on GraphQL. The GraphQL schema is represented 'as code' in the API. To generate the appropriate GraphQL schema (`schema.gql`), execute `npx nx run api:generate`.
- All TypeScript types necessary for frontend GraphQL queries to ensure type safety are generated from the GraphQL schema and queries using [codegen](https://the-guild.dev/graphql/codegen/docs/guides/angular). Generate these types with `npx nx run graphql-types:generate`. This command should be run whenever the schema or GraphQL queries are updated.

## Project Structure

This project utilizes Nx, a build system (refer to <https://nx.dev/getting-started/intro> for documentation), to manage and optimize the monorepository. Nx provides a CLI accessible via `npx nx`. If you're unfamiliar with this tool, we strongly recommend reviewing the documentation as the project heavily depends on it.

### Applications

Applications are self-sufficient code artifacts that can be deployed independently. The project comprises the following applications:

#### api

The API application serves application data. It exposes a GraphQL API documented at `http://localhost:8080/` and a GRAPHQL API documented at `http://localhost:3000/api/graphql`. It is built with Rust and Actix.

#### pwa

The PWA serves as the public frontend of the application. It is constructed with Angular (<https://angular.io/>).

### Libraries

Libraries are code artifacts designed to be used by other libraries or applications. They are not self-sufficient and cannot be deployed independently.
