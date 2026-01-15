# Table Trace (Rust)

TableTrace is a lightweight HTTP API that extracts the table names involved in a given SQL query. It supports parsing
queries and returning a JSON list of table names. This is a **Rust** version of the [Table Trace](https://github.com/rafafrdz/table-trace) project written in **Scala**.

## Keep in mind

:warning: The logic for traversing the parsed query AST is not fully implemented or thoroughly explored. The current implementation is a naive approach intended for demonstration purposes. For example, it supports the query shown in the example below, but it does not handle all possible queries.

## Getting Started

Follow these steps to set up and run your Table Trace api:

### Cloning the Repository

First, clone this repository to your local machine:

```bash
git clone git@github.com:rafafrdz/table-trace-rs.git
cd table-trace-rs
```

## Prerequisites

- Rust 1.89+
- Cargo

### Running the Table Trace api locally

You can run the Table Trace api directly from Cargo. Open a terminal in the project directory and execute:

```bash
cargo run
```

By default, the service runs on [`http://localhost:9876`](http://localhost:9876)

## API Usage

### Endpoint

**POST** `/analyze`

**Body**: JSON with the field query containing the SQL string.

**Response**: JSON array with the extracted tables.

### Examples

#### Succeeded Request

```bash
curl -X POST http://localhost:9876/analyze -H "Content-Type: application/json" -d '{
"query": "UPDATE wine w SET stock = stock - (SELECT SUM(quantity) FROM order WHERE date = CURRENT_DATE AND order.wine_name = w.name) WHERE w.name IN (SELECT order.wine_name FROM order WHERE date = CURRENT_DATE)"
}'
```

#### Succeeded Response

```json
 [
  "wine",
  "order"
]

```

#### Failed Request

```bash
curl -X POST http://localhost:9876/analyze -H "Content-Type: application/json" -d '{
"query": "UPDATE wine w WHERE w.name IN (SELECT order.wine_name FROM order WHERE date = CURRENT_DATE)"
}'
```

#### Failed Response

```json
{
  "error": "Error processing the query `UPDATE wine w WHERE w.name IN (SELECT order.wine_name FROM order WHERE date = CURRENT_DATE)`. Validation error: sql parser error: Expected: SET, found: WHERE at Line: 1, Column: 15"
}
```

### Building the project

Prepare your project for distribution or deployment by building it with Cargo:

1. Navigate to the project directory:

```bash
cd table-trace-rs
```

2. Run the following command:

```bash
cargo build --release
```

This command creates a binary in `target/release/table-trace-rs`. For more details of this command, refer to the [cargo-build](https://doc.rust-lang.org/cargo/commands/cargo-build.html#compilation-options) documentation.

### Running the Binary

Once the binary is built, you can run the API directly with:

```bash
./target/release/table-trace-rs
```

By default, the service runs on [`http://localhost:9876`](http://localhost:9876)

## License

This project is available under your choice of the Apache 2.0 or CC0 1.0 license. Choose the one that best suits your
needs:

- [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [CC0 1.0 Universal (Public Domain Dedication)](https://creativecommons.org/publicdomain/zero/1.0/)

This template is provided "as-is" without any warranties. Modify and distribute as needed to fit your project
requirements.
