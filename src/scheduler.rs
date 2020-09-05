
use actix::prelude::*;
use std::{time::Duration};


// Define actor
pub struct Scheduler;

// Provide Actor implementation for our actor
impl Actor for Scheduler {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
       println!("Actor is alive");
       let five_second_ms = 5000;

        // executes every 5 seconds
        ctx.run_interval(
            Duration::from_millis(five_second_ms),
            move |_this, _ctx| {
                println!("run_interval event");
            },
        );
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
       println!("Actor is stopped");
    }

}
