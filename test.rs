use std::collections::HashMap;

enum Action {
	Shift(u32),
	Reduce(u32),
	Accept,
	Error
}

enum GotoState {
	State(u32),
	Error
}


struct Production(u32, Vec<u32>);

#[repr(u32)]
enum Terminal {
	Rparen = 0,
	Id = 1,
	Lparen = 2,
	EOI = 3,
	Mul = 4,
	Plus = 5,
	COUNT
}

#[repr(u32)]
enum Nonterminal {
	T = 0,
	F = 1,
	E = 2,
	COUNT
}


const Augmented: u32 = Nonterminal::COUNT as u32;

use Nonterminal::*;
use Terminal::*;

let rules: [Production; 7] = [
	Production(T as u32, vec![T as u32, Mul as u32, F as u32]),
	Production(E as u32, vec![E as u32, Plus as u32, T as u32]),
	Production(Augmented as u32, vec![E as u32]),
	Production(E as u32, vec![T as u32]),
	Production(F as u32, vec![Id as u32]),
	Production(T as u32, vec![F as u32]),
	Production(F as u32, vec![Lparen as u32, E as u32, Rparen as u32]),
];

const actiontab: [[Action; 6]; 22] = [
	[Action::Error, Action::Shift(2), Action::Shift(3), Action::Error, Action::Error, Action::Error, ],
	[Action::Error, Action::Error, Action::Error, Action::Reduce(3), Action::Shift(6), Action::Reduce(3), ],
	[Action::Error, Action::Error, Action::Error, Action::Reduce(4), Action::Reduce(4), Action::Reduce(4), ],
	[Action::Error, Action::Shift(8), Action::Shift(9), Action::Error, Action::Error, Action::Error, ],
	[Action::Error, Action::Error, Action::Error, Action::Reduce(5), Action::Reduce(5), Action::Reduce(5), ],
	[Action::Error, Action::Error, Action::Error, Action::Accept, Action::Error, Action::Shift(1), ],
	[Action::Error, Action::Shift(2), Action::Shift(3), Action::Error, Action::Error, Action::Error, ],
	[Action::Reduce(3), Action::Error, Action::Error, Action::Error, Action::Shift(1), Action::Reduce(3), ],
	[Action::Reduce(4), Action::Error, Action::Error, Action::Error, Action::Reduce(4), Action::Reduce(4), ],
	[Action::Error, Action::Shift(8), Action::Shift(9), Action::Error, Action::Error, Action::Error, ],
	[Action::Reduce(5), Action::Error, Action::Error, Action::Error, Action::Reduce(5), Action::Reduce(5), ],
	[Action::Shift(1), Action::Error, Action::Error, Action::Error, Action::Error, Action::Shift(1), ],
	[Action::Error, Action::Shift(2), Action::Shift(3), Action::Error, Action::Error, Action::Error, ],
	[Action::Error, Action::Error, Action::Error, Action::Reduce(0), Action::Reduce(0), Action::Reduce(0), ],
	[Action::Error, Action::Shift(8), Action::Shift(9), Action::Error, Action::Error, Action::Error, ],
	[Action::Shift(2), Action::Error, Action::Error, Action::Error, Action::Error, Action::Shift(1), ],
	[Action::Error, Action::Error, Action::Error, Action::Reduce(6), Action::Reduce(6), Action::Reduce(6), ],
	[Action::Error, Action::Shift(8), Action::Shift(9), Action::Error, Action::Error, Action::Error, ],
	[Action::Error, Action::Error, Action::Error, Action::Reduce(1), Action::Shift(6), Action::Reduce(1), ],
	[Action::Reduce(0), Action::Error, Action::Error, Action::Error, Action::Reduce(0), Action::Reduce(0), ],
	[Action::Reduce(6), Action::Error, Action::Error, Action::Error, Action::Reduce(6), Action::Reduce(6), ],
	[Action::Reduce(1), Action::Error, Action::Error, Action::Error, Action::Shift(1), Action::Reduce(1), ],
];

const gototab: [[GotoState; 3]; 22] = [
	[GotoState::State(1), GotoState::State(4), GotoState::State(5), ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::State(7), GotoState::State(10), GotoState::State(11), ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::State(13), GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::State(7), GotoState::State(10), GotoState::State(15), ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::State(18), GotoState::State(4), GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::State(19), GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::State(21), GotoState::State(10), GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
	[GotoState::Error, GotoState::Error, GotoState::Error, ],
];

fn main() {}
