use actix::prelude::*;
use chrono::Local;
use cron::Schedule;
use std::{str::FromStr, time::Duration};

// Define actor
pub struct Scheduler;

// Provide Actor implementation for our actor
impl Actor for Scheduler {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is alive");

        ctx.run_later(duration_until_next(), move |this, ctx| {
            this.schedule_task(ctx)
        });
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}

impl Scheduler {
    fn schedule_task(&self, ctx: &mut Context<Self>) {
        // executes every 1 minute based on cron schedule
        println!("schedule_task event - {:?}", Local::now());

        ctx.run_later(duration_until_next(), move |this, ctx| {
            this.schedule_task(ctx)
        });
    }
}

pub fn duration_until_next() -> Duration {
    let cron_expression = "0 * * * * * *"; //every minute
    let cron_schedule = Schedule::from_str(cron_expression).unwrap();
    let now = Local::now();
    let next = cron_schedule.upcoming(Local).next().unwrap();
    let duration_until = next.signed_duration_since(now);
    duration_until.to_std().unwrap()
}
