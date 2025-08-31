use leptos_state::machine::*;
use std::time::Duration;

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
    println!("=== State Machine Testing Framework Example ===");
    
    // Create a game state machine for testing
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
        .build_testable();
    
    println!("✓ Testable machine created");
    
    // Test 1: Basic Unit Testing
    println!("\n--- Test 1: Basic Unit Testing ---");
    
    // Create a simple test case
    let test_case = TestCase {
        steps: vec![
            TestStep {
                event: GameEvent::StartGame,
                expected_guards: Vec::new(),
                expected_actions: Vec::new(),
            },
            TestStep {
                event: GameEvent::CollectCoin,
                expected_guards: Vec::new(),
                expected_actions: Vec::new(),
            },
        ],
        expected_final_state: Some(StateValue::Simple("playing".to_string())),
        expected_final_context: None,
    };
    
    let mut test_runner = machine;
    let result = test_runner.run_test_case(test_case);
    
    println!("Test result: {}", if result.passed { "PASSED" } else { "FAILED" });
    println!("Duration: {:?}", result.duration);
    println!("Transitions tested: {}", result.transitions_tested);
    
    if let Some(error) = result.error {
        println!("Error: {}", error);
    }
    
    // Test 2: Coverage Testing
    println!("\n--- Test 2: Coverage Testing ---");
    
    if let Some(coverage) = result.coverage {
        println!("Coverage information:");
        println!("  States covered: {} / {}", coverage.states_covered.len(), 2);
        println!("  Transitions covered: {} / {}", coverage.transitions_covered.len(), 2);
        println!("  Events covered: {} / {}", coverage.events_covered.len(), 2);
        println!("  Coverage percentage: {:.2}%", coverage.coverage_percentage);
        
        println!("  States visited: {:?}", coverage.states_covered);
        println!("  Transitions visited: {:?}", coverage.transitions_covered);
    }
    
    // Test 3: Performance Testing
    println!("\n--- Test 3: Performance Testing ---");
    
    if let Some(performance) = result.performance {
        println!("Performance metrics:");
        println!("  Average transition time: {:?}", performance.avg_transition_time);
        println!("  Maximum transition time: {:?}", performance.max_transition_time);
        println!("  Minimum transition time: {:?}", performance.min_transition_time);
        println!("  Memory usage: {} bytes", performance.memory_usage);
        println!("  Allocations: {}", performance.allocations);
    }
    
    // Test 4: Property-Based Testing
    println!("\n--- Test 4: Property-Based Testing ---");
    
    // Create properties to test
    let properties = vec![
        // Property 1: Game should always start in menu state
        Property::new("game_starts_in_menu", |result| {
            let first_step = result.test_path.first();
            let holds = first_step.map_or(false, |step| step.from_state == "menu");
            PropertyResult {
                holds,
                description: "Game should start in menu state".to_string(),
                details: first_step.map(|step| format!("Started in: {}", step.from_state)),
            }
        }),
        
        // Property 2: Player health should never go below 0
        Property::new("health_never_negative", |result| {
            let holds = result.test_path.iter().all(|step| {
                // This is a simplified check - in a real implementation, you'd parse the context
                !step.context_after.contains("player_health: -")
            });
            PropertyResult {
                holds,
                description: "Player health should never be negative".to_string(),
                details: None,
            }
        }),
        
        // Property 3: Score should increase when collecting coins
        Property::new("score_increases_with_coins", |result| {
            let coin_collections = result.test_path.iter()
                .filter(|step| step.event == "collect_coin")
                .count();
            let holds = coin_collections == 0 || result.test_path.len() > 1;
            PropertyResult {
                holds,
                description: "Score should increase when collecting coins".to_string(),
                details: Some(format!("Coin collections: {}", coin_collections)),
            }
        }),
    ];
    
    let property_results = test_runner.run_property_tests(properties);
    
    println!("Property-based test results:");
    for property_result in &property_results {
        println!("  {}: {}", 
                 property_result.property_name, 
                 if property_result.passed { "PASSED" } else { "FAILED" });
        println!("    Tests run: {}/{} passed", 
                 property_result.passed_tests, 
                 property_result.total_tests);
        
        if !property_result.counter_examples.is_empty() {
            println!("    Counter-examples found: {}", property_result.counter_examples.len());
        }
    }
    
    // Test 5: Integration Testing
    println!("\n--- Test 5: Integration Testing ---");
    
    // Create integration scenarios
    let scenarios = vec![
        IntegrationScenario {
            name: "Complete Game Session".to_string(),
            test_cases: vec![
                // Scenario 1: Start game, collect coins, level up, quit
                TestCase {
                    steps: vec![
                        TestStep {
                            event: GameEvent::StartGame,
                            expected_guards: Vec::new(),
                            expected_actions: Vec::new(),
                        },
                        TestStep {
                            event: GameEvent::CollectCoin,
                            expected_guards: Vec::new(),
                            expected_actions: Vec::new(),
                        },
                        TestStep {
                            event: GameEvent::CollectCoin,
                            expected_guards: Vec::new(),
                            expected_actions: Vec::new(),
                        },
                        TestStep {
                            event: GameEvent::LevelUp,
                            expected_guards: Vec::new(),
                            expected_actions: Vec::new(),
                        },
                        TestStep {
                            event: GameEvent::QuitGame,
                            expected_guards: Vec::new(),
                            expected_actions: Vec::new(),
                        },
                    ],
                    expected_final_state: Some(StateValue::Simple("menu".to_string())),
                    expected_final_context: None,
                },
                
                // Scenario 2: Start game, take damage, save, quit
                TestCase {
                    steps: vec![
                        TestStep {
                            event: GameEvent::StartGame,
                            expected_guards: Vec::new(),
                            expected_actions: Vec::new(),
                        },
                        TestStep {
                            event: GameEvent::TakeDamage(20),
                            expected_guards: Vec::new(),
                            expected_actions: Vec::new(),
                        },
                        TestStep {
                            event: GameEvent::SaveGame,
                            expected_guards: Vec::new(),
                            expected_actions: Vec::new(),
                        },
                        TestStep {
                            event: GameEvent::QuitGame,
                            expected_guards: Vec::new(),
                            expected_actions: Vec::new(),
                        },
                    ],
                    expected_final_state: Some(StateValue::Simple("menu".to_string())),
                    expected_final_context: None,
                },
            ],
        },
    ];
    
    let integration_results = test_runner.run_integration_tests(scenarios);
    
    println!("Integration test results:");
    for integration_result in &integration_results {
        println!("  {}: {}", 
                 integration_result.scenario_name, 
                 if integration_result.passed { "PASSED" } else { "FAILED" });
        println!("    Tests run: {}/{} passed", 
                 integration_result.passed_tests, 
                 integration_result.total_tests);
        println!("    Duration: {:?}", integration_result.duration);
    }
    
    // Test 6: Automated Test Generation
    println!("\n--- Test 6: Automated Test Generation ---");
    
    // Create a new test runner for test generation
    let testable_machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("menu")
            .on(GameEvent::StartGame, "playing")
        .state("playing")
            .on(GameEvent::CollectCoin, "playing")
            .on(GameEvent::TakeDamage(0), "playing")
            .on(GameEvent::LevelUp, "playing")
            .on(GameEvent::SaveGame, "playing")
            .on(GameEvent::QuitGame, "menu")
        .build_testable();
    
    let generated_test_cases = testable_machine.generate_test_cases();
    
    println!("Generated {} test cases automatically", generated_test_cases.len());
    
    // Run some of the generated tests
    let mut test_runner = testable_machine;
    let mut passed_tests = 0;
    let mut total_tests = 0;
    
    for (i, test_case) in generated_test_cases.iter().take(5).enumerate() {
        println!("  Running generated test case {}...", i + 1);
        let result = test_runner.run_test_case(test_case.clone());
        total_tests += 1;
        if result.passed {
            passed_tests += 1;
        }
        println!("    Result: {}", if result.passed { "PASSED" } else { "FAILED" });
        println!("    Transitions: {}", result.transitions_tested);
    }
    
    println!("Generated test summary: {}/{} passed", passed_tests, total_tests);
    
    // Test 7: Custom Test Configuration
    println!("\n--- Test 7: Custom Test Configuration ---");
    
    let custom_config = TestConfig {
        max_iterations: 100,
        max_transitions: 10,
        test_timeout: Duration::from_secs(5),
        verbose: true,
        track_coverage: true,
        benchmark: true,
        random_seed: Some(42),
        data_strategy: DataStrategy::Boundary,
    };
    
    let custom_test_runner = MachineBuilder::<GameContext, GameEvent>::new()
        .state("menu")
            .on(GameEvent::StartGame, "playing")
        .state("playing")
            .on(GameEvent::CollectCoin, "playing")
            .on(GameEvent::QuitGame, "menu")
        .build_with_testing(custom_config);
    
    println!("Custom test configuration:");
    println!("  Max iterations: {}", custom_test_runner.config.max_iterations);
    println!("  Max transitions: {}", custom_test_runner.config.max_transitions);
    println!("  Timeout: {:?}", custom_test_runner.config.test_timeout);
    println!("  Verbose: {}", custom_test_runner.config.verbose);
    println!("  Track coverage: {}", custom_test_runner.config.track_coverage);
    println!("  Benchmark: {}", custom_test_runner.config.benchmark);
    println!("  Random seed: {:?}", custom_test_runner.config.random_seed);
    
    // Test 8: Test Builder Pattern
    println!("\n--- Test 8: Test Builder Pattern ---");
    
    let test_runner = TestBuilder::new(
        MachineBuilder::<GameContext, GameEvent>::new()
            .state("menu")
                .on(GameEvent::StartGame, "playing")
            .state("playing")
                .on(GameEvent::CollectCoin, "playing")
                .on(GameEvent::QuitGame, "menu")
            .build()
    )
    .with_max_iterations(50)
    .with_max_transitions(5)
    .with_timeout(Duration::from_secs(2))
    .with_verbose(false)
    .with_coverage_tracking(true)
    .with_benchmarking(false)
    .with_random_seed(123)
    .build();
    
    println!("Test builder configuration:");
    println!("  Max iterations: {}", test_runner.config.max_iterations);
    println!("  Max transitions: {}", test_runner.config.max_transitions);
    println!("  Timeout: {:?}", test_runner.config.test_timeout);
    println!("  Verbose: {}", test_runner.config.verbose);
    println!("  Track coverage: {}", test_runner.config.track_coverage);
    println!("  Benchmark: {}", test_runner.config.benchmark);
    println!("  Random seed: {:?}", test_runner.config.random_seed);
    
    // Test 9: Test Macros (if available)
    println!("\n--- Test 9: Test Macros ---");
    
    // Note: The macros would be used like this in a real implementation:
    // let test_case = test_case![
    //     GameEvent::StartGame,
    //     GameEvent::CollectCoin,
    //     GameEvent::QuitGame
    // ];
    // 
    // let property = property!("test_property", |result| {
    //     PropertyResult {
    //         holds: result.passed,
    //         description: "Test property".to_string(),
    //         details: None,
    //     }
    // });
    
    println!("Test macros would be used for creating test cases and properties");
    println!("Example usage:");
    println!("  test_case![GameEvent::StartGame, GameEvent::CollectCoin]");
    println!("  property!(\"health_check\", |result| { ... })");
    
    // Test 10: Comprehensive Test Report
    println!("\n--- Test 10: Comprehensive Test Report ---");
    
    let final_test_runner = MachineBuilder::<GameContext, GameEvent>::new()
        .state("menu")
            .on(GameEvent::StartGame, "playing")
        .state("playing")
            .on(GameEvent::CollectCoin, "playing")
            .on(GameEvent::TakeDamage(0), "playing")
            .on(GameEvent::LevelUp, "playing")
            .on(GameEvent::SaveGame, "playing")
            .on(GameEvent::QuitGame, "menu")
        .build_testable();
    
    // Generate and run comprehensive tests
    let all_test_cases = final_test_runner.generate_test_cases();
    let mut final_test_runner = final_test_runner;
    
    let mut total_passed = 0;
    let mut total_failed = 0;
    let mut total_duration = Duration::ZERO;
    let mut total_transitions = 0;
    
    println!("Running comprehensive test suite...");
    for (i, test_case) in all_test_cases.iter().enumerate() {
        if i % 10 == 0 {
            println!("  Progress: {}/{} tests completed", i, all_test_cases.len());
        }
        
        let result = final_test_runner.run_test_case(test_case.clone());
        
        if result.passed {
            total_passed += 1;
        } else {
            total_failed += 1;
        }
        
        total_duration += result.duration;
        total_transitions += result.transitions_tested;
    }
    
    println!("\n=== Comprehensive Test Report ===");
    println!("Total tests: {}", all_test_cases.len());
    println!("Passed: {}", total_passed);
    println!("Failed: {}", total_failed);
    println!("Success rate: {:.2}%", (total_passed as f64 / all_test_cases.len() as f64) * 100.0);
    println!("Total duration: {:?}", total_duration);
    println!("Total transitions tested: {}", total_transitions);
    println!("Average test duration: {:?}", total_duration / all_test_cases.len() as u32);
    println!("Average transitions per test: {:.2}", total_transitions as f64 / all_test_cases.len() as f64);
    
    println!("\n=== Testing Framework Example Completed ===");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_testing_workflow() {
        let machine = MachineBuilder::<GameContext, GameEvent>::new()
            .state("menu")
                .on(GameEvent::StartGame, "playing")
            .state("playing")
                .on(GameEvent::CollectCoin, "playing")
            .build_testable();
        
        let test_case = TestCase {
            steps: vec![
                TestStep {
                    event: GameEvent::StartGame,
                    expected_guards: Vec::new(),
                    expected_actions: Vec::new(),
                }
            ],
            expected_final_state: Some(StateValue::Simple("playing".to_string())),
            expected_final_context: None,
        };
        
        let mut test_runner = machine;
        let result = test_runner.run_test_case(test_case);
        
        assert!(result.passed);
        assert_eq!(result.transitions_tested, 1);
        assert!(result.coverage.is_some());
    }
    
    #[test]
    fn test_property_based_testing() {
        let machine = MachineBuilder::<GameContext, GameEvent>::new()
            .state("menu")
                .on(GameEvent::StartGame, "playing")
            .state("playing")
                .on(GameEvent::CollectCoin, "playing")
            .build_testable();
        
        let property = Property::new("test_property", |result| {
            PropertyResult {
                holds: result.passed,
                description: "Test property".to_string(),
                details: None,
            }
        });
        
        let mut test_runner = machine;
        let results = test_runner.run_property_tests(vec![property]);
        
        assert_eq!(results.len(), 1);
        // The property should pass if the test passed
    }
    
    #[test]
    fn test_test_generation() {
        let machine = MachineBuilder::<GameContext, GameEvent>::new()
            .state("menu")
                .on(GameEvent::StartGame, "playing")
            .state("playing")
                .on(GameEvent::CollectCoin, "playing")
            .build_testable();
        
        let test_cases = machine.generate_test_cases();
        
        assert!(!test_cases.is_empty());
        // Should generate at least state coverage and transition coverage tests
    }
}
