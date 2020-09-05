# actix-web-cron-example

## About 

An example actix-web project with a simple cron job making use of the Actix Actor framework.

## Examples 

Parses a cron schedule and executes a closure when it is due next using: [Actix run_later](https://actix.rs/actix/actix/prelude/trait.AsyncContext.html#method.run_later)


## Usage

Clone, build, and run

```bash
$ cargo run

Running `target/debug/actix-web-cron-example`
Actor is alive
schedule_task event - 2020-09-04T20:47:00.003280-07:00
schedule_task event - 2020-09-04T20:48:00.001561-07:00
schedule_task event - 2020-09-04T20:49:00.005547-07:00
```

## Dependencies

* [Actix](https://actix.rs/) - a powerful Actor framework
* [Cron](https://docs.rs/cron/0.6.1/cron/) - A cron expression parser and schedule explorer
* [Chrono](https://github.com/chronotope/chrono) - a Date and Time library for Rust

