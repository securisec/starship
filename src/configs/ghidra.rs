use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GhidraConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for GhidraConfig<'a> {
    fn new() -> Self {
        GhidraConfig {
            symbol: SegmentConfig::new(""),
            style: Color::Yellow.bold(),
            disabled: false,
        }
    }
}
