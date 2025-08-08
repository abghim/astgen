use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Action {
	Shift(u32),
	Reduce(u32),
	Accept,
	Error
}

#[derive(Copy, Clone)]
enum GotoState {
	State(u32),
	Error
}

#[derive(Copy, Clone)]
struct Production {
	lhs: u32,
	rhs: &'static [u32]
}

#[repr(u32)]
enum Terminal {
	eq = 0,
	mul = 1,
	id = 2,
	EOI = 3,
	COUNT
}

#[repr(u32)]
enum Nonterminal {
	S = 0,
	L = 1,
	R = 2,
	Augmented
}


use Nonterminal::*;
use Terminal::*;

static RULES: [Production; 6] = [
	Production {lhs:Augmented as u32, rhs:&[S as u32]},
	Production {lhs:S as u32, rhs:&[L as u32, eq as u32, R as u32]},
	Production {lhs:L as u32, rhs:&[id as u32]},
	Production {lhs:R as u32, rhs:&[L as u32]},
	Production {lhs:S as u32, rhs:&[R as u32]},
	Production {lhs:L as u32, rhs:&[mul as u32, R as u32]},
];

static ACTION: [[Action; 4]; 14] = [
	[Action::Error, Action::Shift(3), Action::Shift(4), Action::Error, ],
	[Action::Error, Action::Error, Action::Error, Action::Accept, ],
	[Action::Shift(6), Action::Error, Action::Error, Action::Reduce(3), ],
	[Action::Error, Action::Shift(3), Action::Shift(4), Action::Error, ],
	[Action::Reduce(2), Action::Error, Action::Error, Action::Reduce(2), ],
	[Action::Error, Action::Error, Action::Error, Action::Reduce(4), ],
	[Action::Error, Action::Shift(1), Action::Shift(1), Action::Error, ],
	[Action::Reduce(3), Action::Error, Action::Error, Action::Reduce(3), ],
	[Action::Reduce(5), Action::Error, Action::Error, Action::Reduce(5), ],
	[Action::Error, Action::Error, Action::Error, Action::Reduce(3), ],
	[Action::Error, Action::Shift(1), Action::Shift(1), Action::Error, ],
	[Action::Error, Action::Error, Action::Error, Action::Reduce(2), ],
	[Action::Error, Action::Error, Action::Error, Action::Reduce(1), ],
	[Action::Error, Action::Error, Action::Error, Action::Reduce(5), ],
];

static GOTO: [[GotoState; 3]; 14] = [
	[GotoState::State(1), GotoState::State(2), GotoState::State(5), ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::State(7), GotoState::State(8), ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::State(9), GotoState::State(12), ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::State(9), GotoState::State(13), ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
];

fn main() {}
