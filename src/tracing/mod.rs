//! Minimal tracing infrastructure for the AVM
//!
//! This module provides lightweight tracing functionality focused on
//! opcode execution and stack state tracking for debugging purposes.

use serde::{Deserialize, Serialize};
use tracing::{Level, info};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// Tracing configuration levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceLevel {
    /// Trace level - most verbose
    Trace,
    /// Debug level - detailed information
    Debug,
    /// Info level - general information
    Info,
    /// Warn level - warnings only
    Warn,
    /// Error level - errors only
    Error,
}

impl Default for TraceLevel {
    fn default() -> Self {
        Self::Info
    }
}

impl From<TraceLevel> for Level {
    fn from(level: TraceLevel) -> Self {
        match level {
            TraceLevel::Trace => Level::TRACE,
            TraceLevel::Debug => Level::DEBUG,
            TraceLevel::Info => Level::INFO,
            TraceLevel::Warn => Level::WARN,
            TraceLevel::Error => Level::ERROR,
        }
    }
}

/// Minimal tracing configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TracingConfig {
    /// Enable tracing
    pub enabled: bool,
    /// Tracing level
    pub level: TraceLevel,
    /// Enable opcode-level tracing
    pub trace_opcodes: bool,
    /// Enable stack state tracing
    pub trace_stack: bool,
    /// Maximum stack depth to trace
    pub max_stack_trace_depth: usize,
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            level: TraceLevel::Info,
            trace_opcodes: true,
            trace_stack: true,
            max_stack_trace_depth: 10,
        }
    }
}

impl TracingConfig {
    /// Create a new tracing configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable tracing with specified level
    pub fn with_level(mut self, level: TraceLevel) -> Self {
        self.enabled = true;
        self.level = level;
        self
    }

    /// Enable opcode tracing
    pub fn with_opcodes(mut self, enabled: bool) -> Self {
        self.trace_opcodes = enabled;
        self
    }

    /// Enable stack tracing
    pub fn with_stack(mut self, enabled: bool) -> Self {
        self.trace_stack = enabled;
        self
    }

    /// Set maximum stack trace depth
    pub fn with_max_stack_depth(mut self, depth: usize) -> Self {
        self.max_stack_trace_depth = depth;
        self
    }
}

/// Tracing subscriber guard to ensure proper cleanup
pub struct TracingGuard;

/// Initialize tracing with the given configuration
pub fn init_tracing(config: &TracingConfig) -> Result<TracingGuard, Box<dyn std::error::Error>> {
    if !config.enabled {
        return Ok(TracingGuard);
    }

    {
        let level: Level = config.level.into();
        let filter = EnvFilter::builder()
            .with_default_directive(level.into())
            .from_env_lossy()
            .add_directive("avm_rs=debug".parse()?)
            .add_directive("avm_rs::vm=debug".parse()?)
            .add_directive("avm_rs::opcodes=debug".parse()?);

        let registry = tracing_subscriber::registry().with(filter);

        let fmt_layer = fmt::layer()
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .compact();

        // Try to set global subscriber, but don't fail if already set
        if registry.with(fmt_layer).try_init().is_err() {
            // Global subscriber already set, just continue
        }

        info!(
            version = env!("CARGO_PKG_VERSION"),
            level = ?config.level,
            opcodes = config.trace_opcodes,
            stack = config.trace_stack,
            "AVM tracing initialized"
        );

        Ok(TracingGuard)
    }
}

/// Helper trait for tracing-enabled types
pub trait Traceable {
    /// Convert to a trace-safe representation
    fn to_trace_string(&self) -> String;
}

/// Stack value representation for tracing
pub fn stack_to_trace_string(stack: &[crate::types::StackValue], max_depth: usize) -> String {
    let depth = stack.len().min(max_depth);
    if depth == 0 {
        return "[]".to_string();
    }

    let values: Vec<String> = stack
        .iter()
        .rev()
        .take(depth)
        .map(|v| match v {
            crate::types::StackValue::Uint(n) => format!("uint({n})"),
            crate::types::StackValue::Bytes(b) => {
                if b.len() <= 8 {
                    format!("bytes(0x{})", hex::encode(b))
                } else {
                    format!("bytes({}B)", b.len())
                }
            }
        })
        .collect();

    if stack.len() > max_depth {
        format!("[{}, ... +{}]", values.join(", "), stack.len() - max_depth)
    } else {
        format!("[{}]", values.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracing_config_builder() {
        let config = TracingConfig::new()
            .with_level(TraceLevel::Debug)
            .with_opcodes(true)
            .with_stack(true);

        assert!(config.enabled);
        assert_eq!(config.level, TraceLevel::Debug);
        assert!(config.trace_opcodes);
        assert!(config.trace_stack);
    }

    #[test]
    fn test_stack_to_trace_string() {
        use crate::types::StackValue;

        let stack = vec![
            StackValue::uint(42),
            StackValue::bytes(vec![0x01, 0x02, 0x03]),
            StackValue::uint(100),
        ];

        let trace_str = stack_to_trace_string(&stack, 5);
        assert!(trace_str.contains("uint(100)"));
        assert!(trace_str.contains("bytes(0x010203)"));
        assert!(trace_str.contains("uint(42)"));
    }

    #[test]
    fn test_stack_to_trace_string_truncated() {
        use crate::types::StackValue;

        let stack: Vec<StackValue> = (0..10).map(StackValue::uint).collect();
        let trace_str = stack_to_trace_string(&stack, 3);

        assert!(trace_str.contains("... +7"));
    }
}
