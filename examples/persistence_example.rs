use leptos_state::machine::*;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct GameContext {
    player_health: i32,
    player_level: u32,
    coins: u32,
    score: u64,
    achievements: Vec<String>,
    last_save_time: u64,
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
            last_save_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            game_session_id: uuid::Uuid::new_v4().to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
enum GameEvent {
    StartGame,
    CollectCoin,
    TakeDamage(i32),
    LevelUp,
    SaveGame,
    LoadGame,
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
            GameEvent::LoadGame => "load_game",
            GameEvent::QuitGame => "quit_game",
            GameEvent::UnlockAchievement(_) => "unlock_achievement",
        }
    }
}

fn main() {
    println!("=== State Machine Persistence Example ===");
    
    // Create a game state machine with persistence
    let machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("menu")
            .on_entry_log("Entered game menu")
            .on_entry_fn(|ctx, _| {
                ctx.last_save_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
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
                    ctx.last_save_time = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    println!("💾 Game saved at timestamp: {}", ctx.last_save_time);
                })
                .action_log("Game saved")
            .on(GameEvent::QuitGame, "menu")
                .action_fn(|ctx, _| {
                    println!("👋 Quitting game. Final score: {}", ctx.score);
                })
                .action_log("Game quit")
        .build_persistent()
        .with_storage(Arc::new(persistence::MemoryStorage::new()))
        .initialize()
        .unwrap();
    
    println!("✓ Persistent machine created and initialized");
    
    // Test initial state
    let initial_state = machine.current_state().unwrap();
    println!("✓ Initial state: {:?}", initial_state.value());
    println!("✓ Initial context: health={}, level={}, coins={}, score={}", 
             initial_state.context().player_health,
             initial_state.context().player_level,
             initial_state.context().coins,
             initial_state.context().score);
    
    // Test game progression with persistence
    println!("\n--- Testing Game Progression with Persistence ---");
    
    let mut game_machine = machine;
    
    // Start the game
    let playing_state = game_machine.transition(GameEvent::StartGame).unwrap();
    println!("Current state: {:?}", playing_state.value());
    
    // Collect some coins
    let state_after_coins = game_machine.transition(GameEvent::CollectCoin).unwrap();
    let state_after_coins = game_machine.transition(GameEvent::CollectCoin).unwrap();
    let state_after_coins = game_machine.transition(GameEvent::CollectCoin).unwrap();
    println!("After collecting coins: coins={}, score={}", 
             state_after_coins.context().coins,
             state_after_coins.context().score);
    
    // Take some damage
    let damaged_state = game_machine.transition(GameEvent::TakeDamage(20)).unwrap();
    println!("After taking damage: health={}", damaged_state.context().player_health);
    
    // Level up
    let leveled_state = game_machine.transition(GameEvent::LevelUp).unwrap();
    println!("After leveling up: level={}, health={}", 
             leveled_state.context().player_level,
             leveled_state.context().player_health);
    
    // Save the game
    let saved_state = game_machine.transition(GameEvent::SaveGame).unwrap();
    println!("Game saved at: {}", saved_state.context().last_save_time);
    
    // Test persistence info
    let persistence_info = game_machine.persistence_info();
    println!("Persistence info: enabled={}, auto_save={}, auto_restore={}, storage_key={}", 
             persistence_info.enabled,
             persistence_info.auto_save,
             persistence_info.auto_restore,
             persistence_info.storage_key);
    
    // Test manual save
    println!("\n--- Testing Manual Save ---");
    game_machine.save().unwrap();
    println!("✓ Game manually saved");
    
    // Create a new machine and test restoration
    println!("\n--- Testing State Restoration ---");
    
    let new_machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("menu")
            .on(GameEvent::StartGame, "playing")
        .state("playing")
            .on(GameEvent::CollectCoin, "playing")
            .on(GameEvent::TakeDamage(0), "playing")
            .on(GameEvent::LevelUp, "playing")
            .on(GameEvent::SaveGame, "playing")
            .on(GameEvent::QuitGame, "menu")
        .build_persistent()
        .with_storage(Arc::new(persistence::MemoryStorage::new()))
        .initialize()
        .unwrap();
    
    let mut restored_machine = new_machine;
    
    // Restore the previous state
    restored_machine.restore().unwrap();
    let restored_state = restored_machine.current_state().unwrap();
    
    println!("✓ State restored successfully!");
    println!("Restored state: {:?}", restored_state.value());
    println!("Restored context: health={}, level={}, coins={}, score={}", 
             restored_state.context().player_health,
             restored_state.context().player_level,
             restored_state.context().coins,
             restored_state.context().score);
    
    // Verify the state was properly restored
    assert_eq!(restored_state.context().player_health, 100); // Should be restored from level up
    assert_eq!(restored_state.context().player_level, 2); // Should be level 2
    assert_eq!(restored_state.context().coins, 3); // Should have 3 coins
    assert_eq!(restored_state.context().score, 30); // Should have 30 score
    
    println!("✓ All state values correctly restored!");
    
    // Test backup functionality
    println!("\n--- Testing Backup Functionality ---");
    
    // Create a backup
    let backup_config = persistence::BackupConfig {
        max_backups: 3,
        auto_backup: true,
        backup_interval: 60, // 1 minute
        compress_backups: true,
    };
    
    let persistence_config = persistence::PersistenceConfig {
        enabled: true,
        storage_key: "game_backup_test".to_string(),
        auto_save: true,
        auto_restore: true,
        backup_config,
        ..Default::default()
    };
    
    let backup_machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("menu")
            .on(GameEvent::StartGame, "playing")
        .state("playing")
            .on(GameEvent::CollectCoin, "playing")
        .build_with_persistence(persistence_config)
        .with_storage(Arc::new(persistence::MemoryStorage::new()))
        .initialize()
        .unwrap();
    
    let mut backup_game = backup_machine;
    
    // Make some changes
    backup_game.transition(GameEvent::StartGame).unwrap();
    backup_game.transition(GameEvent::CollectCoin).unwrap();
    backup_game.transition(GameEvent::CollectCoin).unwrap();
    
    // Save (this should create a backup)
    backup_game.save().unwrap();
    
    // List backups
    let backup_info = backup_game.persistence_info();
    println!("Available backups: {}", backup_info.backups.len());
    for backup in &backup_info.backups {
        println!("  Backup: {} at timestamp {}", backup.key, backup.timestamp);
    }
    
    // Test custom persistence configuration
    println!("\n--- Testing Custom Persistence Configuration ---");
    
    let custom_config = persistence::PersistenceConfig {
        enabled: true,
        storage_key: "custom_game".to_string(),
        auto_save: false, // Manual save only
        auto_restore: true,
        max_size: 1024 * 1024, // 1MB
        compression_level: 0, // No compression
        encrypt: false, // No encryption
        backup_config: persistence::BackupConfig {
            max_backups: 2,
            auto_backup: false, // Manual backup only
            backup_interval: 3600,
            compress_backups: false,
        },
    };
    
    let custom_machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("menu")
            .on(GameEvent::StartGame, "playing")
        .state("playing")
            .on(GameEvent::CollectCoin, "playing")
        .build_with_persistence(custom_config)
        .with_storage(Arc::new(persistence::MemoryStorage::new()))
        .initialize()
        .unwrap();
    
    let mut custom_game = custom_machine;
    
    // Make changes without auto-save
    custom_game.transition(GameEvent::StartGame).unwrap();
    custom_game.transition(GameEvent::CollectCoin).unwrap();
    
    // Manual save
    custom_game.save().unwrap();
    println!("✓ Custom configuration machine saved manually");
    
    // Test persistence clearing
    println!("\n--- Testing Persistence Clearing ---");
    
    let clear_machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("menu")
            .on(GameEvent::StartGame, "playing")
        .state("playing")
            .on(GameEvent::CollectCoin, "playing")
        .build_persistent()
        .with_storage(Arc::new(persistence::MemoryStorage::new()))
        .initialize()
        .unwrap();
    
    let mut clear_game = clear_machine;
    
    // Make some changes and save
    clear_game.transition(GameEvent::StartGame).unwrap();
    clear_game.transition(GameEvent::CollectCoin).unwrap();
    clear_game.save().unwrap();
    
    // Clear persistence
    clear_game.clear_persistence().unwrap();
    println!("✓ All persisted data cleared");
    
    // Try to restore (should fail)
    match clear_game.restore() {
        Ok(_) => println!("⚠️ Unexpected: Restore succeeded after clearing"),
        Err(_) => println!("✓ Expected: Restore failed after clearing"),
    }
    
    println!("\n=== Persistence Example Completed ===");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_persistence_workflow() {
        let machine = MachineBuilder::<GameContext, GameEvent>::new()
            .state("menu")
                .on(GameEvent::StartGame, "playing")
            .state("playing")
                .on(GameEvent::CollectCoin, "playing")
            .build_persistent()
            .with_storage(Arc::new(persistence::MemoryStorage::new()))
            .initialize()
            .unwrap();
        
        let mut game = machine;
        
        // Start game
        game.transition(GameEvent::StartGame).unwrap();
        
        // Collect coin
        let state = game.transition(GameEvent::CollectCoin).unwrap();
        assert_eq!(state.context().coins, 1);
        
        // Save
        game.save().unwrap();
        
        // Create new machine and restore
        let new_machine = MachineBuilder::<GameContext, GameEvent>::new()
            .state("menu")
                .on(GameEvent::StartGame, "playing")
            .state("playing")
                .on(GameEvent::CollectCoin, "playing")
            .build_persistent()
            .with_storage(Arc::new(persistence::MemoryStorage::new()))
            .initialize()
            .unwrap();
        
        let mut new_game = new_machine;
        new_game.restore().unwrap();
        
        let restored_state = new_game.current_state().unwrap();
        assert_eq!(restored_state.context().coins, 1);
    }
}
