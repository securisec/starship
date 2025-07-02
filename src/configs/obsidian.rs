use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct ObsidianConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for ObsidianConfig<'a> {
    fn new() -> Self {
        ObsidianConfig {
            symbol: SegmentConfig::new(""),
            style: Color::Yellow.bold(),
            disabled: false,
        }
    }
}
