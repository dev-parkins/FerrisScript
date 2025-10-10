//! FerrisScript Headless Testing Harness
//!
//! Provides automated testing infrastructure for FerrisScript + Godot integration.
//! Enables:
//! - Headless Godot execution via CLI
//! - Dynamic test scene generation
//! - Structured output parsing
//! - CI-friendly test reporting

pub mod godot_cli;
pub mod metadata_parser;
pub mod output_parser;
pub mod scene_builder;
pub mod test_config;
pub mod test_runner;

pub use godot_cli::{GodotRunner, TestOutput};
pub use metadata_parser::{
    Assertion, AssertionKind, MetadataParser, ParseError, TestCategory, TestExpectation,
    TestMetadata,
};
pub use output_parser::{OutputParser, TestMarker, TestMarkerKind, TestResults};
pub use scene_builder::SceneBuilder;
pub use test_config::{OutputFormat, TestConfig};
pub use test_runner::{TestHarness, TestResult};

/// Result type for test harness operations
pub type Result<T> = anyhow::Result<T>;
