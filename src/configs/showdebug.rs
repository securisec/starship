use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct ShowdebugConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for ShowdebugConfig<'a> {
    fn new() -> Self {
        ShowdebugConfig {
            symbol: SegmentConfig::new(""),
            style: Color::Blue.bold(),
            disabled: false,
        }
    }
}
