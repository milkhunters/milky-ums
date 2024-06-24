# Running Migrator CLI

- Generate a new migration file
    ```sh
    sea-orm-cli migrate generate "example migration"
    ```
- Apply migrations

  You need to set the `DATABASE_URL` environment variable to the database URL.

  ```sh
  export DATABASE_URL=postgresql://username:password@host:5432/dbname
  ```

  ```sh
  sea-orm-cli migrate up
  ```
