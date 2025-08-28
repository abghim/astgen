use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Action {
    Shift(u32),
    Reduce(u32),
    Accept,
    SRConflict,
    RRConflict,
    Error,
}

#[derive(Copy, Clone)]
enum GotoState {
    State(u32),
    Error,
}

#[derive(Copy, Clone)]
struct Production {
    lhs: u32,
    rhs: &'static [u32],
}

#[repr(u32)]
enum Terminal {
    EOI = 0,
    Lparen = 1,
    Rparen = 2,
    Id = 3,
    Plus = 4,
    Mul = 5,
    COUNT,
}

#[repr(u32)]
enum Nonterminal {
    F = 0,
    T = 1,
    E = 2,
    Augmented,
}

use Nonterminal::*;
use Terminal::*;

static RULES: [Production; 7] = [
    Production {
        lhs: E as u32,
        rhs: &[E as u32, Plus as u32, T as u32],
    },
    Production {
        lhs: T as u32,
        rhs: &[F as u32],
    },
    Production {
        lhs: E as u32,
        rhs: &[T as u32],
    },
    Production {
        lhs: Augmented as u32,
        rhs: &[E as u32],
    },
    Production {
        lhs: T as u32,
        rhs: &[T as u32, Mul as u32, F as u32],
    },
    Production {
        lhs: F as u32,
        rhs: &[Lparen as u32, E as u32, Rparen as u32],
    },
    Production {
        lhs: F as u32,
        rhs: &[Id as u32],
    },
];

static ACTION: [[Action; 6]; 22] = [
    [
        Action::Error,
        Action::Shift(1),
        Action::Error,
        Action::Shift(3),
        Action::Error,
        Action::Error,
    ],
    [
        Action::Error,
        Action::Shift(6),
        Action::Error,
        Action::Shift(8),
        Action::Error,
        Action::Error,
    ],
    [
        Action::Reduce(1),
        Action::Error,
        Action::Error,
        Action::Error,
        Action::Reduce(1),
        Action::Reduce(1),
    ],
    [
        Action::Reduce(6),
        Action::Error,
        Action::Error,
        Action::Error,
        Action::Reduce(6),
        Action::Reduce(6),
    ],
    [
        Action::Accept,
        Action::Error,
        Action::Error,
        Action::Error,
        Action::Shift(1),
        Action::Error,
    ],
    [
        Action::Reduce(2),
        Action::Error,
        Action::Error,
        Action::Error,
        Action::Reduce(2),
        Action::Shift(1),
    ],
    [
        Action::Error,
        Action::Shift(6),
        Action::Error,
        Action::Shift(8),
        Action::Error,
        Action::Error,
    ],
    [
        Action::Error,
        Action::Error,
        Action::Reduce(1),
        Action::Error,
        Action::Reduce(1),
        Action::Reduce(1),
    ],
    [
        Action::Error,
        Action::Error,
        Action::Reduce(6),
        Action::Error,
        Action::Reduce(6),
        Action::Reduce(6),
    ],
    [
        Action::Error,
        Action::Error,
        Action::Shift(1),
        Action::Error,
        Action::Shift(1),
        Action::Error,
    ],
    [
        Action::Error,
        Action::Error,
        Action::Reduce(2),
        Action::Error,
        Action::Reduce(2),
        Action::Shift(1),
    ],
    [
        Action::Error,
        Action::Shift(1),
        Action::Error,
        Action::Shift(3),
        Action::Error,
        Action::Error,
    ],
    [
        Action::Error,
        Action::Shift(1),
        Action::Error,
        Action::Shift(3),
        Action::Error,
        Action::Error,
    ],
    [
        Action::Error,
        Action::Error,
        Action::Shift(1),
        Action::Error,
        Action::Shift(1),
        Action::Error,
    ],
    [
        Action::Error,
        Action::Shift(6),
        Action::Error,
        Action::Shift(8),
        Action::Error,
        Action::Error,
    ],
    [
        Action::Reduce(5),
        Action::Error,
        Action::Error,
        Action::Error,
        Action::Reduce(5),
        Action::Reduce(5),
    ],
    [
        Action::Error,
        Action::Shift(6),
        Action::Error,
        Action::Shift(8),
        Action::Error,
        Action::Error,
    ],
    [
        Action::Reduce(0),
        Action::Error,
        Action::Error,
        Action::Error,
        Action::Reduce(0),
        Action::Shift(1),
    ],
    [
        Action::Reduce(4),
        Action::Error,
        Action::Error,
        Action::Error,
        Action::Reduce(4),
        Action::Reduce(4),
    ],
    [
        Action::Error,
        Action::Error,
        Action::Reduce(5),
        Action::Error,
        Action::Reduce(5),
        Action::Reduce(5),
    ],
    [
        Action::Error,
        Action::Error,
        Action::Reduce(0),
        Action::Error,
        Action::Reduce(0),
        Action::Shift(1),
    ],
    [
        Action::Error,
        Action::Error,
        Action::Reduce(4),
        Action::Error,
        Action::Reduce(4),
        Action::Reduce(4),
    ],
];

