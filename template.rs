use std::collections::{HashMap, String, Vector};

enum Action {
    Shift(u32),
    Reduce(u32),
    Accept,
    Error
}

struct Production(u32, Vector<u32>);

/*
 * Production(2, vec![2, 1])
 */

const rules: [Production; NUMBER_OF_RULES] = [
    /* DEFINE_YOUR_RULES */
];

const actiontab: [[u32; NUMBER_OF_SYMBOLS]; NUMBER_OF_STATES] = [
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
