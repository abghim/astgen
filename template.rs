use std::collections::{HashMap, String, Vector};

enum Action {
    Shift(u32),
    Reduce(u32),
    Accept,
    Error
}

/* debug: String for human-readable token output */
let SymbolMap: HashMap<u32, &str>;

SymbolMap.put(0, "EOI");
SymbolMap.put(1, "E");
...

struct Production(Symbol, Vector<u32>);

/*
 * Production(2, vec![1])
 */

const rules: [Production; NUMBER_OF_RULES] = [
    DEFINE_YOUR_RULES
];

const actiontab: [[HashMap<Symbol, Action>]; NUMBER_OF_STATES] = [
    /* symbol 1 */ [/* state 0 */ Action::Shift(3), /* state 1 */ Action::Shift(2), ...],
    /* symbol 2 */ [...],
    ...
]

/*
 * reference actiontab:
 * actiontab[symbol, state]
 */
