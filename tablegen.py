# CLR parsing table generator

# terminals and non-terminals are treated as numbers (ids)
EPSILON = "epsilon" # ε (empty string)
EOI = "EOI"
DOT = "."

class Item: # LR(1) item
	def __init__(self, LHS, RHS: list, lookahead) -> None:
		self.LHS = LHS
		self.RHS = RHS
		self.lookahead = lookahead
	def __repr__(self) -> str:
		return f"[{self.LHS} -> {' '.join(self.RHS)}, {self.lookahead}]"

class Production:
    def __init__(self, LHS, RHS: list) -> None:
        self.LHS = LHS
        self.RHS = RHS
    def __repr__(self) -> str:
        return f"[{self.LHS} -> {' '.join(self.RHS)}]"
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

#####################   DEFINE YOUR GRAMMAR HERE!   #######################
##### make sure to define S, Saug!

S = "E"
Saug = "S'"

T.update(["id", '+', '*', '(', ')', EOI])
V.update(["S'", "E", "T", "F"])
P.update([
	Production("S'", ['E']),
	Production("E", ['E', '+', 'T']),
	Production("E", ["T"]),
	Production("T", ['T', '*', 'F']),
	Production("T", ['F']),
	Production("F", ['id']),
	Production("F", ['(', 'E', ')'])
])


#####################   DEFINE YOUR GRAMMAR ABOVE!  #######################



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

compute_first()

def closure(I: set) -> set:
	closureset = I
	

