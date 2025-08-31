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
        }
    }
}

fn main() {
    println!("=== Advanced Guards Example ===");
    
    // Create a complex game state machine with various guards
    let machine = MachineBuilder::<GameContext, GameEvent>::new()
        .state("idle")
            .on(GameEvent::Attack, "attacking")
                .guard_fn(|ctx, _| ctx.player_health > 0)
                .guard_field_equals(|ctx| ctx.is_online, true)
                .guard_field_range(|ctx| ctx.player_level, 1, 100)
            .on(GameEvent::Heal, "healing")
                .guard_fn(|ctx, _| ctx.player_health < 100)
                .guard_field_equals(|ctx| ctx.coins, 10)
            .on(GameEvent::CollectCoin, "collecting")
                .guard_time_limit(Duration::from_secs(1))
            .on(GameEvent::SpecialMove, "special")
                .guard_fn(|ctx, _| ctx.player_level >= 5)
                .guard_max_transitions(3)
        .state("attacking")
            .on(GameEvent::Attack, "idle")
                .guard_fn(|ctx, _| ctx.action_count < 10)
        .state("healing")
            .on(GameEvent::Heal, "idle")
                .guard_time_limit(Duration::from_millis(500))
        .state("collecting")
            .on(GameEvent::CollectCoin, "idle")
        .state("special")
            .on(GameEvent::SpecialMove, "idle")
        .build();
    
    println!("✓ Machine created with advanced guards");
    
    // Test initial state
    let mut context = GameContext {
        player_health: 50,
        player_level: 3,
        coins: 5,
        is_online: true,
        last_action_time: std::time::Instant::now(),
        action_count: 0,
    };
    
    let mut state = machine.initial_state();
    println!("✓ Initial state: {:?}", state.value());
    
    // Test attack transition with multiple guards
    println!("\n--- Testing Attack Transition ---");
    let attack_result = machine.transition(&state, GameEvent::Attack);
    println!("Attack transition result: {:?}", attack_result.value());
    
    // Test heal transition (should fail due to insufficient coins)
    println!("\n--- Testing Heal Transition (Insufficient Coins) ---");
    let heal_result = machine.transition(&state, GameEvent::Heal);
    println!("Heal transition result: {:?}", heal_result.value());
    
    // Add coins and try again
    context.coins = 10;
    state = MachineStateImpl {
        value: state.value().clone(),
        context: context.clone(),
    };
    
    println!("\n--- Testing Heal Transition (With Coins) ---");
    let heal_result = machine.transition(&state, GameEvent::Heal);
    println!("Heal transition result: {:?}", heal_result.value());
    
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
    
    // Test time-based guard
    println!("\n--- Testing Time-Based Guard ---");
    let collect_result1 = machine.transition(&state, GameEvent::CollectCoin);
    println!("First coin collection: {:?}", collect_result1.value());
    
    // Try to collect again immediately (should fail due to time guard)
    let collect_result2 = machine.transition(&state, GameEvent::CollectCoin);
    println!("Immediate second collection: {:?}", collect_result2.value());
    
    // Wait and try again
    std::thread::sleep(Duration::from_millis(1100));
    let collect_result3 = machine.transition(&state, GameEvent::CollectCoin);
    println!("Delayed second collection: {:?}", collect_result3.value());
    
    // Test counter guard
    println!("\n--- Testing Counter Guard ---");
    for i in 1..=5 {
        let special_result = machine.transition(&state, GameEvent::SpecialMove);
        println!("Special move attempt {}: {:?}", i, special_result.value());
    }
    
    // Test going offline (should block attacks)
    println!("\n--- Testing Offline State ---");
    context.is_online = false;
    state = MachineStateImpl {
        value: state.value().clone(),
        context: context.clone(),
    };
    
    let offline_attack = machine.transition(&state, GameEvent::Attack);
    println!("Attack while offline: {:?}", offline_attack.value());
    
    println!("\n=== Advanced Guards Example Completed ===");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_advanced_guards() {
        let machine = MachineBuilder::<GameContext, GameEvent>::new()
            .state("idle")
                .on(GameEvent::Attack, "attacking")
                    .guard_fn(|ctx, _| ctx.player_health > 0)
                    .guard_field_equals(|ctx| ctx.is_online, true)
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
        };
        
        let state = machine.initial_state();
        let attack_result = machine.transition(&state, GameEvent::Attack);
        
        assert_eq!(*attack_result.value(), StateValue::Simple("attacking".to_string()));
    }
}
