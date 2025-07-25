use super::{Context, Module, RootModuleConfig};

use crate::configs::mise::MiseConfig;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_mise_project = context
        .try_begin_scan()?
        .set_files(&["mise.toml", "mise.local.toml"])
        .is_match();

    if !is_mise_project {
        return None;
    }

    let mut module = context.new_module("mise");
    let config: MiseConfig = MiseConfig::try_load(module.config);

    module.set_style(config.style);

    // let formatted_version = node_version.trim();
    module.create_segment("symbol", &config.symbol);
    // module.create_segment("version", &SegmentConfig::new(formatted_version));

    Some(module)
}
