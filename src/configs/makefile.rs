use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct MakefileConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for MakefileConfig<'a> {
    fn new() -> Self {
        MakefileConfig {
            symbol: SegmentConfig::new(""),
            style: Color::Yellow.bold(),
            disabled: false,
        }
    }
}
