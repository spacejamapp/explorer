> [!NOTE]
>
> Make thinkgs work first, optimization doesn't matter atm.

# The backend of Jamscan

development script:

```bash
# if not installed
cargo install sqlx-cli

# after installed sqlx-cli
export DATABASE_URL="postgres://postgres:postgres@localhost/jamscan"
sqlx db create
sqlx migrate run
cargo sqlx prepare
RUST_LOG=jamdex=trace,jamscan=trace cargo run
```

## Errors

via meeting sth like

```bash
Error: invalid parent: 0xcbadd1 != 0x000000
```

delete the database and start again.
