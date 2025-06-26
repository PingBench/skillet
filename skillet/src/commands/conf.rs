use crate::Context;
use crate::config::bootstrap;

use serde_json;

pub fn run(ctx: &Context) {
    let boot = bootstrap(ctx.quiet);

    if ctx.quiet {
        let json = serde_json::to_string_pretty(&boot.reconciled).unwrap();
        println!("{}", json);
    }
}
