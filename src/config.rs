use crate::formatter::FormatterType;
use crate::level::LogLevel;
use crate::output::OutputType;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LogConfig {
    pub level: LogLevel,
    pub colors_enabled: bool,
    pub timestamps: bool,
    pub show_date: bool,
    pub show_target: bool,
    pub show_module: bool,
    pub show_file_line: bool,
    pub bold_subtitles: bool,
    pub formatter: FormatterType,
    pub output: OutputType,
    pub indent_size: usize,
    pub max_width: Option<usize>,
    pub custom_fields: HashMap<String, String>,
    pub filter_targets: Vec<String>,
    pub exclude_targets: Vec<String>,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            colors_enabled: true,
            timestamps: true,
            show_date: false,
            show_target: false,
            show_module: false,
            show_file_line: false,
            bold_subtitles: true,
            formatter: FormatterType::Pretty,
            output: OutputType::Stdout,
            indent_size: 2,
            max_width: None,
            custom_fields: HashMap::new(),
            filter_targets: Vec::new(),
            exclude_targets: Vec::new(),
        }
    }
}

impl LogConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    pub fn with_colors(mut self, enabled: bool) -> Self {
        self.colors_enabled = enabled;
        self
    }

    pub fn with_timestamps(mut self, enabled: bool) -> Self {
        self.timestamps = enabled;
        self
    }

    pub fn with_date(mut self, enabled: bool) -> Self {
        self.show_date = enabled;
        self
    }

    pub fn with_target(mut self, enabled: bool) -> Self {
        self.show_target = enabled;
        self
    }

    pub fn with_module(mut self, enabled: bool) -> Self {
        self.show_module = enabled;
        self
    }

    pub fn with_file_line(mut self, enabled: bool) -> Self {
        self.show_file_line = enabled;
        self
    }

    pub fn with_bold_subtitles(mut self, enabled: bool) -> Self {
        self.bold_subtitles = enabled;
        self
    }

    pub fn with_formatter(mut self, formatter: FormatterType) -> Self {
        self.formatter = formatter;
        self
    }

    pub fn with_output(mut self, output: OutputType) -> Self {
        self.output = output;
        self
    }

    pub fn with_indent_size(mut self, size: usize) -> Self {
        self.indent_size = size;
        self
    }

    pub fn with_max_width(mut self, width: usize) -> Self {
        self.max_width = Some(width);
        self
    }

    pub fn with_custom_field(mut self, key: String, value: String) -> Self {
        self.custom_fields.insert(key, value);
        self
    }

    pub fn filter_target(mut self, target: String) -> Self {
        self.filter_targets.push(target);
        self
    }

    pub fn exclude_target(mut self, target: String) -> Self {
        self.exclude_targets.push(target);
        self
    }

    pub fn should_log(&self, level: LogLevel, target: &str) -> bool {
        // Check log level
        if level < self.level {
            return false;
        }

        // Check exclude targets
        if self.exclude_targets.iter().any(|t| target.contains(t)) {
            return false;
        }

        // Check filter targets (if any specified, target must match)
        if !self.filter_targets.is_empty() {
            return self.filter_targets.iter().any(|t| target.contains(t));
        }

        true
    }

    pub fn pretty() -> Self {
        Self::default()
            .with_colors(true)
            .with_timestamps(true)
            .with_formatter(FormatterType::Pretty)
    }

    pub fn compact() -> Self {
        Self::default()
            .with_colors(false)
            .with_timestamps(false)
            .with_formatter(FormatterType::Compact)
    }

    pub fn json() -> Self {
        Self::default()
            .with_colors(false)
            .with_timestamps(true)
            .with_formatter(FormatterType::Json)
    }

    pub fn dev() -> Self {
        Self::default()
            .with_colors(true)
            .with_timestamps(true)
            .with_module(true)
            .with_file_line(true)
            .with_formatter(FormatterType::Pretty)
    }
}
