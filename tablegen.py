# CLR parsing table generator

# terminals and non-terminals are treated as numbers (ids)
from sys import exit

EPSILON = "epsilon" # ε (empty string)
EOI = "EOI"
DOT = "."

# special parse table actions
ERROR = "ERR"
ACCEPT = "ACC"
RRCONFLICT = "r-r"
SRCONFLICT = "s-r"
WEIRDCONFLICT = "???"

class Item: # LR(1) item
	def __init__(self, LHS, RHS: list, lookahead) -> None:
		self.LHS = LHS
		self.RHS = RHS
		self.lookahead = lookahead
	def __repr__(self) -> str:
		return f"[{self.LHS} -> {' '.join(self.RHS)}, {self.lookahead}]"
	def __eq__(self, other):
		# we need this to ensure items with the same content but different mem. address aren't considered two different items
		return (isinstance(other, Item)
				and self.LHS == other.LHS
			   	and self.RHS == other.RHS
				and self.lookahead == other.lookahead)

	def __hash__(self):
		return hash((self.LHS, tuple(self.RHS), self.lookahead))
	def afterdot(self):
		i = 0
		for x in self.RHS:
			if x == DOT:
				if i == len(self.RHS) - 1:
					return (None, None)
				else: # returns tuple(Nterm after dot, its index)
					return ((self.RHS)[i+1], i+1)
			i += 1
	def shiftdot(self):
		new_RHS = list(self.RHS)
		new_RHS[self.afterdot()[1]], new_RHS[self.afterdot()[1]-1] = new_RHS[self.afterdot()[1]-1], new_RHS[self.afterdot()[1]]
		return Item(self.LHS, new_RHS, self.lookahead)

class Production:
	def __init__(self, LHS, RHS: list) -> None:
		self.LHS = LHS
		self.RHS = RHS
	def __repr__(self) -> str:
		return f"[{self.LHS} -> {' '.join(self.RHS)}]"
	def __eq__(self, other):
		# we need this to ensure productions with the same content but different mem. address aren't considered two different ones
		return (isinstance(other, Production)
				and self.LHS == other.LHS
				and self.RHS == other.RHS)
	def __hash__(self):
		return hash((self.LHS, tuple(self.RHS)))
def printset(s):
	print("[", end="")
	for i in s:
		print(i, end=", ")
	print("]")

calculator_syntax = {
	"name": "calculator",
	"start": "E",
	"terminals": ["Id", 'Plus', 'Mul', 'Lparen', 'Rparen'],
	"nonterminals": ["E", "T", "F"],
	"productions": [
		["E", 'E', 'Plus', 'T'], # represents the production E -> E + T
		["E", "T"],
		["T",'T', 'Mul', 'F'],
		["T", 'F'],
		["F", 'Id'],
		["F", 'Lparen', 'E', 'Rparen']
	]
}

SAUG = 'Augmented'

# grammar from json (= python dict-like structure)
def from_dict(grammar:dict) -> tuple: # returns (V, T, P, S)
	keys = grammar.keys()
	if not all([(x in keys) for x in ["name", "start", "terminals", "nonterminals", "productions"]]):
		print("Bad json file: missing either one of field name, start, terminals, nonterminals, or productions")
		exit()

	grammar["nonterminals"].append(SAUG) # augmented grammar
	grammar["terminals"].append(EOI)
	grammar["productions"].append([SAUG, grammar["start"]])

	return (set(grammar["nonterminals"]), set(grammar["terminals"]), {Production(x[0], x[1:]) for x in grammar["productions"]}, grammar["start"], grammar["name"])


# load grammar
V, T, P, S, syntax_name = from_dict(calculator_syntax)


#####################   DEFINE YOUR GRAMMAR HERE!   #######################
##### make sure to define S, SAUG!

# syntax_name = "calculator"

# S = "E"
# SAUG = "S'"

# T.update(["id", '+', '*', '(', ')'])
# V.update(["S'", "E", "T", "F"])
# P.update([
# 	Production("E", ['E', '+', 'T']),
# 	Production("E", ["T"]),
# 	Production("T", ['T', '*', 'F']),
# 	Production("T", ['F']),
# 	Production("F", ['id']),
# 	Production("F", ['(', 'E', ')'])
# ])


# Test 2
# syntax_name = "calculator"

# S = "S"
# SAUG = "S'"

# T.update(['c', 'd'])
# V.update(['C', 'D', 'S'])
# P.update([
# 	Production("S", ['C', 'C']),
# 	Production("C", ['c', 'C']),
# 	Production("C", ['d'])
# ])

#####################   DEFINE YOUR GRAMMAR ABOVE!  #######################


firstset = dict()
for nt in V:
	firstset[nt] = set()

def first(alpha) -> set:
	if isinstance(alpha, int) or isinstance(alpha, str):
		if alpha in T:
			return {alpha}
		elif alpha in V:
			return firstset[alpha]
		else:
			raise ValueError("input to FIRST contains an undefined symbol")
	elif isinstance(alpha, list):
		result = set()
		includeep = True
		for X in alpha:
			temp = first(X)
			result = result | (temp - {EPSILON})
			if not (EPSILON in temp):
				includeep = False
				break
		if includeep:
			result = result | {EPSILON}
		return result
	else:
		raise ValueError("alpha must be either symbol(int) or list of symbols (list)")

def compute_first():
	changed = True
	while changed:
		changed = False
		for production in P:
			old_size = len(firstset[production.LHS])
			new_symbols = first(production.RHS)
			unioned = firstset[production.LHS] | new_symbols
			if len(unioned) > old_size:
				firstset[production.LHS] = unioned
				changed = True

compute_first() # generates the pre-computed FIRST(a) for every a E V

