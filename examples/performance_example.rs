use leptos_state::machine::*;
use std::time::Duration;
use std::thread;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct GameContext {
    player_health: i32,
    player_level: u32,
    coins: u32,
    score: u64,
    achievements: Vec<String>,
    game_session_id: String,
    expensive_data: Option<String>, // Simulate expensive data
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
            expensive_data: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum GameEvent {
    StartGame,
    CollectCoin,
    TakeDamage(i32),
    LevelUp,
    SaveGame,
    QuitGame,
    UnlockAchievement(String),
    LoadExpensiveData,
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
            GameEvent::LoadExpensiveData => "load_expensive_data",
        }
    }
}

fn main() {
    println!("=== State Machine Performance Optimization Example ===");
    
    // Create a game state machine with performance optimization
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
            .on(GameEvent::LoadExpensiveData, "playing")
                .action_fn(|ctx, _| {
                    // Simulate expensive operation
                    thread::sleep(Duration::from_millis(100));
                    ctx.expensive_data = Some("Expensive data loaded".to_string());
                    println!("📊 Expensive data loaded!");
                })
                .action_log("Expensive data loaded")
            .on(GameEvent::QuitGame, "menu")
                .action_fn(|ctx, _| {
                    println!("👋 Quitting game. Final score: {}", ctx.score);
                })
                .action_log("Game quit")
        .build_optimized();
    
    println!("✓ Optimized machine created");
    
    // Test 1: Basic Performance Optimization
    println!("\n--- Test 1: Basic Performance Optimization ---");
    
    let initial_state = machine.machine().initial_state();
    
    // First transition (cache miss)
    let start_time = std::time::Instant::now();
    let playing_state = machine.transition(&initial_state, GameEvent::StartGame);
    let first_duration = start_time.elapsed();
    println!("First transition (cache miss): {:?}", first_duration);
    
    // Second transition (should be cached)
    let start_time = std::time::Instant::now();
    let playing_state2 = machine.transition(&initial_state, GameEvent::StartGame);
    let second_duration = start_time.elapsed();
    println!("Second transition (cache hit): {:?}", second_duration);
    
    // Verify caching worked
    if second_duration < first_duration {
        println!("✅ Caching is working! Second transition was faster.");
    } else {
        println!("⚠️ Caching may not be working as expected.");
    }
    
    // Test 2: Performance Metrics
    println!("\n--- Test 2: Performance Metrics ---");
    
    let metrics = machine.get_performance_metrics();
    println!("Performance metrics:");
    println!("  Total transitions: {}", metrics.total_transitions);
    println!("  Cache hits: {}", metrics.cache_hits);
    println!("  Cache misses: {}", metrics.cache_misses);
    println!("  Cache hit ratio: {:.2}%", metrics.cache_hit_ratio * 100.0);
    println!("  Average transition time: {:?}", metrics.avg_transition_time);
    println!("  Maximum transition time: {:?}", metrics.max_transition_time);
    println!("  Minimum transition time: {:?}", metrics.min_transition_time);
    println!("  Total execution time: {:?}", metrics.total_execution_time);
    println!("  Memory usage: {} bytes", metrics.memory_usage);
    println!("  Allocations: {}", metrics.allocations);
    println!("  Deallocations: {}", metrics.deallocations);
    
    // Test 3: Lazy Evaluation
    println!("\n--- Test 3: Lazy Evaluation ---");
    
    // Create a lazy evaluator for expensive data
    let mut expensive_data = LazyEvaluator::new(|| {
        println!("🔄 Computing expensive data...");
        thread::sleep(Duration::from_millis(200)); // Simulate expensive computation
        "Computed expensive data".to_string()
    });
    
    println!("Lazy evaluator created");
    println!("Is evaluated: {}", expensive_data.is_evaluated());
    
    // First access (triggers evaluation)
    let start_time = std::time::Instant::now();
    let data = expensive_data.get();
    let eval_duration = start_time.elapsed();
    println!("First access (evaluation): {:?} - {}", eval_duration, data);
    println!("Is evaluated: {}", expensive_data.is_evaluated());
    
    // Second access (cached)
    let start_time = std::time::Instant::now();
    let data2 = expensive_data.get();
    let cache_duration = start_time.elapsed();
    println!("Second access (cached): {:?} - {}", cache_duration, data2);
    
    if cache_duration < eval_duration {
        println!("✅ Lazy evaluation is working! Cached access was faster.");
    }
    
    // Test 4: Performance Bottlenecks
    println!("\n--- Test 4: Performance Bottlenecks ---");
    
    // Perform some transitions to generate bottlenecks
    let coin_state = machine.transition(&playing_state, GameEvent::CollectCoin);
    let damage_state = machine.transition(&coin_state, GameEvent::TakeDamage(20));
    let level_state = machine.transition(&damage_state, GameEvent::LevelUp);
    
    // Load expensive data (simulates bottleneck)
    let expensive_state = machine.transition(&level_state, GameEvent::LoadExpensiveData);
    
    let updated_metrics = machine.get_performance_metrics();
    
    if !updated_metrics.bottlenecks.is_empty() {
        println!("Performance bottlenecks detected:");
        for bottleneck in &updated_metrics.bottlenecks {
            println!("  Type: {:?}", bottleneck.bottleneck_type);
            println!("  Description: {}", bottleneck.description);
            println!("  Impact: {:.1}%", bottleneck.impact * 100.0);
            println!("  Solution: {}", bottleneck.solution);
            println!("  Location: {}", bottleneck.location);
            println!();
        }
    } else {
        println!("No performance bottlenecks detected.");
    }
    
    // Test 5: Optimization Suggestions
    println!("\n--- Test 5: Optimization Suggestions ---");
    
    let suggestions = machine.get_optimization_suggestions();
    
    if !suggestions.is_empty() {
        println!("Optimization suggestions:");
        for suggestion in &suggestions {
            println!("  Type: {:?}", suggestion.optimization_type);
            println!("  Description: {}", suggestion.description);
            println!("  Expected improvement: {:.1}%", suggestion.expected_improvement * 100.0);
            println!("  Difficulty: {}/10", suggestion.difficulty);
            println!("  Priority: {}/10", suggestion.priority);
            println!();
        }
    } else {
        println!("No optimization suggestions at this time.");
    }
    
    // Test 6: Cache Management
    println!("\n--- Test 6: Cache Management ---");
    
    // Perform many transitions to test cache behavior
    let mut current_state = expensive_state;
    for i in 0..10 {
        current_state = machine.transition(&current_state, GameEvent::CollectCoin);
        current_state = machine.transition(&current_state, GameEvent::TakeDamage(5));
    }
    
    let final_metrics = machine.get_performance_metrics();
    println!("After many transitions:");
    println!("  Total transitions: {}", final_metrics.total_transitions);
    println!("  Cache hits: {}", final_metrics.cache_hits);
    println!("  Cache misses: {}", final_metrics.cache_misses);
    println!("  Cache hit ratio: {:.2}%", final_metrics.cache_hit_ratio * 100.0);
    
    // Clear cache and test
    println!("\nClearing cache...");
    machine.clear_cache();
    
    let after_clear_metrics = machine.get_performance_metrics();
    println!("After clearing cache:");
    println!("  Cache hits: {}", after_clear_metrics.cache_hits);
    println!("  Cache misses: {}", after_clear_metrics.cache_misses);
    
    // Test 7: Custom Performance Configuration
    println!("\n--- Test 7: Custom Performance Configuration ---");
    
    let custom_config = PerformanceConfig {
        enabled: true,
        enable_caching: true,
        enable_lazy_evaluation: true,
        enable_profiling: true,
        cache_size_limit: 100, // Smaller cache
        cache_ttl: Duration::from_secs(60), // Shorter TTL
        cache_guard_results: true,
        cache_action_results: false,
        monitoring_interval: Duration::from_millis(500),
        track_memory_usage: true,
        track_allocations: true,
        optimization_strategies: vec![
            OptimizationStrategy::TransitionCaching,
            OptimizationStrategy::MemoryOptimization,
            OptimizationStrategy::LazyEvaluation,
        ],
    };
    
    let custom_machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("menu")
            .on(GameEvent::StartGame, "playing")
        .state("playing")
            .on(GameEvent::CollectCoin, "playing")
            .on(GameEvent::QuitGame, "menu")
        .build_with_performance_optimization(custom_config);
    
    println!("Custom performance configuration:");
    println!("  Cache size limit: {}", custom_machine.config().cache_size_limit);
    println!("  Cache TTL: {:?}", custom_machine.config().cache_ttl);
    println!("  Monitoring interval: {:?}", custom_machine.config().monitoring_interval);
    println!("  Optimization strategies: {:?}", custom_machine.config().optimization_strategies);
    
    // Test 8: Performance Builder Pattern
    println!("\n--- Test 8: Performance Builder Pattern ---");
    
    let built_machine = PerformanceBuilder::new(
        MachineBuilder::<GameContext, GameEvent>::new()
            .state("menu")
                .on(GameEvent::StartGame, "playing")
            .state("playing")
                .on(GameEvent::CollectCoin, "playing")
                .on(GameEvent::QuitGame, "menu")
            .build()
    )
    .with_caching(true)
    .with_lazy_evaluation(true)
    .with_profiling(true)
    .with_cache_size(200)
    .with_cache_ttl(Duration::from_secs(120))
    .with_memory_tracking(true)
    .with_allocation_tracking(true)
    .with_optimization_strategy(OptimizationStrategy::TransitionCaching)
    .with_optimization_strategy(OptimizationStrategy::MemoryOptimization)
    .build();
    
    println!("Performance builder configuration:");
    println!("  Caching enabled: {}", built_machine.config().enable_caching);
    println!("  Lazy evaluation enabled: {}", built_machine.config().enable_lazy_evaluation);
    println!("  Profiling enabled: {}", built_machine.config().enable_profiling);
    println!("  Cache size: {}", built_machine.config().cache_size_limit);
    println!("  Cache TTL: {:?}", built_machine.config().cache_ttl);
    println!("  Memory tracking: {}", built_machine.config().track_memory_usage);
    println!("  Allocation tracking: {}", built_machine.config().track_allocations);
    println!("  Strategies: {:?}", built_machine.config().optimization_strategies);
    
    // Test 9: Memory Usage Tracking
    println!("\n--- Test 9: Memory Usage Tracking ---");
    
    // Simulate some memory allocations
    let profiler = PerformanceProfiler::new(PerformanceConfig::default());
    
    profiler.record_allocation(1024); // 1KB
    profiler.record_allocation(2048); // 2KB
    profiler.record_allocation(512);  // 512B
    
    profiler.record_deallocation(1024); // Free 1KB
    
    let memory_metrics = profiler.get_metrics();
    println!("Memory tracking:");
    println!("  Total allocations: {}", memory_metrics.allocations);
    println!("  Total deallocations: {}", memory_metrics.deallocations);
    println!("  Current memory usage: {} bytes", memory_metrics.memory_usage);
    
    // Test 10: Performance Comparison
    println!("\n--- Test 10: Performance Comparison ---");
    
    // Create a regular machine for comparison
    let regular_machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("menu")
            .on(GameEvent::StartGame, "playing")
        .state("playing")
            .on(GameEvent::CollectCoin, "playing")
            .on(GameEvent::QuitGame, "menu")
        .build();
    
    let optimized_machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("menu")
            .on(GameEvent::StartGame, "playing")
        .state("playing")
            .on(GameEvent::CollectCoin, "playing")
            .on(GameEvent::QuitGame, "menu")
        .build_optimized();
    
    let regular_initial = regular_machine.initial_state();
    let optimized_initial = optimized_machine.machine().initial_state();
    
    // Test regular machine performance
    let regular_start = std::time::Instant::now();
    for _ in 0..100 {
        let _ = regular_machine.transition(&regular_initial, GameEvent::StartGame);
    }
    let regular_duration = regular_start.elapsed();
    
    // Test optimized machine performance
    let optimized_start = std::time::Instant::now();
    for _ in 0..100 {
        let _ = optimized_machine.transition(&optimized_initial, GameEvent::StartGame);
    }
    let optimized_duration = optimized_start.elapsed();
    
    println!("Performance comparison (100 transitions each):");
    println!("  Regular machine: {:?}", regular_duration);
    println!("  Optimized machine: {:?}", optimized_duration);
    
    if optimized_duration < regular_duration {
        let improvement = (regular_duration.as_nanos() as f64 / optimized_duration.as_nanos() as f64) - 1.0;
        println!("  ✅ Optimization improved performance by {:.1}%", improvement * 100.0);
    } else {
        println!("  ⚠️ Optimization did not improve performance in this test.");
    }
    
    // Test 11: Cache Statistics
    println!("\n--- Test 11: Cache Statistics ---");
    
    let cache_stats = CacheStats::new();
    let mut cache_stats = cache_stats;
    
    // Simulate some cache accesses
    for i in 0..20 {
        let hit = i % 3 == 0; // 1/3 hit ratio
        cache_stats.record_access(hit);
    }
    
    println!("Cache statistics:");
    println!("  Total accesses: {}", cache_stats.total_accesses);
    println!("  Hits: {}", cache_stats.hits);
    println!("  Misses: {}", cache_stats.misses);
    println!("  Hit ratio: {:.2}%", cache_stats.hit_ratio * 100.0);
    
    // Test 12: Memory Tracker
    println!("\n--- Test 12: Memory Tracker ---");
    
    let mut memory_tracker = MemoryTracker::new();
    
    memory_tracker.record_allocation(1024);
    memory_tracker.record_allocation(2048);
    memory_tracker.record_allocation(512);
    memory_tracker.record_deallocation(1024);
    memory_tracker.record_allocation(4096);
    
    println!("Memory tracker:");
    println!("  Total allocated: {} bytes", memory_tracker.total_allocated);
    println!("  Total freed: {} bytes", memory_tracker.total_freed);
    println!("  Current usage: {} bytes", memory_tracker.current_usage);
    println!("  Peak usage: {} bytes", memory_tracker.peak_usage);
    println!("  Allocation count: {}", memory_tracker.allocation_count);
    println!("  Deallocation count: {}", memory_tracker.deallocation_count);
    
    // Test 13: Comprehensive Performance Report
    println!("\n--- Test 13: Comprehensive Performance Report ---");
    
    let final_metrics = machine.get_performance_metrics();
    
    println!("=== Final Performance Report ===");
    println!("Overall Performance:");
    println!("  Total transitions: {}", final_metrics.total_transitions);
    println!("  Total execution time: {:?}", final_metrics.total_execution_time);
    println!("  Average transition time: {:?}", final_metrics.avg_transition_time);
    println!("  Performance range: {:?} - {:?}", final_metrics.min_transition_time, final_metrics.max_transition_time);
    
    println!("\nCaching Performance:");
    println!("  Cache hits: {}", final_metrics.cache_hits);
    println!("  Cache misses: {}", final_metrics.cache_misses);
    println!("  Cache hit ratio: {:.2}%", final_metrics.cache_hit_ratio * 100.0);
    
    println!("\nMemory Performance:");
    println!("  Current memory usage: {} bytes", final_metrics.memory_usage);
    println!("  Total allocations: {}", final_metrics.allocations);
    println!("  Total deallocations: {}", final_metrics.deallocations);
    println!("  Allocation rate: {:.2} bytes/second", final_metrics.allocation_rate);
    
    println!("\nPerformance Bottlenecks:");
    if final_metrics.bottlenecks.is_empty() {
        println!("  No bottlenecks detected");
    } else {
        for bottleneck in &final_metrics.bottlenecks {
            println!("  - {} (Impact: {:.1}%)", bottleneck.description, bottleneck.impact * 100.0);
        }
    }
    
    println!("\nOptimization Suggestions:");
    if final_metrics.optimization_suggestions.is_empty() {
        println!("  No suggestions at this time");
    } else {
        for suggestion in &final_metrics.optimization_suggestions {
            println!("  - {} (Priority: {}/10)", suggestion.description, suggestion.priority);
        }
    }
    
    println!("\n=== Performance Optimization Example Completed ===");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_performance_optimization() {
        let machine = MachineBuilder::<GameContext, GameEvent>::new()
            .state("menu")
                .on(GameEvent::StartGame, "playing")
            .state("playing")
                .on(GameEvent::CollectCoin, "playing")
            .build_optimized();
        
        let initial_state = machine.machine().initial_state();
        
        // Test transition with caching
        let result1 = machine.transition(&initial_state, GameEvent::StartGame);
        let result2 = machine.transition(&initial_state, GameEvent::StartGame);
        
        assert_eq!(*result1.value(), StateValue::Simple("playing".to_string()));
        assert_eq!(*result2.value(), StateValue::Simple("playing".to_string()));
        
        // Check performance metrics
        let metrics = machine.get_performance_metrics();
        assert_eq!(metrics.total_transitions, 2);
        assert!(metrics.cache_hits > 0);
    }
    
    #[test]
    fn test_lazy_evaluation() {
        let mut evaluator = LazyEvaluator::new(|| {
            "expensive result".to_string()
        });
        
        assert!(!evaluator.is_evaluated());
        
        let result = evaluator.get();
        assert_eq!(*result, "expensive result");
        assert!(evaluator.is_evaluated());
    }
    
    #[test]
    fn test_performance_builder() {
        let machine = MachineBuilder::<GameContext, GameEvent>::new()
            .state("menu")
                .on(GameEvent::StartGame, "playing")
            .state("playing")
                .on(GameEvent::CollectCoin, "playing")
            .build();
        
        let optimized_machine = PerformanceBuilder::new(machine)
            .with_caching(true)
            .with_lazy_evaluation(true)
            .with_profiling(true)
            .with_cache_size(500)
            .with_cache_ttl(Duration::from_secs(60))
            .with_memory_tracking(true)
            .with_allocation_tracking(true)
            .with_optimization_strategy(OptimizationStrategy::TransitionCaching)
            .build();
        
        let config = optimized_machine.config();
        assert!(config.enable_caching);
        assert!(config.enable_lazy_evaluation);
        assert!(config.enable_profiling);
        assert_eq!(config.cache_size_limit, 500);
        assert_eq!(config.cache_ttl, Duration::from_secs(60));
        assert!(config.track_memory_usage);
        assert!(config.track_allocations);
        assert!(config.optimization_strategies.contains(&OptimizationStrategy::TransitionCaching));
    }
}
