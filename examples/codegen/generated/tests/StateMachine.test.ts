import { StateMachine, State, StateEvent } from '../src/StateMachine';

describe('StateMachine', () => {
    let machine: StateMachine;

    beforeEach(() => {
        machine = new StateMachine();
    });

    test('should create with initial state', () => {
        expect(machine.getCurrentState()).toBe(State.idle);
    });

    test('should handle valid transitions', () => {
        const result = machine.transition(StateEvent.Start);
        expect(result).toBe(State.running);
        const result = machine.transition(StateEvent.Pause);
        expect(result).toBe(State.paused);
        const result = machine.transition(StateEvent.Resume);
        expect(result).toBe(State.running);
        const result = machine.transition(StateEvent.Stop);
        expect(result).toBe(State.idle);
    });
});
