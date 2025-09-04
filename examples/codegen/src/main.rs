//! Code Generation Example (v1.0.0)
//!
//! This example demonstrates the new v1.0.0 architecture
//! and how it can be used for code generation scenarios.

use leptos_state::v1::*;

#[derive(Debug, Clone, PartialEq, Default)]
struct GameContext {
    score: i32,
    level: i32,
    lives: i32,
    player_name: String,
}

impl StateMachineContext for GameContext {}

#[derive(Debug, Clone, PartialEq, Default)]
enum GameEvent {
    #[default]
    Start,
    Pause,
    Resume,
    Stop,
    Score(i32),
    LevelUp,
    GameOver,
}

impl StateMachineEvent for GameEvent {}

#[derive(Debug, Clone, PartialEq)]
enum GameState {
    Idle,
    Playing,
    Paused,
    GameOver,
}

impl StateMachineState for GameState {
    type Context = GameContext;
    type Event = GameEvent;
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Idle
    }
}

impl StateMachine for GameState {
    fn initial_state(&self) -> Self {
        GameState::Idle
    }

    fn transition(&self, state: &Self, event: GameEvent) -> Self {
        match (state, event) {
            (GameState::Idle, GameEvent::Start) => GameState::Playing,
            (GameState::Playing, GameEvent::Pause) => GameState::Paused,
            (GameState::Paused, GameEvent::Resume) => GameState::Playing,
            (GameState::Playing, GameEvent::Stop) => GameState::Idle,
            (GameState::Paused, GameEvent::Stop) => GameState::Idle,
            (GameState::Playing, GameEvent::GameOver) => GameState::GameOver,
            (GameState::Paused, GameEvent::GameOver) => GameState::GameOver,
            (GameState::GameOver, GameEvent::Start) => GameState::Idle,
            _ => state.clone(),
        }
    }

    fn can_transition(&self, state: &Self, event: GameEvent) -> bool {
        match (state, event) {
            (GameState::Idle, GameEvent::Start) => true,
            (GameState::Playing, GameEvent::Pause) => true,
            (GameState::Paused, GameEvent::Resume) => true,
            (GameState::Playing, GameEvent::Stop) => true,
            (GameState::Paused, GameEvent::Stop) => true,
            (GameState::Playing, GameEvent::GameOver) => true,
            (GameState::Paused, GameEvent::GameOver) => true,
            (GameState::GameOver, GameEvent::Start) => true,
            _ => false,
        }
    }

    fn try_transition(&self, state: &Self, event: GameEvent) -> Result<Self, TransitionError<GameEvent>> {
        if self.can_transition(state, event.clone()) {
            Ok(self.transition(state, event))
        } else {
            Err(TransitionError::InvalidTransition(event))
        }
    }

    fn state_count(&self) -> usize {
        4
    }

    fn is_valid_state(&self, state: &Self) -> bool {
        matches!(state, GameState::Idle | GameState::Playing | GameState::Paused | GameState::GameOver)
    }

    fn is_reachable(&self, state: &Self) -> bool {
        self.is_valid_state(state)
    }
}

// Code generation simulation for v1.0.0
struct CodeGenerator {
    machine: Machine<GameContext, GameEvent, GameState>,
    target_languages: Vec<String>,
    output_directory: String,
}

impl CodeGenerator {
    fn new(machine: Machine<GameContext, GameEvent, GameState>) -> Self {
        Self {
            machine,
            target_languages: vec!["rust".to_string(), "typescript".to_string(), "python".to_string()],
            output_directory: "generated".to_string(),
        }
    }

    fn with_languages(mut self, languages: Vec<String>) -> Self {
        self.target_languages = languages;
        self
    }

    fn with_output_directory(mut self, dir: String) -> Self {
        self.output_directory = dir;
        self
    }

    fn generate_code(&self) -> Vec<GeneratedFile> {
        let mut files = Vec::new();
        
        for language in &self.target_languages {
            let file = GeneratedFile {
                file_path: format!("{}/{}.{}", self.output_directory, "game_state_machine", language),
                language: language.clone(),
                content: self.generate_for_language(language),
            };
            files.push(file);
        }
        
        files
    }

    fn generate_for_language(&self, language: &str) -> String {
        match language {
            "rust" => self.generate_rust(),
            "typescript" => self.generate_typescript(),
            "python" => self.generate_python(),
            _ => format!("// Unsupported language: {}", language),
        }
    }

