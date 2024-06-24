# Milky User Management Service

User Managment Service (или UMS) - прикладной сервис, который занимается управлением пользователей, сессиями и ролями.


## Сборка

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
sea-orm migration up
```

## Rollback migrations

```bash
sea-orm migration down
```

Generate entities:

```bash
sea-orm-cli generate entity \
    -u protocol://username:password@localhost/database \
    -o adapters/database/models
```