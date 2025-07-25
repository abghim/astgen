# CLR parsing table generator

# terminals and non-terminals are treated as numbers (ids)
EPSILON = "epsilon" # ε (empty string)
EOI = "EOI"
DOT = "."

# special parse table actions
ERROR = "0"
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


# define the grammar

V = set() # non-terminals
T = set() # terminals
P = set() # production
S = 2
Saug = 3

syntax_name = "syntax"
#####################   DEFINE YOUR GRAMMAR HERE!   #######################
##### make sure to define S, Saug!
syntax_name = "calculator"

S = "E"
Saug = "S'"

T.update(["id", '+', '*', '(', ')'])
V.update(["S'", "E", "T", "F"])
P.update([
	Production("E", ['E', '+', 'T']),
	Production("E", ["T"]),
	Production("T", ['T', '*', 'F']),
	Production("T", ['F']),
	Production("F", ['id']),
	Production("F", ['(', 'E', ')'])
])

#####################   DEFINE YOUR GRAMMAR ABOVE!  #######################

P.add(Production("S'", ['E'])) # make sure to augment grammar
T.add(EOI)

firstset = dict()
for nt in V:
	firstset[nt] = set() # initialize to empty set

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
	I0 = closure({Item(Saug, [DOT, S], EOI)})
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
				elif X in V:
					gototab[i][X] = C.index(goto(C[i], X)) # the state number of GOTO(Ii, X)
			else: # dot is at the end, read everything
				if item.LHS != Saug:
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


with open(f"{syntax_name}.map", "w") as f:
	ti = 0
	for t in Tnumbered:
		f.write(f"{t}\t{ti+10}\n")
		ti += 1
	f.write("%%\n")
	vi = 0
	lent = len(Tnumbered)
	for v in Vnumbered:
		f.write(f"{v}\t{vi+10+lent}\n")
		vi += 1

with open(f"{syntax_name}.pstb", "w") as f:
	content = ""
	ti = 0
	for t in Tnumbered:
		content += f"{ti+10}\t"
		ti += 1
	content += "\n"
	
	for state in action:
		for t in Tnumbered:
			content += f"{state[t]}\t"
		content += "\n"
	content += "%%\n"
	vi = 0
	for v in Vnumbered:
		content += f"{vi+10+len(Tnumbered)}\t"
		vi += 1
	content += "\n"
	for state in gototab:
		for v in Vnumbered:
			content += f"{state[v]}\t"
		content += "\n"
	f.write(content)
printtab()
