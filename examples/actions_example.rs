use leptos_state::machine::*;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Default)]
struct GameContext {
    player_health: i32,
    player_level: u32,
    coins: u32,
    is_online: bool,
    last_action_time: std::time::Instant,
    action_count: usize,
    achievements: Vec<String>,
    game_state: String,
}

#[derive(Debug, Clone, PartialEq)]
enum GameEvent {
    Attack,
    Heal,
    CollectCoin,
    LevelUp,
    GoOffline,
    GoOnline,
    SpecialMove,
    SaveGame,
    LoadGame,
}

impl Event for GameEvent {
    fn event_type(&self) -> &str {
        match self {
            GameEvent::Attack => "attack",
            GameEvent::Heal => "heal",
            GameEvent::CollectCoin => "collect_coin",
            GameEvent::LevelUp => "level_up",
            GameEvent::GoOffline => "go_offline",
            GameEvent::GoOnline => "go_online",
            GameEvent::SpecialMove => "special_move",
            GameEvent::SaveGame => "save_game",
            GameEvent::LoadGame => "load_game",
        }
    }
}

fn main() {
    println!("=== Advanced Actions Example ===");
    
    // Create a complex game state machine with various actions
    let machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("idle")
            .on_entry_log("Player entered idle state")
            .on_entry_fn(|ctx, _| {
                ctx.game_state = "idle".to_string();
                println!("🎮 Player is now idle");
            })
            .on_exit_log("Player left idle state")
            .on(GameEvent::Attack, "attacking")
                .guard_fn(|ctx, _| ctx.player_health > 0)
                .action_fn(|ctx, _| {
                    ctx.action_count += 1;
                    println!("⚔️ Player attacked! Action count: {}", ctx.action_count);
                })
                .action_log("Player performed attack")
            .on(GameEvent::Heal, "healing")
                .guard_fn(|ctx, _| ctx.coins >= 10)
                .action_fn(|ctx, _| {
                    ctx.coins -= 10;
                    ctx.player_health = (ctx.player_health + 20).min(100);
                    println!("💚 Player healed! Health: {}, Coins: {}", ctx.player_health, ctx.coins);
                })
                .action_log("Player used healing potion")
            .on(GameEvent::CollectCoin, "collecting")
                .action_fn(|ctx, _| {
                    ctx.coins += 1;
                    println!("🪙 Collected coin! Total coins: {}", ctx.coins);
                })
                .action_log("Player collected a coin")
            .on(GameEvent::SpecialMove, "special")
                .guard_fn(|ctx, _| ctx.player_level >= 5)
                .action_fn(|ctx, _| {
                    ctx.action_count += 1;
                    println!("✨ Special move executed! Action count: {}", ctx.action_count);
                })
                .action_log("Player used special move")
        .state("attacking")
            .on_entry_log("Player entered attacking state")
            .on_entry_fn(|ctx, _| {
                ctx.game_state = "attacking".to_string();
                println!("⚔️ Player is now attacking");
            })
            .on_exit_log("Player left attacking state")
            .on(GameEvent::Attack, "idle")
                .action_fn(|ctx, _| {
                    println!("🏁 Attack sequence completed");
                })
        .state("healing")
            .on_entry_log("Player entered healing state")
            .on_entry_fn(|ctx, _| {
                ctx.game_state = "healing".to_string();
                println!("💚 Player is now healing");
            })
            .on_exit_log("Player left healing state")
            .on(GameEvent::Heal, "idle")
                .action_fn(|ctx, _| {
                    println!("🏁 Healing sequence completed");
                })
        .state("collecting")
            .on_entry_log("Player entered collecting state")
            .on_entry_fn(|ctx, _| {
                ctx.game_state = "collecting".to_string();
                println!("🪙 Player is now collecting");
            })
            .on_exit_log("Player left collecting state")
            .on(GameEvent::CollectCoin, "idle")
                .action_fn(|ctx, _| {
                    println!("🏁 Collection sequence completed");
                })
        .state("special")
            .on_entry_log("Player entered special state")
            .on_entry_fn(|ctx, _| {
                ctx.game_state = "special".to_string();
                println!("✨ Player is now in special mode");
            })
            .on_exit_log("Player left special state")
            .on(GameEvent::SpecialMove, "idle")
                .action_fn(|ctx, _| {
                    println!("🏁 Special move sequence completed");
                })
        .build();
    
    println!("✓ Machine created with advanced actions");
    
    // Test initial state
    let mut context = GameContext {
        player_health: 50,
        player_level: 3,
        coins: 5,
        is_online: true,
        last_action_time: std::time::Instant::now(),
        action_count: 0,
        achievements: Vec::new(),
        game_state: "".to_string(),
    };
    
    let mut state = machine.initial_state();
    println!("✓ Initial state: {:?}", state.value());
    
    // Test attack transition with actions
    println!("\n--- Testing Attack Transition with Actions ---");
    let attack_result = machine.transition(&state, GameEvent::Attack);
    println!("Attack transition result: {:?}", attack_result.value());
    println!("Context after attack: health={}, action_count={}", 
             attack_result.context().player_health, 
             attack_result.context().action_count);
    
    // Test heal transition (should fail due to insufficient coins)
    println!("\n--- Testing Heal Transition (Insufficient Coins) ---");
    let heal_result = machine.transition(&state, GameEvent::Heal);
    println!("Heal transition result: {:?}", heal_result.value());
    
    // Add coins and try again
    context.coins = 15;
    state = MachineStateImpl {
        value: state.value().clone(),
        context: context.clone(),
    };
    
    println!("\n--- Testing Heal Transition (With Coins) ---");
    let heal_result = machine.transition(&state, GameEvent::Heal);
    println!("Heal transition result: {:?}", heal_result.value());
    println!("Context after heal: health={}, coins={}", 
             heal_result.context().player_health, 
             heal_result.context().coins);
    
    // Test coin collection
    println!("\n--- Testing Coin Collection ---");
    let collect_result = machine.transition(&state, GameEvent::CollectCoin);
    println!("Collect transition result: {:?}", collect_result.value());
    println!("Context after collection: coins={}", collect_result.context().coins);
    
    // Test special move (should fail due to low level)
    println!("\n--- Testing Special Move (Low Level) ---");
    let special_result = machine.transition(&state, GameEvent::SpecialMove);
    println!("Special move result: {:?}", special_result.value());
    
    // Level up and try again
    context.player_level = 8;
    state = MachineStateImpl {
        value: state.value().clone(),
        context: context.clone(),
    };
    
    println!("\n--- Testing Special Move (High Level) ---");
    let special_result = machine.transition(&state, GameEvent::SpecialMove);
    println!("Special move result: {:?}", special_result.value());
    println!("Context after special move: action_count={}", 
             special_result.context().action_count);
    
    // Test state transitions with entry/exit actions
    println!("\n--- Testing State Transitions with Entry/Exit Actions ---");
    
    // Go to attacking state
    let attacking_state = machine.transition(&state, GameEvent::Attack);
    println!("Current state: {:?}, game_state: {}", 
             attacking_state.value(), 
             attacking_state.context().game_state);
    
    // Go back to idle
    let idle_state = machine.transition(&attacking_state, GameEvent::Attack);
    println!("Current state: {:?}, game_state: {}", 
             idle_state.value(), 
             idle_state.context().game_state);
    
    // Test complex action composition
    println!("\n--- Testing Complex Action Composition ---");
    
    // Create a composite action
    let action1 = actions::FunctionAction::new(|ctx: &mut GameContext, _| {
        ctx.achievements.push("First Action".to_string());
        println!("🏆 Achievement unlocked: First Action");
    });
    
    let action2 = actions::LogAction::new("Complex action executed");
    
    let action3 = actions::PureAction::new(|| {
        println!("🎯 Pure action executed");
    });
    
    let composite = actions::SequentialAction::new(vec![
        Box::new(action1),
        Box::new(action2),
        Box::new(action3),
    ]);
    
    // Execute the composite action
    composite.execute(&mut context, &GameEvent::Attack);
    println!("Achievements: {:?}", context.achievements);
    
    // Test conditional actions
    println!("\n--- Testing Conditional Actions ---");
    
    let conditional_action = actions::ConditionalAction::new(
        |ctx: &GameContext, _| ctx.player_level >= 5,
        Box::new(actions::FunctionAction::new(|ctx: &mut GameContext, _| {
            ctx.achievements.push("High Level Player".to_string());
            println!("🏆 Achievement unlocked: High Level Player");
        }))
    );
    
    conditional_action.execute(&mut context, &GameEvent::LevelUp);
    println!("Achievements: {:?}", context.achievements);
    
    // Test timer actions
    println!("\n--- Testing Timer Actions ---");
    
    let timed_action = actions::TimerAction::new(Box::new(
        actions::FunctionAction::new(|ctx: &mut GameContext, _| {
            ctx.action_count += 1;
            println!("⏱️ Timed action executed");
        })
    ));
    
    timed_action.execute(&mut context, &GameEvent::Attack);
    println!("Action count after timed action: {}", context.action_count);
    
    // Test metrics actions
    println!("\n--- Testing Metrics Actions ---");
    
    let metrics_action = actions::MetricsAction::new(
        Box::new(actions::FunctionAction::new(|ctx: &mut GameContext, _| {
            ctx.action_count += 1;
            println!("📊 Action executed and metrics recorded");
        })),
        "game_actions"
    );
    
    metrics_action.execute(&mut context, &GameEvent::Attack);
    metrics_action.execute(&mut context, &GameEvent::Attack);
    
    let metrics = metrics_action.get_metrics();
    println!("Metrics: {:?}", metrics);
    
    println!("\n=== Advanced Actions Example Completed ===");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_advanced_actions() {
        let machine = MachineBuilder::<GameContext, GameEvent>::new()
            .state("idle")
                .on_entry_log("Entered idle")
                .on(GameEvent::Attack, "attacking")
                    .action_fn(|ctx, _| ctx.action_count += 1)
                .state("attacking")
                    .on(GameEvent::Attack, "idle")
            .build();
        
        let context = GameContext {
            player_health: 50,
            player_level: 3,
            coins: 5,
            is_online: true,
            last_action_time: std::time::Instant::now(),
            action_count: 0,
            achievements: Vec::new(),
            game_state: "".to_string(),
        };
        
        let state = machine.initial_state();
        let attack_result = machine.transition(&state, GameEvent::Attack);
        
        assert_eq!(*attack_result.value(), StateValue::Simple("attacking".to_string()));
        assert_eq!(attack_result.context().action_count, 1);
    }
}
