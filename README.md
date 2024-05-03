## Introduction

Simple todos web build with rust, actix, and htmx.

## Getting Started

1. Install [Rust](https://www.rust-lang.org/tools/install)

2. Install SQLx CLI

```bash
cargo install sqlx-cli
```

3. Clone the repository

```bash
git clone https://github.com/davidepastore/todos-actix-htmx
```

4. Rename the env file to `.env`

5. Create the database and run the migration for the first time

```bash
cd todos-actix-htmx

sqlx database create

sqlx migrate run
```

6. Run the web

```bash
cargo run
```

4. Go to http://localhost:3000
