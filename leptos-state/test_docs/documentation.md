# State Machine Documentation

## Overview

This document provides comprehensive documentation for the state machine.

## States

### counting

State description and behavior.

### idle

State description and behavior.

## Events

### start

Event description and effects.

### stop

Event description and effects.

### pause

Event description and effects.

## Transitions

- **idle** → **running** (Event: start)
- **running** → **paused** (Event: pause)
- **running** → **idle** (Event: stop)

## Guards

State transition guards and conditions.

## Actions

State entry/exit actions and transition actions.

## Usage Examples

```rust
// Example state machine usage
let machine = MachineBuilder::new()
    .state("idle")
    .on(Event::Start, "running")
    .build();
```

## API Reference

### MachineBuilder

The main builder for creating state machines.

### Methods

- `state(name)` - Define a new state
- `on(event, target)` - Define a transition
- `build()` - Build the state machine

## State Diagram

