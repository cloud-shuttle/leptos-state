use leptos_state::machine::*;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
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

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
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
    println!("=== State Machine Visualization Example ===");
    
    // Create a game state machine with visualization
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
        .build_visualized();
    
    println!("✓ Visualized machine created");
    
    // Test initial state
    let initial_state = machine.machine().initial_state();
    println!("✓ Initial state: {:?}", initial_state.value());
    
    // Test game progression with visualization
    println!("\n--- Testing Game Progression with Visualization ---");
    
    // Start the game
    let playing_state = machine.transition(&initial_state, GameEvent::StartGame);
    println!("Current state: {:?}", playing_state.value());
    
    // Collect some coins
    let state_after_coins = machine.transition(&playing_state, GameEvent::CollectCoin);
    let state_after_coins = machine.transition(&state_after_coins, GameEvent::CollectCoin);
    let state_after_coins = machine.transition(&state_after_coins, GameEvent::CollectCoin);
    println!("After collecting coins: coins={}, score={}", 
             state_after_coins.context().coins,
             state_after_coins.context().score);
    
    // Take some damage
    let damaged_state = machine.transition(&state_after_coins, GameEvent::TakeDamage(20));
    println!("After taking damage: health={}", damaged_state.context().player_health);
    
    // Level up
    let leveled_state = machine.transition(&damaged_state, GameEvent::LevelUp);
    println!("After leveling up: level={}, health={}", 
             leveled_state.context().player_level,
             leveled_state.context().player_health);
    
    // Save the game
    let saved_state = machine.transition(&leveled_state, GameEvent::SaveGame);
    
    // Test visualization features
    println!("\n--- Testing Visualization Features ---");
    
    // Get real-time state information
    let state_info = machine.get_state_info();
    println!("Visualization stats:");
    println!("  Total transitions: {}", state_info.stats.total_transitions);
    println!("  Total snapshots: {}", state_info.stats.total_snapshots);
    println!("  Uptime: {:?}", state_info.stats.uptime);
    println!("  Average transition time: {:?}", state_info.stats.average_transition_time);
    println!("  Current state: {:?}", state_info.stats.current_state);
    
    // Show recent transitions
    println!("\nRecent transitions:");
    for (i, transition) in state_info.recent_transitions.iter().enumerate() {
        println!("  {}. {} -> {} (event: {})", 
                 i + 1,
                 transition.from_state,
                 transition.to_state,
                 transition.event.event_type());
    }
    
    // Test diagram export in different formats
    println!("\n--- Testing Diagram Export ---");
    
    // Export as DOT format
    let dot_diagram = machine.export_diagram(ExportFormat::Dot).unwrap();
    println!("DOT Diagram:");
    println!("{}", dot_diagram);
    
    // Export as Mermaid format
    let mermaid_diagram = machine.export_diagram(ExportFormat::Mermaid).unwrap();
    println!("\nMermaid Diagram:");
    println!("{}", mermaid_diagram);
    
    // Export as JSON format
    let json_diagram = machine.export_diagram(ExportFormat::Json).unwrap();
    println!("\nJSON Diagram (first 500 chars):");
    println!("{}", &json_diagram[..json_diagram.len().min(500)]);
    
    // Test time travel debugging
    println!("\n--- Testing Time Travel Debugging ---");
    
    let time_travel_position = state_info.time_travel_position;
    println!("Time travel position:");
    println!("  Current index: {}", time_travel_position.current_index);
    println!("  Total snapshots: {}", time_travel_position.total_snapshots);
    println!("  Can go back: {}", time_travel_position.can_go_back);
    println!("  Can go forward: {}", time_travel_position.can_go_forward);
    
    // Test time travel operations
    let monitor = machine.monitor();
    
    // Go back in time
    if let Some(snapshot) = monitor.go_back() {
        println!("✓ Went back in time to: {:?}", snapshot.state.value());
        println!("  Context: coins={}, score={}", 
                 snapshot.state.context().coins,
                 snapshot.state.context().score);
    }
    
    // Go forward in time
    if let Some(snapshot) = monitor.go_forward() {
        println!("✓ Went forward in time to: {:?}", snapshot.state.value());
        println!("  Context: coins={}, score={}", 
                 snapshot.state.context().coins,
                 snapshot.state.context().score);
    }
    
    // Go to the beginning
    if let Some(snapshot) = monitor.go_to_start() {
        println!("✓ Went to the beginning: {:?}", snapshot.state.value());
    }
    
    // Go to the end
    if let Some(snapshot) = monitor.go_to_end() {
        println!("✓ Went to the end: {:?}", snapshot.state.value());
    }
    
    // Test custom visualization configuration
    println!("\n--- Testing Custom Visualization Configuration ---");
    
    let custom_config = VisualizationConfig {
        enabled: true,
        update_interval: 50, // 50ms
        max_history: 20,
        capture_snapshots: true,
        enable_time_travel: true,
        show_transitions: true,
        show_context_changes: true,
        show_actions: true,
        show_guards: true,
        export_format: ExportFormat::Mermaid,
    };
    
    let custom_machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("menu")
            .on(GameEvent::StartGame, "playing")
        .state("playing")
            .on(GameEvent::CollectCoin, "playing")
            .on(GameEvent::TakeDamage(0), "playing")
            .on(GameEvent::LevelUp, "playing")
            .on(GameEvent::SaveGame, "playing")
            .on(GameEvent::QuitGame, "menu")
        .build_with_visualization(custom_config);
    
    let custom_initial = custom_machine.machine().initial_state();
    
    // Make some transitions
    let custom_playing = custom_machine.transition(&custom_initial, GameEvent::StartGame);
    let custom_coin = custom_machine.transition(&custom_playing, GameEvent::CollectCoin);
    let custom_damage = custom_machine.transition(&custom_coin, GameEvent::TakeDamage(15));
    
    // Get custom machine stats
    let custom_info = custom_machine.get_state_info();
    println!("Custom machine stats:");
    println!("  Total transitions: {}", custom_info.stats.total_transitions);
    println!("  Max history: {}", custom_machine.config().max_history);
    println!("  Export format: {:?}", custom_machine.config().export_format);
    
    // Test visualization with different export formats
    println!("\n--- Testing Different Export Formats ---");
    
    // Test SVG export (placeholder)
    let svg_diagram = custom_machine.export_diagram(ExportFormat::Svg).unwrap();
    println!("SVG Diagram: {}", svg_diagram);
    
    // Test PNG export (placeholder)
    let png_diagram = custom_machine.export_diagram(ExportFormat::Png).unwrap();
    println!("PNG Diagram: {}", png_diagram);
    
    // Test visualization statistics
    println!("\n--- Testing Visualization Statistics ---");
    
    let visualizer = machine.visualizer();
    let stats = visualizer.get_stats();
    
    println!("Detailed statistics:");
    println!("  Total transitions: {}", stats.total_transitions);
    println!("  Total snapshots: {}", stats.total_snapshots);
    println!("  Uptime: {:?}", stats.uptime);
    println!("  Average transition time: {:?}", stats.average_transition_time);
    println!("  Current state: {:?}", stats.current_state);
    
    // Test recent transitions and snapshots
    let recent_transitions = visualizer.recent_transitions(3);
    println!("\nLast 3 transitions:");
    for (i, transition) in recent_transitions.iter().enumerate() {
        println!("  {}. {} -> {} (duration: {:?})", 
                 i + 1,
                 transition.from_state,
                 transition.to_state,
                 transition.duration);
    }
    
    let recent_snapshots = visualizer.recent_snapshots(3);
    println!("\nLast 3 snapshots:");
    for (i, snapshot) in recent_snapshots.iter().enumerate() {
        println!("  {}. {:?} at {:?}", 
                 i + 1,
                 snapshot.state.value(),
                 snapshot.uptime);
    }
    
    // Test history clearing
    println!("\n--- Testing History Clearing ---");
    
    let before_clear = visualizer.get_stats();
    println!("Before clearing: {} transitions, {} snapshots", 
             before_clear.total_transitions, 
             before_clear.total_snapshots);
    
    visualizer.clear_history();
    
    let after_clear = visualizer.get_stats();
    println!("After clearing: {} transitions, {} snapshots", 
             after_clear.total_transitions, 
             after_clear.total_snapshots);
    
    println!("\n=== Visualization Example Completed ===");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_visualization_workflow() {
        let machine = MachineBuilder::<GameContext, GameEvent>::new()
            .state("menu")
                .on(GameEvent::StartGame, "playing")
            .state("playing")
                .on(GameEvent::CollectCoin, "playing")
            .build_visualized();
        
        let initial_state = machine.machine().initial_state();
        
        // Test transition with visualization
        let new_state = machine.transition(&initial_state, GameEvent::StartGame);
        assert_eq!(*new_state.value(), StateValue::Simple("playing".to_string()));
        
        // Test state info
        let state_info = machine.get_state_info();
        assert_eq!(state_info.stats.total_transitions, 1);
        
        // Test diagram export
        let diagram = machine.export_diagram(ExportFormat::Dot).unwrap();
        assert!(diagram.contains("digraph StateMachine"));
        assert!(diagram.contains("menu"));
        assert!(diagram.contains("playing"));
    }
    
    #[test]
    fn test_time_travel() {
        let machine = MachineBuilder::<GameContext, GameEvent>::new()
            .state("menu")
                .on(GameEvent::StartGame, "playing")
            .state("playing")
                .on(GameEvent::CollectCoin, "playing")
            .build_visualized();
        
        let initial_state = machine.machine().initial_state();
        
        // Make some transitions
        let state1 = machine.transition(&initial_state, GameEvent::StartGame);
        let state2 = machine.transition(&state1, GameEvent::CollectCoin);
        
        let monitor = machine.monitor();
        
        // Test time travel
        let snapshot = monitor.go_back().unwrap();
        assert_eq!(*snapshot.state.value(), StateValue::Simple("menu".to_string()));
        
        let snapshot = monitor.go_forward().unwrap();
        assert_eq!(*snapshot.state.value(), StateValue::Simple("playing".to_string()));
    }
}
