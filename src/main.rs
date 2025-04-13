mod clock;
mod env;
mod executor;
mod logger;
mod scanner;

use crate::clock::Clock;
use crate::env::Env;
use crate::executor::SyncExecutor;
use crate::logger::init_logger;
use tracing::info;

fn main() {
    let env = Env::new();

    init_logger(env.verbose);

    info!("Started syncing {:?} to {:?}", &env.source, &env.target);
    let clock = Clock::new();

    let executor = SyncExecutor::new(&env.source, &env.target);
    executor.execute();

    info!("Syncing finished {:?}", clock.elapsed());
}
