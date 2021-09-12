mod single_actor;
mod single_actor_async_msgs;
use anyhow::Result;

fn main() -> Result<()> {
    single_actor::runner()
}
