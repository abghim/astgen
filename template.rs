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
};

#[repr(u32)]
enum Terminal {
	Multiplication = 0,
	Addition = 1,
	Identifier = 2,
	EndOfInput = 3,
	...,
	COUNT
}

#[repr(u32)]
enum Nonterminal {
	Expr = 0,
	Term = 1,
	Start = 2,
	...,
	COUNT
}

const Augmented = (Nonterminal::COUNT as u32);


use Nonterminal::*;
use Terminal::*;

/*
 * Nonterminals as u32
 */

const rules: [Production; NUMBER_OF_RULES] = [
    /* DEFINE_YOUR_RULES */
];

const actiontab: [[Action; NUMBER_OF_SYMBOLS]; NUMBER_OF_STATES] = [
    /* state 0 */ [/* sym 0 */ Action::Shift(3), /* sym 1 */ Action::Shift(2), ...],
    /* state 1 */ [...],
    ...
];

const gototab: [[u32; NUMBER_OF_SYMBOLS]; NUMBER_OF_STATES] = [
    /* define it here */
];

/*
 * reference actiontab:
 * actiontab[state][symbol]
 */

fn main() {
    /* debug: String for human-readable token output */
    let SymbolMap: HashMap<u32, &str>;

    SymbolMap.put(0, "EOI");
    SymbolMap.put(1, "E");
    /* ... */
}

/*
 * To sum it up:
 * - boilerplate (defs)
 * - rules list
 * - actiontab
 * - gototab
 * - main + symbolmap (debug)
 * - PARSING LOGIC
 */
