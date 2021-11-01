use super::{Context, Module, RootModuleConfig};

use crate::configs::makefile::MakefileConfig;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_js_project = context
        .try_begin_scan()?
        .set_files(&["Makefile"])
        .is_match();

    if !is_js_project {
        return None;
    }

    let mut module = context.new_module("makefile");
    let config: MakefileConfig = MakefileConfig::try_load(module.config);

    module.set_style(config.style);

    // let formatted_version = node_version.trim();
    module.create_segment("symbol", &config.symbol);
    // module.create_segment("version", &SegmentConfig::new(formatted_version));

    Some(module)
}
