use crate::config::LogConfig;
use crate::context::LogContext;
use crate::level::LogLevel;
use owo_colors::OwoColorize;
use serde_json;

#[derive(Debug, Clone)]
pub enum FormatterType {
    Pretty,
    Compact,
    Json,
    Custom(fn(&LogLevel, &str, &LogContext, &LogConfig) -> String),
}

pub struct Formatter {
    config: LogConfig,
}

impl Formatter {
    pub fn new(config: LogConfig) -> Self {
        Self { config }
    }

    pub fn format(&self, level: LogLevel, message: &str, context: &LogContext) -> String {
        match &self.config.formatter {
            FormatterType::Pretty => self.format_pretty(level, message, context),
            FormatterType::Compact => self.format_compact(level, message, context),
            FormatterType::Json => self.format_json(level, message, context),
            FormatterType::Custom(formatter) => formatter(&level, message, context, &self.config),
        }
    }

    fn format_pretty(&self, level: LogLevel, message: &str, context: &LogContext) -> String {
        let mut parts = Vec::new();

        // Timestamp
        if self.config.timestamps {
            let timestamp = if self.config.show_date {
                context.timestamp.format("%Y-%m-%d %H:%M:%S%.3f")
            } else {
                context.timestamp.format("%H:%M:%S%.3f")
            };

            if self.config.colors_enabled {
                parts.push(format!("{}", timestamp.dimmed()));
            } else {
                parts.push(timestamp.to_string());
            }
        }

        // Level with colors
        let level_str = level.short_name().to_string();

        if self.config.colors_enabled {
            let colored_level = match level {
                LogLevel::Trace => level_str.purple().to_string(),
                LogLevel::Debug => level_str.blue().to_string(),
                LogLevel::Info => level_str.green().to_string(),
                LogLevel::Warn => level_str.yellow().to_string(),
                LogLevel::Error => level_str.red().to_string(),
                LogLevel::Fatal => level_str.on_red().white().bold().to_string(),
            };
            parts.push(format!("[{}]", colored_level));
        } else {
            parts.push(format!("[{}]", level_str));
        }

        // Subtitle with bold formatting and colors
        if let Some(ref subtitle) = context.subtitle {
            if self.config.colors_enabled && self.config.bold_subtitles {
                let colored_subtitle = match level {
                    LogLevel::Trace => subtitle.purple().bold().to_string(),
                    LogLevel::Debug => subtitle.blue().bold().to_string(),
                    LogLevel::Info => subtitle.green().bold().to_string(),
                    LogLevel::Warn => subtitle.yellow().bold().to_string(),
                    LogLevel::Error => subtitle.red().bold().to_string(),
                    LogLevel::Fatal => subtitle.on_red().white().bold().to_string(),
                };
                parts.push(colored_subtitle);
            } else if self.config.bold_subtitles {
                parts.push(subtitle.bold().to_string());
            } else if self.config.colors_enabled {
                let colored_subtitle = match level {
                    LogLevel::Trace => subtitle.purple().to_string(),
                    LogLevel::Debug => subtitle.blue().to_string(),
                    LogLevel::Info => subtitle.green().to_string(),
                    LogLevel::Warn => subtitle.yellow().to_string(),
                    LogLevel::Error => subtitle.red().to_string(),
                    LogLevel::Fatal => subtitle.on_red().white().to_string(),
                };
                parts.push(colored_subtitle);
            } else {
                parts.push(subtitle.clone());
            }
        }

        // Target/Module
        if self.config.show_target {
            if self.config.colors_enabled {
                parts.push(format!("{}", context.target.cyan()));
            } else {
                parts.push(context.target.clone());
            }
        }

        if self.config.show_module {
            if let Some(ref module) = context.module {
                if self.config.colors_enabled {
                    parts.push(format!("{}::", module.cyan()));
                } else {
                    parts.push(format!("{}::", module));
                }
            }
        }

        // File and line
        if self.config.show_file_line {
            if let (Some(ref file), Some(line)) = (&context.file, context.line) {
                if self.config.colors_enabled {
                    parts.push(format!("({}:{})", file.dimmed(), line.to_string().dimmed()));
                } else {
                    parts.push(format!("({}:{})", file, line));
                }
            }
        }

        // Indentation for nested logs
        let _indent = " ".repeat(context.nesting_level() as usize * self.config.indent_size);
        let indent_marker = if context.is_nested() {
            if self.config.colors_enabled {
                "│ "
                    .repeat(context.nesting_level() as usize)
                    .dimmed()
                    .to_string()
            } else {
                "│ ".repeat(context.nesting_level() as usize)
            }
        } else {
            String::new()
        };

        // Message
        let formatted_message = if self.config.colors_enabled {
            match level {
                LogLevel::Error | LogLevel::Fatal => message.red().to_string(),
                LogLevel::Warn => message.yellow().to_string(),
                LogLevel::Info => message.white().to_string(),
                LogLevel::Debug => message.blue().to_string(),
                LogLevel::Trace => message.purple().to_string(),
            }
        } else {
            message.to_string()
        };

        // Custom fields
        let mut fields_str = String::new();
        if !context.fields.is_empty() {
            let fields: Vec<String> = context
                .fields
                .iter()
                .map(|(k, v)| {
                    if self.config.colors_enabled {
                        format!("{}={}", k.cyan(), v.to_string().white())
                    } else {
                        format!("{}={}", k, v)
                    }
                })
                .collect();
            fields_str = format!(" {{{}}}", fields.join(", "));
        }

        // Combine all parts
        let prefix = if parts.is_empty() {
            String::new()
        } else {
            format!("{} ", parts.join(" "))
        };

        // Apply width limit if configured
        let full_message = format!(
            "{}{}{}{}{}",
            indent_marker, prefix, formatted_message, fields_str, ""
        );

        if let Some(max_width) = self.config.max_width {
            if full_message.len() > max_width {
                format!("{}...", &full_message[..max_width.saturating_sub(3)])
            } else {
                full_message
            }
        } else {
            full_message
        }
    }

