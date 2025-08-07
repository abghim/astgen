use std::collections::{HashMap, String, Vector};

enum Action {
    Shift(u32),
    Reduce(u32),
    Accept,
    Error
};

enum GotoState {
	State(u32),
	Error
};

struct Production(u32, Vector<u32>);

/*
 * Production(2, vec![2, 1])
 */


/*
 * terminals: 0, 1, 2, 3, 4, ...
 * nonterminals: 256, 257, ...
 */

#[repr(u32)]
enum Terminal {
	Multiplication = 0,
	Addition = 1,
	Identifier = 2,
	EndOfInput = 3,
	...,
	COUNT
};

#[repr(u32)]
enum Nonterminal {
	Expr = 0,
	Term = 1,
	Start = 2,
	...,
	COUNT
};

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
