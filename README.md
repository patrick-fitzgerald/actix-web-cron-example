# actix-web-cron-example

## About 

A simple cron job example with a health check endpoint.

It uses [actix_web](https://github.com/actix/actix-web) and [tokio_schedule](https://github.com/dedefer/tokio_schedule).

## Usage

Clone, build, and run

```bash
$ cargo run

Running `target/debug/actix-web-cron-example`
schedule_task event - 2022-09-22T21:41:36.001745-07:00
schedule_task event - 2022-09-22T21:41:37.002239-07:00
schedule_task event - 2022-09-22T21:41:38.001552-07:00
schedule_task event - 2022-09-22T21:41:39.000929-07:00
schedule_task event - 2022-09-22T21:41:40.001357-07:00
```

Health check endpoint:

```
curl http://127.0.0.1:8080/health

>> success
```

## Dependencies

* [actix-web](https://crates.io/crates/actix-web) - a powerful, pragmatic, and extremely fast web framework for Rust
* [actix-rt](https://crates.io/crates/actix-rt) - Tokio-based single-threaded async runtime for the Actix ecosystem
* [chrono](https://github.com/chronotope/chrono) - a Date and Time library for Rust
* [tokio_schedule](https://github.com/dedefer/tokio_schedule) - schedule tasks in tokio runtime