    fn format_compact(&self, level: LogLevel, message: &str, context: &LogContext) -> String {
        let timestamp = if self.config.timestamps {
            if self.config.show_date {
                context.timestamp.format("%Y-%m-%d %H:%M:%S").to_string()
            } else {
                context.timestamp.format("%H:%M:%S").to_string()
            }
        } else {
            String::new()
        };

        let level_char = match level {
            LogLevel::Trace => "T",
            LogLevel::Debug => "D",
            LogLevel::Info => "I",
            LogLevel::Warn => "W",
            LogLevel::Error => "E",
            LogLevel::Fatal => "F",
        };

        let prefix = if timestamp.is_empty() {
            format!("{} ", level_char)
        } else {
            format!("{} {} ", timestamp, level_char)
        };

        let indent = "  ".repeat(context.nesting_level() as usize);
        format!("{}{}{}", prefix, indent, message)
    }

    fn format_json(&self, level: LogLevel, message: &str, context: &LogContext) -> String {
        let mut json_obj = serde_json::json!({
            "timestamp": context.timestamp.to_rfc3339(),
            "level": level.as_str(),
            "message": message,
            "target": context.target,
            "id": context.id,
            "nesting_level": context.nesting_level(),
        });

        if let Some(ref subtitle) = context.subtitle {
            json_obj["subtitle"] = serde_json::Value::String(subtitle.clone());
        }

        if let Some(ref module) = context.module {
            json_obj["module"] = serde_json::Value::String(module.clone());
        }

        if let (Some(ref file), Some(line)) = (&context.file, context.line) {
            json_obj["file"] = serde_json::Value::String(file.clone());
            json_obj["line"] = serde_json::Value::Number(line.into());
        }

        if let Some(ref parent_id) = context.parent_id {
            json_obj["parent_id"] = serde_json::Value::String(parent_id.clone());
        }

        if !context.fields.is_empty() {
            json_obj["fields"] = serde_json::Value::Object(
                context
                    .fields
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            );
        }

        serde_json::to_string(&json_obj).unwrap_or_else(|_| "{}".to_string())
    }
}

pub fn colorize_level(level: LogLevel, text: &str, enabled: bool) -> String {
    if !enabled {
        return text.to_string();
    }

    match level {
        LogLevel::Trace => text.purple().to_string(),
        LogLevel::Debug => text.blue().to_string(),
        LogLevel::Info => text.green().to_string(),
        LogLevel::Warn => text.yellow().to_string(),
        LogLevel::Error => text.red().to_string(),
        LogLevel::Fatal => text.on_red().white().bold().to_string(),
    }
}