def closure(I: set) -> set:
	closureset = I
	changed = True
	while changed:
		changed = False
		old_len = len(closureset)
		add_set = set()
		for item in closureset:
			B, index = item.afterdot()
			if B != None and B in V:
				beta = []
				for i in range(index+1, len(item.RHS)):
					beta.append(item.RHS[i])
				beta.append(item.lookahead)
				lookaheads = first(beta)
				for production in P:
					if production.LHS == B:
						for b in lookaheads-{EPSILON}:
							new_RHS = [DOT]
							new_RHS.extend(production.RHS)
							add_set.add(Item(B, new_RHS, b))
		closureset = closureset | add_set
		if len(closureset) > old_len:
			changed = True
	return closureset

def goto(I, X):
	gotoset = set()
	for item in I:
		sym, index = item.afterdot()
		if sym == X:
			gotoset.add(item.shiftdot())
	return closure(gotoset)

def buildcc() -> list:
	I0 = closure({Item(SAUG, [DOT, S], EOI)})
	C = [I0]

	worklist = [I0]

	while len(worklist) != 0:
		state = worklist.pop(0)
		for X in V|T:
			new = goto(state, X)
			if new not in C and len(new) != 0:
				C.append(new)
				worklist.append(new)
	return C

C = buildcc()

# we'll number all the production rules
Pnumbered = []
for p in P:
	Pnumbered.append(p)

def tablegen():
	action, gototab = [], [] # table [i, t] where i is a state number and t is a terminal symbol
	for i in range(0, len(C)): # initialize everything to error
		action.append({})
		gototab.append({})
		for t in T:
			action[i][t] = ERROR
		for v in V:
			if v != SAUG:
				gototab[i][v] = ERROR
	for i in range(0, len(C)): # for each state
		for item in C[i]:
			X, idx = item.afterdot()
			alpha = []
			if idx == None: alpha = list(item.RHS); alpha.pop(len(alpha)-1)
			else:
				for k in range(0, idx):
					alpha.append(item.RHS[k])

			if X != None: # dot isn't at the end
				if X in T:
					if action[i][X] != ERROR: action[i][X] = SRCONFLICT if action[i][X][0] == 'r' else f"s{C.index(goto(C[i], X))}"
					else: action[i][X] = f"s{C.index(goto(C[i], X))}" # the state number of GOTO(Ii, X)
				elif X in V and X != SAUG:
					gototab[i][X] = C.index(goto(C[i], X)) # the state number of GOTO(Ii, X)
			else: # dot is at the end, read everything
				if item.LHS != SAUG:
					if action[i][item.lookahead] != ERROR: action[i][item.lookahead] = RRCONFLICT if action[i][item.lookahead][0] == 'r' else SRCONFLICT
					else: action[i][item.lookahead] = f"r{Pnumbered.index(Production(item.LHS, alpha))}"
				else:
					if action[i][item.lookahead] != ERROR: action[i][item.lookahead] = WEIRDCONFLICT
					else: action[i][item.lookahead] = ACCEPT

	return (action, gototab)

action, gototab = tablegen()

Tnumbered, Vnumbered = [], []
for t in T: Tnumbered.append(t)
for v in V: Vnumbered.append(v)

Vnumbered.remove(SAUG)

def printtab():
	print("\n\nACITON")
	print("state\t", end='')
	for t in Tnumbered:
		print(f"{t}\t", end='')
	print("")
	print("---------------------------------------------------")
	i = 0
	for state in action:
		print(f"{i}\t", end='')
		i+=1
		for t in Tnumbered:
			print(f"{state[t]}\t", end='')
		print("\n")


	print("\n\nGOTO")
	print("state\t", end='')
	i=0
	for v in Vnumbered:
		print(f"{v}\t", end='')
	print("")
	print("---------------------------------------------------")
	for state in gototab:
		print(f"{i}\t", end='')
		i+=1
		for v in Vnumbered:
			print(f"{state[v]}\t", end='')
		print("")

def rustprint():
	hashmapt = {}
	hashmapv = {}

	print("""use std::collections::{HashMap, String, Vector};

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


struct Production(u32, Vector<u32>);""")

	print("""#[repr(u32)]
enum Terminal {""")
	for i, t in enumerate(Tnumbered):
		print(f"\t{t} = {i},")
		hashmapt[i] = t
	print("""\tCOUNT
};\n""")

	print("""#[repr(u32)]
enum Nonterminal {""")
	for i, t in enumerate(Vnumbered):
		print(f"\t{t} = {i},")
		hashmapv[i] = t
	print("""COUNT
};\n""")

	print(f"const rules: [Production; {len(Pnumbered)}] = [")
	for i, p in enumerate(Pnumbered):
		print(f"\tProduction({p.LHS}, vec![{', '.join(p.RHS)}]),")
	print("];")


	print(f"const actiontab: [[u32; {len(action)}]; {len(action[0])}] = [")
	for i, state in enumerate(action):
		print("\t[", end='')
		for t in Tnumbered:
			if state[t] == ERROR:
				toprint = 'Action::Error'
			elif state[t] == ACCEPT:
				toprint = 'Action::Accept'
			elif state[t][0] == 's' or state[t][0] == 'r':
				toprint = (f"Action::Reduce({state[t][1]})" if state[t][0] == 'r' else f"Action::Shift({state[t][1]})")
			else:
				raise ValueError


			print(f"{toprint}, ", end='')
		print("],")
	print("];")

	print(f"const gototab: [[u32; {len(gototab)}]; {len(gototab[0])}] = [")
	for i, state in enumerate(gototab):
		print('\t[', end='')
		for t in Vnumbered:
			if ERROR == state[t]:
				print("GotoState::Error, ", end='')
			else:
				print(f"GotoState::State({state[t]}), ", end='')
		print('],')
	print("];")

rustprint()