static GOTO: [[GotoState; 3]; 22] = [
    [
        GotoState::State(2),
        GotoState::State(5),
        GotoState::State(4),
    ],
    [
        GotoState::State(7),
        GotoState::State(10),
        GotoState::State(9),
    ],
    [GotoState::Error, GotoState::Error, GotoState::Error],
    [GotoState::Error, GotoState::Error, GotoState::Error],
    [GotoState::Error, GotoState::Error, GotoState::Error],
    [GotoState::Error, GotoState::Error, GotoState::Error],
    [
        GotoState::State(7),
        GotoState::State(10),
        GotoState::State(13),
    ],
    [GotoState::Error, GotoState::Error, GotoState::Error],
    [GotoState::Error, GotoState::Error, GotoState::Error],
    [GotoState::Error, GotoState::Error, GotoState::Error],
    [GotoState::Error, GotoState::Error, GotoState::Error],
    [GotoState::State(2), GotoState::State(17), GotoState::Error],
    [GotoState::State(18), GotoState::Error, GotoState::Error],
    [GotoState::Error, GotoState::Error, GotoState::Error],
    [GotoState::State(7), GotoState::State(20), GotoState::Error],
    [GotoState::Error, GotoState::Error, GotoState::Error],
    [GotoState::State(21), GotoState::Error, GotoState::Error],
    [GotoState::Error, GotoState::Error, GotoState::Error],
    [GotoState::Error, GotoState::Error, GotoState::Error],
    [GotoState::Error, GotoState::Error, GotoState::Error],
    [GotoState::Error, GotoState::Error, GotoState::Error],
    [GotoState::Error, GotoState::Error, GotoState::Error],
];

enum StackState {
    State(u32),
    T(Terminal),
    V(Nonterminal),
}

fn main() {
    let mut stack = vec![StackState::State(0)];
    let input = &[Lparen, Id, Plus, Id, Rparen, Mul, Id, EOI];

    let mut ip: u64 = 0;

    // let mut s: u32 = 0; // state
    let mut a: Terminal = input[0]; // symbol pointed by a

    while ip < input.len() {
        if let Some(s) = stack.last() {
            a = input[ip];
            match ACTION[s.unwrap()][a as u32] {
                Action::Shift(n) => {
                    stack.push(StackState::T(a));
                    stack.push(StackState::State(n));
                    println!("shift: {n}");
                    ip = ip + 1;
                }

                Action::Reduce(r) => {
                    stack.truncate(stack.len() - 2 * RULES[r].rhs.len());
                    stack.push(StackState::V(RULES[r].rhs.len())) // SOLVE: ISSUE: RULES appear as u32
                }
            }
        }
    }
}
