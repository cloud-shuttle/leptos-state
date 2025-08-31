use leptos_state::machine::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct GameContext {
    player_health: i32,
    player_level: u32,
    coins: u32,
    score: u64,
    achievements: Vec<String>,
    game_session_id: String,
}

impl Default for GameContext {
    fn default() -> Self {
        Self {
            player_health: 100,
            player_level: 1,
            coins: 0,
            score: 0,
            achievements: Vec::new(),
            game_session_id: uuid::Uuid::new_v4().to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum GameEvent {
    StartGame,
    CollectCoin,
    TakeDamage(i32),
    LevelUp,
    SaveGame,
    QuitGame,
    UnlockAchievement(String),
}

impl Event for GameEvent {
    fn event_type(&self) -> &str {
        match self {
            GameEvent::StartGame => "start_game",
            GameEvent::CollectCoin => "collect_coin",
            GameEvent::TakeDamage(_) => "take_damage",
            GameEvent::LevelUp => "level_up",
            GameEvent::SaveGame => "save_game",
            GameEvent::QuitGame => "quit_game",
            GameEvent::UnlockAchievement(_) => "unlock_achievement",
        }
    }
}

fn main() {
    println!("=== State Machine Documentation Generator Example ===");
    
    // Create a game state machine with documentation capabilities
    let machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("menu")
            .on_entry_log("Entered game menu")
            .on_entry_fn(|ctx, _| {
                println!("🎮 Welcome to the game menu!");
            })
            .on(GameEvent::StartGame, "playing")
                .action_fn(|ctx, _| {
                    println!("🚀 Starting new game session: {}", ctx.game_session_id);
                })
                .action_log("Game started")
        .state("playing")
            .on_entry_log("Entered playing state")
            .on_entry_fn(|ctx, _| {
                println!("🎯 Game is now active! Health: {}, Level: {}", ctx.player_health, ctx.player_level);
            })
            .on_exit_log("Exited playing state")
            .on(GameEvent::CollectCoin, "playing")
                .action_fn(|ctx, _| {
                    ctx.coins += 1;
                    ctx.score += 10;
                    println!("🪙 Collected coin! Coins: {}, Score: {}", ctx.coins, ctx.score);
                })
                .action_log("Coin collected")
            .on(GameEvent::TakeDamage(0), "playing")
                .action_fn(|ctx, event| {
                    if let GameEvent::TakeDamage(amount) = event {
                        ctx.player_health = (ctx.player_health - amount).max(0);
                        println!("💔 Took {} damage! Health: {}", amount, ctx.player_health);
                    }
                })
                .action_log("Player took damage")
            .on(GameEvent::LevelUp, "playing")
                .action_fn(|ctx, _| {
                    ctx.player_level += 1;
                    ctx.player_health = 100; // Full heal on level up
                    println!("⭐ Level up! New level: {}, Health restored!", ctx.player_level);
                })
                .action_log("Player leveled up")
            .on(GameEvent::SaveGame, "playing")
                .action_fn(|ctx, _| {
                    println!("💾 Game saved!");
                })
                .action_log("Game saved")
            .on(GameEvent::QuitGame, "menu")
                .action_fn(|ctx, _| {
                    println!("👋 Quitting game. Final score: {}", ctx.score);
                })
                .action_log("Game quit")
        .build_documented();
    
    println!("✓ Documented machine created");
    
    // Test 1: Basic Documentation Generation
    println!("\n--- Test 1: Basic Documentation Generation ---");
    
    let docs = machine.generate_documentation().unwrap();
    println!("Generated {} documentation files", docs.len());
    
    for doc in &docs {
        println!("  - {}: {} bytes", format!("{:?}", doc.format), doc.content.len());
    }
    
    // Test 2: Multiple Documentation Formats
    println!("\n--- Test 2: Multiple Documentation Formats ---");
    
    let multi_format_config = DocumentationConfig {
        output_formats: vec![
            DocumentationFormat::Markdown,
            DocumentationFormat::Html,
            DocumentationFormat::Json,
            DocumentationFormat::Yaml,
            DocumentationFormat::AsciiDoc,
            DocumentationFormat::Rst,
        ],
        output_directory: "multi_format_docs".to_string(),
        ..Default::default()
    };
    
    let multi_format_machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("idle")
            .on(GameEvent::StartGame, "active")
        .state("active")
            .on(GameEvent::CollectCoin, "active")
            .on(GameEvent::QuitGame, "idle")
        .build_with_documentation(multi_format_config);
    
    let multi_docs = multi_format_machine.generate_documentation().unwrap();
    println!("Generated {} different format documentation files", multi_docs.len());
    
    for doc in &multi_docs {
        println!("  - {}: {}", format!("{:?}", doc.format), doc.file_path);
    }
    
    // Test 3: Documentation Templates
    println!("\n--- Test 3: Documentation Templates ---");
    
    let template_configs = vec![
        DocumentationTemplate::Default,
        DocumentationTemplate::Minimal,
        DocumentationTemplate::Comprehensive,
        DocumentationTemplate::ApiDocs,
        DocumentationTemplate::UserGuide,
    ];
    
    println!("Available documentation templates:");
    for template in &template_configs {
        println!("  - {:?}", template);
    }
    
    // Test 4: Documentation Styling
    println!("\n--- Test 4: Documentation Styling ---");
    
    let custom_styling = DocumentationStyling {
        theme: "dark".to_string(),
        custom_css: Some("body { background-color: #1a1a1a; color: #ffffff; }".to_string()),
        logo_url: Some("https://example.com/logo.png".to_string()),
        primary_color: "#ff6b6b".to_string(),
        secondary_color: "#4ecdc4".to_string(),
        font_family: "Roboto, sans-serif".to_string(),
        font_size: "16px".to_string(),
    };
    
    let styled_config = DocumentationConfig {
        output_formats: vec![DocumentationFormat::Html],
        output_directory: "styled_docs".to_string(),
        styling: custom_styling,
        ..Default::default()
    };
    
    let styled_machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("idle")
            .on(GameEvent::StartGame, "active")
        .state("active")
            .on(GameEvent::CollectCoin, "active")
        .build_with_documentation(styled_config);
    
    let styled_docs = styled_machine.generate_documentation().unwrap();
    println!("Generated styled documentation with custom CSS");
    
    // Test 5: Documentation Builder Pattern
    println!("\n--- Test 5: Documentation Builder Pattern ---");
    
    let built_docs = DocumentationBuilder::new(
        MachineBuilder::<GameContext, GameEvent>::new()
            .state("menu")
                .on(GameEvent::StartGame, "playing")
            .state("playing")
                .on(GameEvent::CollectCoin, "playing")
                .on(GameEvent::QuitGame, "menu")
            .build()
    )
    .with_format(DocumentationFormat::Markdown)
    .with_format(DocumentationFormat::Html)
    .with_format(DocumentationFormat::Json)
    .with_output_directory("builder_docs".to_string())
    .with_template(DocumentationTemplate::Comprehensive)
    .with_diagrams(true)
    .with_code_examples(true)
    .with_api_docs(true)
    .with_usage_examples(true)
    .with_metadata("version".to_string(), "1.0.0".to_string())
    .with_metadata("author".to_string(), "Game Developer".to_string())
    .with_metadata("project".to_string(), "State Machine Game".to_string())
    .with_styling(DocumentationStyling {
        theme: "modern".to_string(),
        primary_color: "#6366f1".to_string(),
        secondary_color: "#8b5cf6".to_string(),
        font_family: "Inter, system-ui, sans-serif".to_string(),
        font_size: "15px".to_string(),
        ..Default::default()
    })
    .build();
    
    let builder_docs = built_docs.generate_documentation().unwrap();
    println!("Generated documentation using builder pattern:");
    for doc in &builder_docs {
        println!("  - {}: {} bytes", format!("{:?}", doc.format), doc.content.len());
    }
    
    // Test 6: Custom Templates
    println!("\n--- Test 6: Custom Templates ---");
    
    let custom_template = r#"
# {{title}}

## Machine Overview
This state machine has the following components:
- **States**: {{states}}
- **Events**: {{events}}

## Quick Start
1. Create a new machine instance
2. Define states and transitions
3. Build and use the machine

## Configuration
- Template: Custom
- Generated: {{generated_at}}
"#;
    
    let custom_config = DocumentationConfig {
        output_formats: vec![DocumentationFormat::Custom("custom".to_string())],
        output_directory: "custom_template_docs".to_string(),
        ..Default::default()
    };
    
    let custom_machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("idle")
            .on(GameEvent::StartGame, "active")
        .state("active")
            .on(GameEvent::CollectCoin, "active")
        .build_with_documentation(custom_config);
    
    // Add custom template
    custom_machine.add_template("custom".to_string(), custom_template.to_string());
    
    let custom_docs = custom_machine.generate_documentation().unwrap();
    println!("Generated documentation with custom template");
    
    // Test 7: Documentation Content Analysis
    println!("\n--- Test 7: Documentation Content Analysis ---");
    
    let markdown_doc = docs.iter().find(|d| matches!(d.format, DocumentationFormat::Markdown)).unwrap();
    println!("Markdown documentation analysis:");
    println!("  - Content length: {} characters", markdown_doc.content.len());
    println!("  - Contains states section: {}", markdown_doc.content.contains("## States"));
    println!("  - Contains events section: {}", markdown_doc.content.contains("## Events"));
    println!("  - Contains transitions section: {}", markdown_doc.content.contains("## Transitions"));
    println!("  - Contains usage examples: {}", markdown_doc.content.contains("## Usage Examples"));
    println!("  - Contains API reference: {}", markdown_doc.content.contains("## API Reference"));
    println!("  - Contains state diagram: {}", markdown_doc.content.contains("## State Diagram"));
    
    let html_doc = docs.iter().find(|d| matches!(d.format, DocumentationFormat::Html)).unwrap();
    println!("HTML documentation analysis:");
    println!("  - Content length: {} characters", html_doc.content.len());
    println!("  - Contains DOCTYPE: {}", html_doc.content.contains("<!DOCTYPE html>"));
    println!("  - Contains CSS styling: {}", html_doc.content.contains("<style>"));
    println!("  - Contains Mermaid script: {}", html_doc.content.contains("mermaid"));
    println!("  - Contains responsive viewport: {}", html_doc.content.contains("viewport"));
    
    let json_doc = docs.iter().find(|d| matches!(d.format, DocumentationFormat::Json)).unwrap();
    println!("JSON documentation analysis:");
    println!("  - Content length: {} characters", json_doc.content.len());
    println!("  - Contains title field: {}", json_doc.content.contains("\"title\""));
    println!("  - Contains states array: {}", json_doc.content.contains("\"states\""));
    println!("  - Contains events array: {}", json_doc.content.contains("\"events\""));
    println!("  - Contains transitions array: {}", json_doc.content.contains("\"transitions\""));
    println!("  - Contains timestamp: {}", json_doc.content.contains("\"generated_at\""));
    
    // Test 8: Documentation Index Generation
    println!("\n--- Test 8: Documentation Index Generation ---");
    
    let index = machine.generate_index().unwrap();
    println!("Generated documentation index:");
    println!("{}", index);
    
    // Test 9: Documentation Configuration Options
    println!("\n--- Test 9: Documentation Configuration Options ---");
    
    let config_options = vec![
        ("Minimal", DocumentationConfig {
            output_formats: vec![DocumentationFormat::Markdown],
            include_diagrams: false,
            include_code_examples: false,
            include_api_docs: false,
            include_usage_examples: false,
            ..Default::default()
        }),
        ("Comprehensive", DocumentationConfig {
            output_formats: vec![DocumentationFormat::Markdown, DocumentationFormat::Html, DocumentationFormat::Json],
            include_diagrams: true,
            include_code_examples: true,
            include_api_docs: true,
            include_usage_examples: true,
            ..Default::default()
        }),
        ("API Only", DocumentationConfig {
            output_formats: vec![DocumentationFormat::Markdown],
            include_diagrams: false,
            include_code_examples: false,
            include_api_docs: true,
            include_usage_examples: false,
            ..Default::default()
        }),
        ("User Guide", DocumentationConfig {
            output_formats: vec![DocumentationFormat::Html],
            include_diagrams: true,
            include_code_examples: true,
            include_api_docs: false,
            include_usage_examples: true,
            ..Default::default()
        }),
    ];
    
    println!("Documentation configuration presets:");
    for (name, config) in &config_options {
        println!("  - {}: {} formats, diagrams={}, examples={}, api={}, usage={}", 
            name,
            config.output_formats.len(),
            config.include_diagrams,
            config.include_code_examples,
            config.include_api_docs,
            config.include_usage_examples
        );
    }
    
    // Test 10: Documentation Metadata
    println!("\n--- Test 10: Documentation Metadata ---");
    
    let metadata_config = DocumentationConfig {
        output_formats: vec![DocumentationFormat::Markdown],
        output_directory: "metadata_docs".to_string(),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("version".to_string(), "2.1.0".to_string());
            meta.insert("author".to_string(), "State Machine Team".to_string());
            meta.insert("license".to_string(), "MIT".to_string());
            meta.insert("repository".to_string(), "https://github.com/example/state-machine".to_string());
            meta.insert("documentation_url".to_string(), "https://docs.example.com".to_string());
            meta.insert("support_email".to_string(), "support@example.com".to_string());
            meta
        },
        ..Default::default()
    };
    
    let metadata_machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("idle")
            .on(GameEvent::StartGame, "active")
        .state("active")
            .on(GameEvent::CollectCoin, "active")
        .build_with_documentation(metadata_config);
    
    let metadata_docs = metadata_machine.generate_documentation().unwrap();
    println!("Generated documentation with metadata");
    
    // Test 11: Documentation File Management
    println!("\n--- Test 11: Documentation File Management ---");
    
    let file_config = DocumentationConfig {
        output_formats: vec![
            DocumentationFormat::Markdown,
            DocumentationFormat::Html,
            DocumentationFormat::Json,
            DocumentationFormat::Yaml,
        ],
        output_directory: "file_management_docs".to_string(),
        ..Default::default()
    };
    
    let file_machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("idle")
            .on(GameEvent::StartGame, "active")
        .state("active")
            .on(GameEvent::CollectCoin, "active")
        .build_with_documentation(file_config);
    
    let file_docs = file_machine.generate_documentation().unwrap();
    println!("Generated documentation files:");
    for doc in &file_docs {
        println!("  - {}", doc.file_path);
        println!("    Format: {:?}", doc.format);
        println!("    Size: {} bytes", doc.content.len());
        println!("    Generated: {:?}", doc.generated_at);
    }
    
    // Test 12: Documentation Performance
    println!("\n--- Test 12: Documentation Performance ---");
    
    let start_time = std::time::Instant::now();
    
    let perf_config = DocumentationConfig {
        output_formats: vec![
            DocumentationFormat::Markdown,
            DocumentationFormat::Html,
            DocumentationFormat::Json,
            DocumentationFormat::Yaml,
            DocumentationFormat::AsciiDoc,
            DocumentationFormat::Rst,
        ],
        output_directory: "performance_docs".to_string(),
        include_diagrams: true,
        include_code_examples: true,
        include_api_docs: true,
        include_usage_examples: true,
        ..Default::default()
    };
    
    let perf_machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("menu")
            .on(GameEvent::StartGame, "playing")
        .state("playing")
            .on(GameEvent::CollectCoin, "playing")
            .on(GameEvent::TakeDamage(0), "playing")
            .on(GameEvent::LevelUp, "playing")
            .on(GameEvent::SaveGame, "playing")
            .on(GameEvent::QuitGame, "menu")
        .build_with_documentation(perf_config);
    
    let perf_docs = perf_machine.generate_documentation().unwrap();
    let generation_time = start_time.elapsed();
    
    println!("Performance test results:");
    println!("  - Generated {} documentation files", perf_docs.len());
    println!("  - Total generation time: {:?}", generation_time);
    println!("  - Average time per format: {:?}", generation_time / perf_docs.len() as u32);
    println!("  - Total documentation size: {} bytes", 
        perf_docs.iter().map(|d| d.content.len()).sum::<usize>());
    
    // Test 13: Documentation Quality Check
    println!("\n--- Test 13: Documentation Quality Check ---");
    
    let quality_config = DocumentationConfig {
        output_formats: vec![DocumentationFormat::Markdown],
        output_directory: "quality_docs".to_string(),
        include_diagrams: true,
        include_code_examples: true,
        include_api_docs: true,
        include_usage_examples: true,
        ..Default::default()
    };
    
    let quality_machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("menu")
            .on(GameEvent::StartGame, "playing")
        .state("playing")
            .on(GameEvent::CollectCoin, "playing")
            .on(GameEvent::QuitGame, "menu")
        .build_with_documentation(quality_config);
    
    let quality_docs = quality_machine.generate_documentation().unwrap();
    let markdown_doc = &quality_docs[0];
    
    println!("Documentation quality check:");
    println!("  - Has title: {}", markdown_doc.content.contains("# State Machine Documentation"));
    println!("  - Has overview: {}", markdown_doc.content.contains("## Overview"));
    println!("  - Has states section: {}", markdown_doc.content.contains("## States"));
    println!("  - Has events section: {}", markdown_doc.content.contains("## Events"));
    println!("  - Has transitions section: {}", markdown_doc.content.contains("## Transitions"));
    println!("  - Has guards section: {}", markdown_doc.content.contains("## Guards"));
    println!("  - Has actions section: {}", markdown_doc.content.contains("## Actions"));
    println!("  - Has usage examples: {}", markdown_doc.content.contains("## Usage Examples"));
    println!("  - Has API reference: {}", markdown_doc.content.contains("## API Reference"));
    println!("  - Has state diagram: {}", markdown_doc.content.contains("## State Diagram"));
    println!("  - Has code blocks: {}", markdown_doc.content.contains("```rust"));
    println!("  - Has mermaid diagram: {}", markdown_doc.content.contains("```mermaid"));
    
    // Test 14: Documentation Templates Comparison
    println!("\n--- Test 14: Documentation Templates Comparison ---");
    
    let templates = vec![
        DocumentationTemplate::Default,
        DocumentationTemplate::Minimal,
        DocumentationTemplate::Comprehensive,
        DocumentationTemplate::ApiDocs,
        DocumentationTemplate::UserGuide,
    ];
    
    for template in templates {
        let template_config = DocumentationConfig {
            output_formats: vec![DocumentationFormat::Markdown],
            output_directory: format!("template_{:?}_docs", template).to_lowercase(),
            template,
            ..Default::default()
        };
        
        let template_machine = MachineBuilder::<GameContext, GameEvent>::new()
            .state("idle")
                .on(GameEvent::StartGame, "active")
            .state("active")
                .on(GameEvent::CollectCoin, "active")
            .build_with_documentation(template_config);
        
        let template_docs = template_machine.generate_documentation().unwrap();
        println!("  - {:?}: {} bytes", template, template_docs[0].content.len());
    }
    
    // Test 15: Comprehensive Documentation Workflow
    println!("\n--- Test 15: Comprehensive Documentation Workflow ---");
    
    // Create a comprehensive documentation setup
    let comprehensive_config = DocumentationConfig {
        output_formats: vec![
            DocumentationFormat::Markdown,
            DocumentationFormat::Html,
            DocumentationFormat::Json,
            DocumentationFormat::Yaml,
        ],
        output_directory: "comprehensive_docs".to_string(),
        template: DocumentationTemplate::Comprehensive,
        include_diagrams: true,
        include_code_examples: true,
        include_api_docs: true,
        include_usage_examples: true,
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("project".to_string(), "State Machine Game Engine".to_string());
            meta.insert("version".to_string(), "3.0.0".to_string());
            meta.insert("author".to_string(), "Game Development Team".to_string());
            meta.insert("license".to_string(), "Apache 2.0".to_string());
            meta.insert("repository".to_string(), "https://github.com/game-dev/state-machine".to_string());
            meta.insert("documentation".to_string(), "https://docs.game-dev.com".to_string());
            meta.insert("support".to_string(), "support@game-dev.com".to_string());
            meta.insert("changelog".to_string(), "https://github.com/game-dev/state-machine/blob/main/CHANGELOG.md".to_string());
            meta
        },
        styling: DocumentationStyling {
            theme: "modern".to_string(),
            custom_css: Some(r#"
                .state { border-left: 4px solid #3b82f6; }
                .event { border-left: 4px solid #10b981; }
                .transition { border-left: 4px solid #f59e0b; }
                .code-block { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); }
            "#.to_string()),
            logo_url: Some("https://game-dev.com/logo.svg".to_string()),
            primary_color: "#3b82f6".to_string(),
            secondary_color: "#64748b".to_string(),
            font_family: "Inter, -apple-system, BlinkMacSystemFont, sans-serif".to_string(),
            font_size: "16px".to_string(),
        },
    };
    
    let comprehensive_machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("menu")
            .on_entry_log("Entered game menu")
            .on(GameEvent::StartGame, "playing")
                .action_log("Game started")
        .state("playing")
            .on_entry_log("Entered playing state")
            .on(GameEvent::CollectCoin, "playing")
                .action_log("Coin collected")
            .on(GameEvent::TakeDamage(0), "playing")
                .action_log("Player took damage")
            .on(GameEvent::LevelUp, "playing")
                .action_log("Player leveled up")
            .on(GameEvent::SaveGame, "playing")
                .action_log("Game saved")
            .on(GameEvent::QuitGame, "menu")
                .action_log("Game quit")
        .build_with_documentation(comprehensive_config);
    
    let comprehensive_docs = comprehensive_machine.generate_documentation().unwrap();
    
    println!("Comprehensive documentation workflow completed:");
    println!("  - Generated {} documentation files", comprehensive_docs.len());
    println!("  - Total documentation size: {} bytes", 
        comprehensive_docs.iter().map(|d| d.content.len()).sum::<usize>());
    
    for doc in &comprehensive_docs {
        println!("  - {}: {} bytes", format!("{:?}", doc.format), doc.content.len());
    }
    
    // Generate documentation index
    let index = comprehensive_machine.generate_index().unwrap();
    println!("\nDocumentation index generated:");
    println!("{}", index);
    
    println!("\n=== Documentation Generator Example Completed ===");
    println!("Check the generated documentation files in the output directories!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_documentation_generation() {
        let machine = MachineBuilder::<GameContext, GameEvent>::new()
            .state("idle")
                .on(GameEvent::StartGame, "active")
            .state("active")
                .on(GameEvent::CollectCoin, "active")
            .build_documented();
        
        let docs = machine.generate_documentation().unwrap();
        assert!(!docs.is_empty());
        
        let markdown_doc = docs.iter().find(|d| matches!(d.format, DocumentationFormat::Markdown)).unwrap();
        assert!(markdown_doc.content.contains("# State Machine Documentation"));
        assert!(markdown_doc.content.contains("## States"));
        assert!(markdown_doc.content.contains("## Events"));
    }
    
    #[test]
    fn test_multiple_format_generation() {
        let config = DocumentationConfig {
            output_formats: vec![
                DocumentationFormat::Markdown,
                DocumentationFormat::Html,
                DocumentationFormat::Json,
            ],
            output_directory: "test_docs".to_string(),
            ..Default::default()
        };
        
        let machine = MachineBuilder::<GameContext, GameEvent>::new()
            .state("idle")
                .on(GameEvent::StartGame, "active")
            .state("active")
                .on(GameEvent::CollectCoin, "active")
            .build_with_documentation(config);
        
        let docs = machine.generate_documentation().unwrap();
        assert_eq!(docs.len(), 3);
        
        let formats: Vec<_> = docs.iter().map(|d| &d.format).collect();
        assert!(formats.contains(&&DocumentationFormat::Markdown));
        assert!(formats.contains(&&DocumentationFormat::Html));
        assert!(formats.contains(&&DocumentationFormat::Json));
    }
    
    #[test]
    fn test_documentation_builder() {
        let machine = MachineBuilder::<GameContext, GameEvent>::new()
            .state("idle")
                .on(GameEvent::StartGame, "active")
            .state("active")
                .on(GameEvent::CollectCoin, "active")
            .build();
        
        let generator = DocumentationBuilder::new(machine)
            .with_format(DocumentationFormat::Markdown)
            .with_format(DocumentationFormat::Html)
            .with_output_directory("builder_test_docs".to_string())
            .with_template(DocumentationTemplate::Comprehensive)
            .with_diagrams(true)
            .with_code_examples(true)
            .with_api_docs(true)
            .with_usage_examples(true)
            .with_metadata("version".to_string(), "1.0.0".to_string())
            .with_styling(DocumentationStyling {
                theme: "dark".to_string(),
                primary_color: "#ff0000".to_string(),
                secondary_color: "#00ff00".to_string(),
                ..Default::default()
            })
            .build();
        
        let config = generator.config;
        assert_eq!(config.output_formats.len(), 2);
        assert_eq!(config.output_directory, "builder_test_docs");
        assert_eq!(config.template, DocumentationTemplate::Comprehensive);
        assert!(config.include_diagrams);
        assert!(config.include_code_examples);
        assert!(config.include_api_docs);
        assert!(config.include_usage_examples);
        assert_eq!(config.metadata.get("version"), Some(&"1.0.0".to_string()));
        assert_eq!(config.styling.theme, "dark");
        assert_eq!(config.styling.primary_color, "#ff0000");
    }
    
    #[test]
    fn test_custom_template() {
        let config = DocumentationConfig {
            output_formats: vec![DocumentationFormat::Custom("custom".to_string())],
            output_directory: "custom_test_docs".to_string(),
            ..Default::default()
        };
        
        let machine = MachineBuilder::<GameContext, GameEvent>::new()
            .state("idle")
                .on(GameEvent::StartGame, "active")
            .state("active")
                .on(GameEvent::CollectCoin, "active")
            .build_with_documentation(config);
        
        // Add custom template
        machine.add_template("custom".to_string(), "Title: {{title}}\nStates: {{states}}\nEvents: {{events}}".to_string());
        
        let docs = machine.generate_documentation().unwrap();
        assert_eq!(docs.len(), 1);
        assert!(docs[0].content.contains("Title: State Machine Documentation"));
        assert!(docs[0].content.contains("States:"));
        assert!(docs[0].content.contains("Events:"));
    }
    
    #[test]
    fn test_documentation_index() {
        let machine = MachineBuilder::<GameContext, GameEvent>::new()
            .state("idle")
                .on(GameEvent::StartGame, "active")
            .state("active")
                .on(GameEvent::CollectCoin, "active")
            .build_documented();
        
        let index = machine.generate_index().unwrap();
        assert!(index.contains("# Documentation Index"));
        assert!(index.contains("Generated documentation files:"));
        assert!(index.contains("Generation Info"));
    }
}