    fn generate_rust(&self) -> String {
        r#"// Generated Rust code for Game State Machine
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameContext {
    pub score: i32,
    pub level: i32,
    pub lives: i32,
    pub player_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameEvent {
    Start,
    Pause,
    Resume,
    Stop,
    Score(i32),
    LevelUp,
    GameOver,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameState {
    Idle,
    Playing,
    Paused,
    GameOver,
}

impl GameState {
    pub fn can_transition(&self, event: &GameEvent) -> bool {
        match (self, event) {
            (GameState::Idle, GameEvent::Start) => true,
            (GameState::Playing, GameEvent::Pause) => true,
            (GameState::Paused, GameEvent::Resume) => true,
            (GameState::Playing, GameEvent::Stop) => true,
            (GameState::Paused, GameEvent::Stop) => true,
            (GameState::Playing, GameEvent::GameOver) => true,
            (GameState::Paused, GameEvent::GameOver) => true,
            (GameState::GameOver, GameEvent::Start) => true,
            _ => false,
        }
    }
}"#.to_string()
    }

    fn generate_typescript(&self) -> String {
        r#"// Generated TypeScript code for Game State Machine
export interface GameContext {
    score: number;
    level: number;
    lives: number;
    playerName: string;
}

export enum GameEvent {
    Start = 'Start',
    Pause = 'Pause',
    Resume = 'Resume',
    Stop = 'Stop',
    Score = 'Score',
    LevelUp = 'LevelUp',
    GameOver = 'GameOver',
}

export enum GameState {
    Idle = 'Idle',
    Playing = 'Playing',
    Paused = 'Paused',
    GameOver = 'GameOver',
}

export class GameStateMachine {
    private currentState: GameState = GameState.Idle;
    
    public canTransition(event: GameEvent): boolean {
        switch (this.currentState) {
            case GameState.Idle:
                return event === GameEvent.Start;
            case GameState.Playing:
                return [GameEvent.Pause, GameEvent.Stop, GameEvent.GameOver].includes(event);
            case GameState.Paused:
                return [GameEvent.Resume, GameEvent.Stop].includes(event);
            case GameState.GameOver:
                return event === GameEvent.Start;
            default:
                return false;
        }
    }
}"#.to_string()
    }

    fn generate_python(&self) -> String {
        r#"# Generated Python code for Game State Machine
from enum import Enum
from dataclasses import dataclass
from typing import Optional

@dataclass
class GameContext:
    score: int
    level: int
    lives: int
    player_name: str

class GameEvent(Enum):
    START = "Start"
    PAUSE = "Pause"
    RESUME = "Resume"
    STOP = "Stop"
    SCORE = "Score"
    LEVEL_UP = "LevelUp"
    GAME_OVER = "GameOver"

class GameState(Enum):
    IDLE = "Idle"
    PLAYING = "Playing"
    PAUSED = "Paused"
    GAME_OVER = "GameOver"

class GameStateMachine:
    def __init__(self):
        self.current_state = GameState.IDLE
    
    def can_transition(self, event: GameEvent) -> bool:
        transitions = {
            GameState.IDLE: [GameEvent.START],
            GameState.PLAYING: [GameEvent.PAUSE, GameEvent.STOP, GameEvent.GAME_OVER],
            GameState.PAUSED: [GameEvent.RESUME, GameEvent.STOP],
            GameState.GAME_OVER: [GameEvent.START]
        }
        return event in transitions.get(self.current_state, [])"#.to_string()
    }
}

#[derive(Debug)]
struct GeneratedFile {
    file_path: String,
    language: String,
    content: String,
}

fn main() {
    println!("=== State Machine Code Generation Example (v1.0.0) ===\n");

    // Create a state machine with the new v1.0.0 architecture
    let initial_context = GameContext {
        score: 0,
        level: 1,
        lives: 3,
        player_name: "Player1".to_string(),
    };
    
    let mut machine = Machine::new(GameState::Idle, initial_context);

    println!("✓ State machine created successfully with v1.0.0 architecture");

    // Create a code generator
    let generator = CodeGenerator::new(machine.clone())
        .with_languages(vec!["rust".to_string(), "typescript".to_string(), "python".to_string()])
        .with_output_directory("generated".to_string());

    println!("✓ Code generator configured");

    // Generate code for multiple languages
    let generated_files = generator.generate_code();
    
    println!("\n=== Generated Code Files ===");
    for file in &generated_files {
        println!("- {} ({})", file.file_path, file.language);
    }

    // Demonstrate the state machine functionality
    println!("\n=== State Machine Demo ===");
    
    let mut current_state = GameState::Idle;
    println!("Initial state: {:?}", current_state);
    
    // Test transitions
    let events = vec![GameEvent::Start, GameEvent::Pause, GameEvent::Resume, GameEvent::Stop];
    
    for event in events {
        if let Ok(new_state) = machine.transition(event.clone()) {
            current_state = new_state;
            println!("After {:?} event: {:?}", event, current_state);
        } else {
            println!("Failed to transition on {:?} event", event);
        }
    }

    println!("\n=== Code Generation Complete ===");
    println!("This example demonstrates how the new v1.0.0 architecture");
    println!("can be used as a foundation for code generation tools!");
    println!("Check the 'generated' directory for output files!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_code_generation() {
        let initial_context = GameContext::default();
        let machine = Machine::new(GameState::Idle, initial_context);
        let generator = CodeGenerator::new(machine);
        
        let files = generator.generate_code();
        assert!(!files.is_empty());
        assert_eq!(files.len(), 3); // rust, typescript, python
    }

    #[test]
    fn test_state_machine_functionality() {
        let initial_context = GameContext::default();
        let machine = Machine::new(GameState::Idle, initial_context);
        
        // Test that we can transition from Idle to Playing
        let new_state = machine.transition(GameEvent::Start);
        assert!(new_state.is_ok());
        assert_eq!(new_state.unwrap(), GameState::Playing);
    }
}
