use actix::prelude::*;
use anyhow::Result;
use tokio::time::{sleep, Duration};

pub struct SmallTask;

impl Message for SmallTask {
    type Result = ();
}
struct ComplexTask;

impl Message for ComplexTask {
    type Result = ();
}
pub struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.notify(ComplexTask {});
        ctx.notify(SmallTask {});
    }
}
impl Handler<SmallTask> for MyActor {
    type Result = ();

    fn handle(&mut self, _msg: SmallTask, ctx: &mut Context<Self>) -> Self::Result {
        let future = Box::pin(async {
            println!("Easy task done!");
        });

        let actor_future = future.into_actor(self);

        ctx.spawn(actor_future);
    }
}
impl Handler<ComplexTask> for MyActor {
    type Result = ();

    fn handle(&mut self, _msg: ComplexTask, ctx: &mut Context<Self>) -> Self::Result {
        let future = Box::pin(async {
            sleep(Duration::from_secs(3)).await;
            println!("Complex task done!");
        });

        let actor_future = future.into_actor(self);

        ctx.spawn(actor_future);
    }
}

pub fn runner() -> Result<()> {
    let sys = actix::System::new();

    sys.block_on(async {
        let _addr = MyActor {}.start();
    });
    sys.run()?;

    Ok(())
}
