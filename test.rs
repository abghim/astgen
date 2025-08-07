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
	Plus = 0,
	Mul = 1,
	Id = 2,
	EOI = 3,
	Rparen = 4,
	Lparen = 5,
	COUNT
}

#[repr(u32)]
enum Nonterminal {
	T = 0,
	F = 1,
	E = 2,
	Augmented
}


use Nonterminal::*;
use Terminal::*;

static RULES: [Production; 7] = [
	Production {lhs:E as u32, rhs:&[T as u32]},
	Production {lhs:T as u32, rhs:&[F as u32]},
	Production {lhs:F as u32, rhs:&[Id as u32]},
	Production {lhs:F as u32, rhs:&[Lparen as u32, E as u32, Rparen as u32]},
	Production {lhs:E as u32, rhs:&[E as u32, Plus as u32, T as u32]},
	Production {lhs:T as u32, rhs:&[T as u32, Mul as u32, F as u32]},
	Production {lhs:Augmented as u32, rhs:&[E as u32]},
];

static ACTION: [[Action; 6]; 22] = [
	[Action::Error, Action::Error, Action::Shift(3), Action::Error, Action::Error, Action::Shift(5), ],
	[Action::Reduce(0), Action::Shift(6), Action::Error, Action::Reduce(0), Action::Error, Action::Error, ],
	[Action::Shift(7), Action::Error, Action::Error, Action::Accept, Action::Error, Action::Error, ],
	[Action::Reduce(2), Action::Reduce(2), Action::Error, Action::Reduce(2), Action::Error, Action::Error, ],
	[Action::Reduce(1), Action::Reduce(1), Action::Error, Action::Reduce(1), Action::Error, Action::Error, ],
	[Action::Error, Action::Error, Action::Shift(1), Action::Error, Action::Error, Action::Shift(1), ],
	[Action::Error, Action::Error, Action::Shift(3), Action::Error, Action::Error, Action::Shift(5), ],
	[Action::Error, Action::Error, Action::Shift(3), Action::Error, Action::Error, Action::Shift(5), ],
	[Action::Reduce(0), Action::Shift(1), Action::Error, Action::Error, Action::Reduce(0), Action::Error, ],
	[Action::Shift(1), Action::Error, Action::Error, Action::Error, Action::Shift(1), Action::Error, ],
	[Action::Reduce(2), Action::Reduce(2), Action::Error, Action::Error, Action::Reduce(2), Action::Error, ],
	[Action::Reduce(1), Action::Reduce(1), Action::Error, Action::Error, Action::Reduce(1), Action::Error, ],
	[Action::Error, Action::Error, Action::Shift(1), Action::Error, Action::Error, Action::Shift(1), ],
	[Action::Reduce(5), Action::Reduce(5), Action::Error, Action::Reduce(5), Action::Error, Action::Error, ],
	[Action::Reduce(4), Action::Shift(6), Action::Error, Action::Reduce(4), Action::Error, Action::Error, ],
	[Action::Error, Action::Error, Action::Shift(1), Action::Error, Action::Error, Action::Shift(1), ],
	[Action::Error, Action::Error, Action::Shift(1), Action::Error, Action::Error, Action::Shift(1), ],
	[Action::Reduce(3), Action::Reduce(3), Action::Error, Action::Reduce(3), Action::Error, Action::Error, ],
	[Action::Shift(1), Action::Error, Action::Error, Action::Error, Action::Shift(2), Action::Error, ],
	[Action::Reduce(5), Action::Reduce(5), Action::Error, Action::Error, Action::Reduce(5), Action::Error, ],
	[Action::Reduce(4), Action::Shift(1), Action::Error, Action::Error, Action::Reduce(4), Action::Error, ],
	[Action::Reduce(3), Action::Reduce(3), Action::Error, Action::Error, Action::Reduce(3), Action::Error, ],
];

static GOTO: [[GotoState; 3]; 22] = [
	[GotoState::State(1), GotoState::State(4), GotoState::State(2), ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::State(8), GotoState::State(11), GotoState::State(9), ],
	[GotoState::Error, GotoState::State(13), GotoState::Error, ],
	[GotoState::State(14), GotoState::State(4), GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::State(8), GotoState::State(11), GotoState::State(18), ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::State(19), GotoState::Error, ],
	[GotoState::State(20), GotoState::State(11), GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
];

fn main() {}
