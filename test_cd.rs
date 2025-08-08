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
	d = 0,
	EOI = 1,
	c = 2,
	COUNT
}

#[repr(u32)]
enum Nonterminal {
	C = 0,
	S = 1,
	Augmented
}


use Nonterminal::*;
use Terminal::*;

static RULES: [Production; 4] = [
	Production {lhs:Augmented as u32, rhs:&[S as u32]},
	Production {lhs:C as u32, rhs:&[c as u32, C as u32]},
	Production {lhs:C as u32, rhs:&[d as u32]},
	Production {lhs:S as u32, rhs:&[C as u32, C as u32]},
];

static ACTION: [[Action; 3]; 10] = [
	[Action::Shift(1), Action::Error, Action::Shift(4), ],
	[Action::Reduce(2), Action::Error, Action::Reduce(2), ],
	[Action::Error, Action::Accept, Action::Error, ],
	[Action::Shift(5), Action::Error, Action::Shift(7), ],
	[Action::Shift(1), Action::Error, Action::Shift(4), ],
	[Action::Error, Action::Reduce(2), Action::Error, ],
	[Action::Error, Action::Reduce(3), Action::Error, ],
	[Action::Shift(5), Action::Error, Action::Shift(7), ],
	[Action::Reduce(1), Action::Error, Action::Reduce(1), ],
	[Action::Error, Action::Reduce(1), Action::Error, ],
];

static GOTO: [[GotoState; 2]; 10] = [
	[GotoState::State(3), GotoState::State(2), ],
	[GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, ],
	[GotoState::State(6), GotoState::Error, ],
	[GotoState::State(8), GotoState::Error, ],
	[GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, ],
	[GotoState::State(9), GotoState::Error, ],
	[GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, ],
];

fn main() {}
