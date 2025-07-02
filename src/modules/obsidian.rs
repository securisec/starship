use super::{Context, Module, RootModuleConfig};

use crate::configs::obsidian::ObsidianConfig;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_obsidian_project = context
        .try_begin_scan()?
        // .set_files(&["obsidian.toml", "obsidian.local.toml"])
        .set_folders(&[".obsidian"])
        .is_match();

    if !is_obsidian_project {
        return None;
    }

    let mut module = context.new_module("obsidian");
    let config: ObsidianConfig = ObsidianConfig::try_load(module.config);

    module.set_style(config.style);

    // let formatted_version = node_version.trim();
    module.create_segment("symbol", &config.symbol);
    // module.create_segment("version", &SegmentConfig::new(formatted_version));

    Some(module)
}
