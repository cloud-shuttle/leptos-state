use leptos_state::machine::*;
use std::time::Duration;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct GameContext {
    player_health: i32,
    player_level: u32,
    coins: u32,
    score: u64,
    achievements: Vec<String>,
    game_session_id: String,
    external_data: Option<String>, // Data from external systems
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
            external_data: None,
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
    SyncWithServer, // Integration event
    LoadExternalData, // Integration event
    SendAnalytics, // Integration event
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
            GameEvent::SyncWithServer => "sync_with_server",
            GameEvent::LoadExternalData => "load_external_data",
            GameEvent::SendAnalytics => "send_analytics",
        }
    }
}

fn main() {
    println!("=== State Machine Integration Patterns Example ===");
    
    // Create a game state machine with integration capabilities
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
            .on(GameEvent::SyncWithServer, "playing")
                .action_fn(|ctx, _| {
                    println!("🔄 Syncing with server...");
                    // This would trigger integration events
                })
                .action_log("Server sync initiated")
            .on(GameEvent::LoadExternalData, "playing")
                .action_fn(|ctx, _| {
                    println!("📊 Loading external data...");
                    ctx.external_data = Some("External data loaded".to_string());
                })
                .action_log("External data loaded")
            .on(GameEvent::SendAnalytics, "playing")
                .action_fn(|ctx, _| {
                    println!("📈 Sending analytics data...");
                })
                .action_log("Analytics sent")
            .on(GameEvent::QuitGame, "menu")
                .action_fn(|ctx, _| {
                    println!("👋 Quitting game. Final score: {}", ctx.score);
                })
                .action_log("Game quit")
        .build_integrated();
    
    println!("✓ Integrated machine created");
    
    // Test 1: Basic Integration Setup
    println!("\n--- Test 1: Basic Integration Setup ---");
    
    // Create and register adapters
    let http_adapter = HttpApiAdapter::new("game_api".to_string(), "https://api.game.com".to_string())
        .with_header("Authorization".to_string(), "Bearer game_token".to_string())
        .with_timeout(Duration::from_secs(30));
    
    let db_adapter = DatabaseAdapter::new(
        "game_db".to_string(),
        "postgresql://localhost/game_db".to_string(),
        "game_events".to_string(),
    );
    
    let queue_adapter = MessageQueueAdapter::new(
        "analytics_queue".to_string(),
        "amqp://localhost".to_string(),
        "analytics_events".to_string(),
    );
    
    // Register adapters with the integration manager
    machine.register_adapter("api".to_string(), Box::new(http_adapter));
    machine.register_adapter("database".to_string(), Box::new(db_adapter));
    machine.register_adapter("queue".to_string(), Box::new(queue_adapter));
    
    println!("✓ Adapters registered");
    
    // Test 2: Integration Event Processing
    println!("\n--- Test 2: Integration Event Processing ---");
    
    // Create integration events
    let sync_event = IntegrationEvent {
        id: "sync_001".to_string(),
        event_type: "sync_with_server".to_string(),
        source: "game_client".to_string(),
        timestamp: std::time::Instant::now(),
        payload: r#"{"player_id": "123", "score": 100, "level": 5}"#.to_string(),
        metadata: HashMap::new(),
        priority: EventPriority::High,
    };
    
    let analytics_event = IntegrationEvent {
        id: "analytics_001".to_string(),
        event_type: "player_action".to_string(),
        source: "game_client".to_string(),
        timestamp: std::time::Instant::now(),
        payload: r#"{"action": "collect_coin", "timestamp": "2024-01-01T12:00:00Z"}"#.to_string(),
        metadata: HashMap::new(),
        priority: EventPriority::Normal,
    };
    
    let save_event = IntegrationEvent {
        id: "save_001".to_string(),
        event_type: "save_game".to_string(),
        source: "game_client".to_string(),
        timestamp: std::time::Instant::now(),
        payload: r#"{"game_state": {"health": 80, "coins": 15, "level": 3}}"#.to_string(),
        metadata: HashMap::new(),
        priority: EventPriority::Critical,
    };
    
    // Process incoming events
    println!("Processing incoming integration events...");
    let result1 = machine.process_incoming_event(sync_event.clone());
    let result2 = machine.process_incoming_event(analytics_event.clone());
    let result3 = machine.process_incoming_event(save_event.clone());
    
    println!("Sync event result: {}", if result1.is_ok() { "SUCCESS" } else { "FAILED" });
    println!("Analytics event result: {}", if result2.is_ok() { "SUCCESS" } else { "FAILED" });
    println!("Save event result: {}", if result3.is_ok() { "SUCCESS" } else { "FAILED" });
    
    // Test 3: Event Routing Configuration
    println!("\n--- Test 3: Event Routing Configuration ---");
    
    // Create routing rules
    let routing_rules = vec![
        RoutingRule {
            name: "sync_events".to_string(),
            pattern: EventPattern {
                event_type: "sync".to_string(),
                source: Some("game_client".to_string()),
            },
            target: "api".to_string(),
            enabled: true,
        },
        RoutingRule {
            name: "analytics_events".to_string(),
            pattern: EventPattern {
                event_type: "analytics".to_string(),
                source: Some("game_client".to_string()),
            },
            target: "queue".to_string(),
            enabled: true,
        },
        RoutingRule {
            name: "save_events".to_string(),
            pattern: EventPattern {
                event_type: "save".to_string(),
                source: Some("game_client".to_string()),
            },
            target: "database".to_string(),
            enabled: true,
        },
    ];
    
    let event_routing = EventRoutingConfig {
        rules: routing_rules,
        default_route: Some("api".to_string()),
    };
    
    println!("Routing rules configured:");
    for rule in &event_routing.rules {
        println!("  {}: {} -> {}", rule.name, rule.pattern.event_type, rule.target);
    }
    
    // Test 4: Error Handling Strategies
    println!("\n--- Test 4: Error Handling Strategies ---");
    
    let error_handling_strategies = vec![
        ErrorHandlingStrategy::FailFast,
        ErrorHandlingStrategy::RetryWithBackoff,
        ErrorHandlingStrategy::ContinueWithFallback,
        ErrorHandlingStrategy::LogAndContinue,
    ];
    
    println!("Error handling strategies:");
    for strategy in &error_handling_strategies {
        match strategy {
            ErrorHandlingStrategy::FailFast => println!("  - Fail Fast: Stop on first error"),
            ErrorHandlingStrategy::RetryWithBackoff => println!("  - Retry with Backoff: Retry with exponential delay"),
            ErrorHandlingStrategy::ContinueWithFallback => println!("  - Continue with Fallback: Use fallback mechanism"),
            ErrorHandlingStrategy::LogAndContinue => println!("  - Log and Continue: Log error and continue processing"),
        }
    }
    
    // Test 5: Retry Configuration
    println!("\n--- Test 5: Retry Configuration ---");
    
    let retry_config = RetryConfig {
        max_retries: 5,
        initial_delay: Duration::from_secs(1),
        max_delay: Duration::from_secs(30),
        backoff_multiplier: 2.0,
    };
    
    println!("Retry configuration:");
    println!("  Max retries: {}", retry_config.max_retries);
    println!("  Initial delay: {:?}", retry_config.initial_delay);
    println!("  Max delay: {:?}", retry_config.max_delay);
    println!("  Backoff multiplier: {}", retry_config.backoff_multiplier);
    
    // Test 6: Integration Metrics
    println!("\n--- Test 6: Integration Metrics ---");
    
    let metrics = machine.get_metrics();
    println!("Integration metrics:");
    println!("  Incoming events: {}", metrics.incoming_events);
    println!("  Outgoing events: {}", metrics.outgoing_events);
    println!("  Total processing time: {:?}", metrics.total_processing_time);
    println!("  Errors: {}", metrics.errors);
    println!("  Retries: {}", metrics.retries);
    
    // Test 7: Custom Integration Configuration
    println!("\n--- Test 7: Custom Integration Configuration ---");
    
    let custom_config = IntegrationConfig {
        enabled: true,
        adapters: vec![
            IntegrationAdapter {
                name: "custom_api".to_string(),
                adapter_type: AdapterType::HttpApi,
                config: HashMap::new(),
                enabled: true,
            },
            IntegrationAdapter {
                name: "custom_db".to_string(),
                adapter_type: AdapterType::Database,
                config: HashMap::new(),
                enabled: true,
            },
        ],
        event_routing: EventRoutingConfig {
            rules: vec![
                RoutingRule {
                    name: "default_rule".to_string(),
                    pattern: EventPattern {
                        event_type: "default".to_string(),
                        source: None,
                    },
                    target: "custom_api".to_string(),
                    enabled: true,
                },
            ],
            default_route: Some("custom_api".to_string()),
        },
        error_handling: ErrorHandlingStrategy::RetryWithBackoff,
        retry_config: RetryConfig {
            max_retries: 3,
            initial_delay: Duration::from_secs(2),
            max_delay: Duration::from_secs(20),
            backoff_multiplier: 1.5,
        },
    };
    
    let custom_machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("menu")
            .on(GameEvent::StartGame, "playing")
        .state("playing")
            .on(GameEvent::CollectCoin, "playing")
            .on(GameEvent::QuitGame, "menu")
        .build_with_integration(custom_config);
    
    println!("Custom integration configuration created");
    
    // Test 8: Integration Builder Pattern
    println!("\n--- Test 8: Integration Builder Pattern ---");
    
    let built_machine = IntegrationBuilder::new(
        MachineBuilder::<GameContext, GameEvent>::new()
            .state("menu")
                .on(GameEvent::StartGame, "playing")
            .state("playing")
                .on(GameEvent::CollectCoin, "playing")
                .on(GameEvent::QuitGame, "menu")
            .build()
    )
    .with_adapter(IntegrationAdapter {
        name: "http_api".to_string(),
        adapter_type: AdapterType::HttpApi,
        config: HashMap::new(),
        enabled: true,
    })
    .with_adapter(IntegrationAdapter {
        name: "message_queue".to_string(),
        adapter_type: AdapterType::MessageQueue,
        config: HashMap::new(),
        enabled: true,
    })
    .with_error_handling(ErrorHandlingStrategy::LogAndContinue)
    .with_retry_config(RetryConfig {
        max_retries: 4,
        initial_delay: Duration::from_secs(1),
        max_delay: Duration::from_secs(25),
        backoff_multiplier: 2.5,
    })
    .build();
    
    println!("Integration builder configuration:");
    println!("  Adapters: {}", built_machine.config.adapters.len());
    println!("  Error handling: {:?}", built_machine.config.error_handling);
    println!("  Max retries: {}", built_machine.config.retry_config.max_retries);
    
    // Test 9: Event Priority Handling
    println!("\n--- Test 9: Event Priority Handling ---");
    
    let priority_events = vec![
        IntegrationEvent {
            id: "low_priority".to_string(),
            event_type: "background_sync".to_string(),
            source: "game_client".to_string(),
            timestamp: std::time::Instant::now(),
            payload: "{}".to_string(),
            metadata: HashMap::new(),
            priority: EventPriority::Low,
        },
        IntegrationEvent {
            id: "normal_priority".to_string(),
            event_type: "player_action".to_string(),
            source: "game_client".to_string(),
            timestamp: std::time::Instant::now(),
            payload: "{}".to_string(),
            metadata: HashMap::new(),
            priority: EventPriority::Normal,
        },
        IntegrationEvent {
            id: "high_priority".to_string(),
            event_type: "save_game".to_string(),
            source: "game_client".to_string(),
            timestamp: std::time::Instant::now(),
            payload: "{}".to_string(),
            metadata: HashMap::new(),
            priority: EventPriority::High,
        },
        IntegrationEvent {
            id: "critical_priority".to_string(),
            event_type: "error_report".to_string(),
            source: "game_client".to_string(),
            timestamp: std::time::Instant::now(),
            payload: "{}".to_string(),
            metadata: HashMap::new(),
            priority: EventPriority::Critical,
        },
    ];
    
    println!("Processing events with different priorities:");
    for event in priority_events {
        println!("  Processing {} (Priority: {:?})", event.id, event.priority);
        let result = built_machine.process_incoming_event(event);
        println!("    Result: {}", if result.is_ok() { "SUCCESS" } else { "FAILED" });
    }
    
    // Test 10: Integration Event Patterns
    println!("\n--- Test 10: Integration Event Patterns ---");
    
    let event_patterns = vec![
        EventPattern {
            event_type: "sync".to_string(),
            source: Some("game_client".to_string()),
        },
        EventPattern {
            event_type: "analytics".to_string(),
            source: Some("game_client".to_string()),
        },
        EventPattern {
            event_type: "save".to_string(),
            source: None, // Match any source
        },
    ];
    
    println!("Event patterns for routing:");
    for (i, pattern) in event_patterns.iter().enumerate() {
        println!("  Pattern {}: event_type='{}', source='{:?}'", 
                 i + 1, pattern.event_type, pattern.source);
    }
    
    // Test 11: Adapter Health Checks
    println!("\n--- Test 11: Adapter Health Checks ---");
    
    let adapters = vec![
        ("HTTP API", HttpApiAdapter::new("health_check_api".to_string(), "https://api.example.com".to_string())),
        ("Database", DatabaseAdapter::new("health_check_db".to_string(), "postgresql://localhost/test".to_string(), "test".to_string())),
        ("Message Queue", MessageQueueAdapter::new("health_check_queue".to_string(), "amqp://localhost".to_string(), "test".to_string())),
    ];
    
    println!("Adapter health checks:");
    for (name, adapter) in adapters {
        let health = adapter.is_healthy();
        println!("  {}: {}", name, if health { "HEALTHY" } else { "UNHEALTHY" });
    }
    
    // Test 12: Comprehensive Integration Workflow
    println!("\n--- Test 12: Comprehensive Integration Workflow ---");
    
    // Simulate a complete game session with integration
    let game_session_events = vec![
        ("Start Game", GameEvent::StartGame),
        ("Collect Coin", GameEvent::CollectCoin),
        ("Take Damage", GameEvent::TakeDamage(20)),
        ("Level Up", GameEvent::LevelUp),
        ("Sync with Server", GameEvent::SyncWithServer),
        ("Load External Data", GameEvent::LoadExternalData),
        ("Send Analytics", GameEvent::SendAnalytics),
        ("Save Game", GameEvent::SaveGame),
        ("Quit Game", GameEvent::QuitGame),
    ];
    
    println!("Simulating complete game session with integration:");
    let initial_state = machine.machine().initial_state();
    let mut current_state = initial_state;
    
    for (action_name, event) in game_session_events {
        println!("  {}", action_name);
        current_state = machine.machine().transition(&current_state, event);
        
        // Create corresponding integration event
        let integration_event = IntegrationEvent {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: event.event_type().to_string(),
            source: "game_client".to_string(),
            timestamp: std::time::Instant::now(),
            payload: format!("{{\"action\": \"{}\", \"state\": \"{}\"}}", 
                           action_name, current_state.value()),
            metadata: HashMap::new(),
            priority: EventPriority::Normal,
        };
        
        // Process integration event
        let result = machine.process_outgoing_event(integration_event);
        if result.is_err() {
            println!("    Integration failed: {:?}", result.err());
        }
    }
    
    // Final metrics
    let final_metrics = machine.get_metrics();
    println!("\nFinal integration metrics:");
    println!("  Total incoming events: {}", final_metrics.incoming_events);
    println!("  Total outgoing events: {}", final_metrics.outgoing_events);
    println!("  Total processing time: {:?}", final_metrics.total_processing_time);
    println!("  Total errors: {}", final_metrics.errors);
    println!("  Total retries: {}", final_metrics.retries);
    
    println!("\n=== Integration Patterns Example Completed ===");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_integration_workflow() {
        let machine = MachineBuilder::<GameContext, GameEvent>::new()
            .state("menu")
                .on(GameEvent::StartGame, "playing")
            .state("playing")
                .on(GameEvent::CollectCoin, "playing")
            .build_integrated();
        
        // Register adapters
        let http_adapter = HttpApiAdapter::new("test_api".to_string(), "https://api.example.com".to_string());
        machine.register_adapter("api".to_string(), Box::new(http_adapter));
        
        // Test integration event processing
        let event = IntegrationEvent {
            id: "test_event".to_string(),
            event_type: "test".to_string(),
            source: "test".to_string(),
            timestamp: std::time::Instant::now(),
            payload: "{}".to_string(),
            metadata: HashMap::new(),
            priority: EventPriority::Normal,
        };
        
        let result = machine.process_incoming_event(event);
        assert!(result.is_ok());
        
        let metrics = machine.get_metrics();
        assert_eq!(metrics.incoming_events, 1);
    }
    
    #[test]
    fn test_adapter_health_checks() {
        let http_adapter = HttpApiAdapter::new("test".to_string(), "https://api.example.com".to_string());
        let db_adapter = DatabaseAdapter::new("test".to_string(), "postgresql://localhost/test".to_string(), "test".to_string());
        let queue_adapter = MessageQueueAdapter::new("test".to_string(), "amqp://localhost".to_string(), "test".to_string());
        
        assert!(http_adapter.is_healthy());
        assert!(db_adapter.is_healthy());
        assert!(queue_adapter.is_healthy());
    }
    
    #[test]
    fn test_integration_builder() {
        let machine = MachineBuilder::<GameContext, GameEvent>::new()
            .state("menu")
                .on(GameEvent::StartGame, "playing")
            .state("playing")
                .on(GameEvent::CollectCoin, "playing")
            .build();
        
        let integration_manager = IntegrationBuilder::new(machine)
            .with_adapter(IntegrationAdapter {
                name: "test".to_string(),
                adapter_type: AdapterType::HttpApi,
                config: HashMap::new(),
                enabled: true,
            })
            .with_error_handling(ErrorHandlingStrategy::RetryWithBackoff)
            .with_retry_config(RetryConfig {
                max_retries: 3,
                initial_delay: Duration::from_secs(1),
                max_delay: Duration::from_secs(10),
                backoff_multiplier: 2.0,
            })
            .build();
        
        let config = integration_manager.config;
        assert!(config.enabled);
        assert_eq!(config.adapters.len(), 1);
        assert_eq!(config.error_handling, ErrorHandlingStrategy::RetryWithBackoff);
        assert_eq!(config.retry_config.max_retries, 3);
    }
}
