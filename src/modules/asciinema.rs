use std::env;
use super::{Context, Module, RootModuleConfig};

use crate::configs::asciinema::AsciinemaConfig;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_debug = env::var("ASCIINEMA_REC").ok().is_some();

    if !is_debug {
        return None;
    }

    let mut module = context.new_module("asciinema");
    let config: AsciinemaConfig = AsciinemaConfig::try_load(module.config);

    module.set_style(config.style);

    // let formatted_version = node_version.trim();
    module.create_segment("symbol", &config.symbol);
    // module.create_segment("version", &SegmentConfig::new(formatted_version));

    Some(module)
}
