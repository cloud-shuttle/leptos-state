//! Code Generation Example
//! 
//! This example demonstrates the code generation capabilities
//! of the state machine library.

use leptos_state::machine::*;
use leptos_state::machine::codegen::*;

#[derive(Debug, Clone, PartialEq)]
struct GameContext {
    score: i32,
    level: i32,
    lives: i32,
    player_name: String,
}

#[derive(Debug, Clone, PartialEq)]
enum GameEvent {
    Start,
    Pause,
    Resume,
    Stop,
    Score(i32),
    LevelUp,
    GameOver,
}

impl Event for GameEvent {
    fn event_type(&self) -> &str {
        match self {
            GameEvent::Start => "start",
            GameEvent::Pause => "pause",
            GameEvent::Resume => "resume",
            GameEvent::Stop => "stop",
            GameEvent::Score(_) => "score",
            GameEvent::LevelUp => "level_up",
            GameEvent::GameOver => "game_over",
        }
    }
}

fn main() {
    println!("=== State Machine Code Generation Example ===\n");

    // Create a state machine with code generation capabilities
    let machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("idle")
            .on(GameEvent::Start, "playing")
            .on_entry_fn(|ctx, _| {
                println!("Entering idle state");
                ctx.score = 0;
                ctx.level = 1;
                ctx.lives = 3;
            })
        .state("playing")
            .on(GameEvent::Pause, "paused")
            .on(GameEvent::Stop, "idle")
            .on(GameEvent::GameOver, "game_over")
            .on_entry_fn(|ctx, _| {
                println!("Starting game for player: {}", ctx.player_name);
            })
            .on_exit_fn(|ctx, _| {
                println!("Exiting playing state with score: {}", ctx.score);
            })
        .state("paused")
            .on(GameEvent::Resume, "playing")
            .on(GameEvent::Stop, "idle")
            .on_entry_fn(|ctx, _| {
                println!("Game paused at level {}", ctx.level);
            })
        .state("game_over")
            .on(GameEvent::Start, "idle")
            .on_entry_fn(|ctx, _| {
                println!("Game over! Final score: {}", ctx.score);
            })
        .initial("idle")
        .build_codegen();

    println!("Generated code files:");
    let files = machine.get_generated_files();
    for file in files {
        println!("- {} ({:?})", file.file_path, file.language);
    }

    // Generate code with custom configuration
    println!("\n=== Custom Code Generation ===");
    
    let custom_config = CodeGenConfig {
        enabled: true,
        target_languages: vec![
            ProgrammingLanguage::Rust,
            ProgrammingLanguage::TypeScript,
            ProgrammingLanguage::Python,
        ],
        output_directory: "custom_generated".to_string(),
        include_tests: true,
        include_documentation: true,
        metadata: {
            let mut map = HashMap::new();
            map.insert("version".to_string(), "1.0.0".to_string());
            map.insert("author".to_string(), "State Machine Generator".to_string());
            map
        },
    };

    let custom_generator = MachineBuilder::<GameContext, GameEvent>::new()
        .state("idle")
            .on(GameEvent::Start, "playing")
        .state("playing")
            .on(GameEvent::Stop, "idle")
        .initial("idle")
        .build_with_code_generation(custom_config);

    let custom_files = custom_generator.generate_code().unwrap();
    println!("Custom generated files:");
    for file in custom_files {
        println!("- {} ({:?})", file.file_path, file.language);
    }

    // Demonstrate builder pattern
    println!("\n=== Builder Pattern Example ===");
    
    let builder_generator = CodeGenBuilder::new(
        MachineBuilder::<GameContext, GameEvent>::new()
            .state("idle")
                .on(GameEvent::Start, "playing")
            .state("playing")
                .on(GameEvent::Stop, "idle")
            .initial("idle")
            .build()
    )
    .with_language(ProgrammingLanguage::Rust)
    .with_language(ProgrammingLanguage::TypeScript)
    .with_output_directory("builder_generated".to_string())
    .with_tests(true)
    .with_documentation(true)
    .with_metadata("generator".to_string(), "builder".to_string())
    .build();

    let builder_files = builder_generator.generate_code().unwrap();
    println!("Builder generated files:");
    for file in builder_files {
        println!("- {} ({:?})", file.file_path, file.language);
    }

    // Generate index
    println!("\n=== Generated Index ===");
    let index = machine.generate_index().unwrap();
    println!("{}", index);

    println!("\n=== Code Generation Complete ===");
    println!("Check the 'generated' directory for output files!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_code_generation() {
        let machine = MachineBuilder::<GameContext, GameEvent>::new()
            .state("idle")
                .on(GameEvent::Start, "playing")
            .state("playing")
                .on(GameEvent::Stop, "idle")
            .initial("idle")
            .build_codegen();

        let files = machine.get_generated_files();
        assert!(!files.is_empty());
        
        // Check that we have files for different languages
        let languages: Vec<_> = files.iter()
            .map(|f| &f.language)
            .collect();
        assert!(languages.contains(&&ProgrammingLanguage::Rust));
        assert!(languages.contains(&&ProgrammingLanguage::TypeScript));
    }

    #[test]
    fn test_custom_code_generation() {
        let config = CodeGenConfig {
            enabled: true,
            target_languages: vec![ProgrammingLanguage::Rust],
            output_directory: "test_generated".to_string(),
            include_tests: true,
            include_documentation: false,
            metadata: HashMap::new(),
        };

        let generator = MachineBuilder::<GameContext, GameEvent>::new()
            .state("idle")
                .on(GameEvent::Start, "playing")
            .state("playing")
                .on(GameEvent::Stop, "idle")
            .initial("idle")
            .build_with_code_generation(config);

        let files = generator.generate_code().unwrap();
        assert!(!files.is_empty());
        
        // Should have Rust files
        let rust_files: Vec<_> = files.iter()
            .filter(|f| matches!(f.language, ProgrammingLanguage::Rust))
            .collect();
        assert!(!rust_files.is_empty());
        
        // Should have test files
        let test_files: Vec<_> = files.iter()
            .filter(|f| f.file_path.contains("test"))
            .collect();
        assert!(!test_files.is_empty());
    }

    #[test]
    fn test_builder_pattern() {
        let generator = CodeGenBuilder::new(
            MachineBuilder::<GameContext, GameEvent>::new()
                .state("idle")
                    .on(GameEvent::Start, "playing")
                .state("playing")
                    .on(GameEvent::Stop, "idle")
                .initial("idle")
                .build()
        )
        .with_language(ProgrammingLanguage::TypeScript)
        .with_output_directory("builder_test".to_string())
        .with_tests(false)
        .with_documentation(true)
        .with_metadata("test".to_string(), "builder".to_string())
        .build();

        let config = generator.config;
        assert_eq!(config.target_languages.len(), 1);
        assert_eq!(config.output_directory, "builder_test");
        assert!(!config.include_tests);
        assert!(config.include_documentation);
        assert_eq!(config.metadata.get("test"), Some(&"builder".to_string()));
    }

    #[test]
    fn test_multiple_languages() {
        let config = CodeGenConfig {
            enabled: true,
            target_languages: vec![
                ProgrammingLanguage::Rust,
                ProgrammingLanguage::TypeScript,
                ProgrammingLanguage::Python,
            ],
            output_directory: "multi_lang".to_string(),
            include_tests: true,
            include_documentation: true,
            metadata: HashMap::new(),
        };

        let generator = MachineBuilder::<GameContext, GameEvent>::new()
            .state("idle")
                .on(GameEvent::Start, "playing")
            .state("playing")
                .on(GameEvent::Stop, "idle")
            .initial("idle")
            .build_with_code_generation(config);

        let files = generator.generate_code().unwrap();
        
        // Should have files for all three languages
        let rust_files: Vec<_> = files.iter()
            .filter(|f| matches!(f.language, ProgrammingLanguage::Rust))
            .collect();
        let ts_files: Vec<_> = files.iter()
            .filter(|f| matches!(f.language, ProgrammingLanguage::TypeScript))
            .collect();
        let py_files: Vec<_> = files.iter()
            .filter(|f| matches!(f.language, ProgrammingLanguage::Python))
            .collect();
        
        assert!(!rust_files.is_empty());
        assert!(!ts_files.is_empty());
        assert!(!py_files.is_empty());
    }

    #[test]
    fn test_generated_content() {
        let generator = MachineBuilder::<GameContext, GameEvent>::new()
            .state("idle")
                .on(GameEvent::Start, "playing")
            .state("playing")
                .on(GameEvent::Stop, "idle")
            .initial("idle")
            .build_codegen();

        let files = generator.generate_code().unwrap();
        
        // Check Rust file content
        let rust_file = files.iter()
            .find(|f| matches!(f.language, ProgrammingLanguage::Rust) && f.file_path.contains("state_machine.rs"))
            .unwrap();
        
        assert!(rust_file.content.contains("pub struct StateMachine"));
        assert!(rust_file.content.contains("pub enum State"));
        assert!(rust_file.content.contains("pub enum StateEvent"));
        assert!(rust_file.content.contains("State::idle"));
        assert!(rust_file.content.contains("State::playing"));
        assert!(rust_file.content.contains("StateEvent::Start"));
        assert!(rust_file.content.contains("StateEvent::Stop"));
        
        // Check TypeScript file content
        let ts_file = files.iter()
            .find(|f| matches!(f.language, ProgrammingLanguage::TypeScript) && f.file_path.contains("StateMachine.ts"))
            .unwrap();
        
        assert!(ts_file.content.contains("export class StateMachine"));
        assert!(ts_file.content.contains("export enum State"));
        assert!(ts_file.content.contains("export enum StateEvent"));
        assert!(ts_file.content.contains("State.idle"));
        assert!(ts_file.content.contains("State.playing"));
        assert!(ts_file.content.contains("StateEvent.Start"));
        assert!(ts_file.content.contains("StateEvent.Stop"));
    }

    #[test]
    fn test_index_generation() {
        let generator = MachineBuilder::<GameContext, GameEvent>::new()
            .state("idle")
                .on(GameEvent::Start, "playing")
            .state("playing")
                .on(GameEvent::Stop, "idle")
            .initial("idle")
            .build_codegen();

        let index = generator.generate_index().unwrap();
        
        assert!(index.contains("# Generated Code Index"));
        assert!(index.contains("Generated code files:"));
        assert!(index.contains("Generation Info"));
        assert!(index.contains("Output directory"));
        assert!(index.contains("Languages"));
    }
}
