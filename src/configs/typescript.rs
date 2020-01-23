use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct TypescriptConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for TypescriptConfig<'a> {
    fn new() -> Self {
        TypescriptConfig {
            symbol: SegmentConfig::new(""),
            style: Color::Blue.bold(),
            disabled: false,
        }
    }
}
