use super::{Context, Module, RootModuleConfig};

use crate::configs::docker::DockerConfig;

///
///
///     - Current directory contains a `.ts` file
///     - Current directory contains a `tsconfig.json` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_js_project = context
        .try_begin_scan()?
        .set_files(&["Dockerfile", "docker-compose.yml", "docker-compose.yaml"])
        // .set_extensions(&["ts"])
        // .set_folders(&["node_modules"])
        .is_match();

    if !is_js_project {
        return None;
    }

    // let node_version = utils::exec_cmd("node", &["--version"])?.stdout;

    let mut module = context.new_module("docker");
    let config: DockerConfig = DockerConfig::try_load(module.config);

    module.set_style(config.style);

    // let formatted_version = node_version.trim();
    module.create_segment("symbol", &config.symbol);
    // module.create_segment("version", &SegmentConfig::new(formatted_version));

    Some(module)
}
