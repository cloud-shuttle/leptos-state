// Generated State Machine Code
// This file was automatically generated

export enum State {
    idle,
    counting,
}

export enum StateEvent {
    Start,
    Stop,
    Pause,
    Resume,
}

export interface StateContext {
    id: string;
    data: Record<string, string>;
}

export class StateMachine {
    private currentState: State;
    private context: StateContext;
    private transitions: Map<string, State>;

    constructor() {
        this.currentState = State.idle;
        this.context = {
            id: crypto.randomUUID(),
            data: {},
        };
        this.transitions = new Map();

        this.transitions.set(`${State.idle}-${StateEvent.Start}`, State.running);
        this.transitions.set(`${State.running}-${StateEvent.Pause}`, State.paused);
        this.transitions.set(`${State.paused}-${StateEvent.Resume}`, State.running);
        this.transitions.set(`${State.running}-${StateEvent.Stop}`, State.idle);
    }

    public transition(event: StateEvent): State | null {
        const key = `${this.currentState}-${event}`;
        const newState = this.transitions.get(key);
        if (newState) {
            this.currentState = newState;
            return newState;
        }
        return null;
    }

    public getCurrentState(): State {
        return this.currentState;
    }

    public getContext(): StateContext {
        return this.context;
    }
}
