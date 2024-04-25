# Milky User Management Service

## Description
This service is responsible for managing users in the Milky ecosystem. It provides the following functionalities:

- User registration
- User login
- User logout
- User profile update
- User password reset
- User password change
- User deletion
- User get
- User list retrieval
- User role assignment
- User role removal
- User role list retrieval
- User permission assignment
- User permission removal
- User permission list retrieval

## Build

```bash
cargo build --release
```

## Migrations

Install `sea-orm-cli`:

```bash
cargo install sea-orm-cli
```

Setup the database URL:

```bash
export DATABASE_URL=protocol://username:password@localhost/database
```

Run the migrations:

```bash
sea-orm migration run
```

