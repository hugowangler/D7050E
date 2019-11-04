# Home Exam D7050E
Answers to the questions of the home exam.

## Your syntax
- Give an as complete as possible EBNF grammar for your language
```EBNF
Program = {FuncDef} ;

Statement = 
	Keyword, Statement
	| Keyword ;

Keyword = 
	"let", Assign, ";"
	| "if", IfStatement 
	| "while", WhileStatement 
	| AssignValue, ";"
	| Return, ";"
	| FuncCall, ";"
	| Print, ";" ;

FuncDef = "fn", Identifier, "(", FuncParams, ")", ["->", LitType], "{", Statement, "}" ;

FuncCall = Identifier, "(", FuncArgs, ")" ;

FuncArgs = VectorizeComma<Expr> ;

FuncParams = VectorizeComma<Param> ;

(* Creates vector of comma-seperated list of type T *)
VectorizeComma<T> = {T, ","}, [T] ;

Param = ["mut"], Var, ":", LitType ;

Return = "return", Expr ;

(* Used for debugging *)
Print = "print(", Expr, ")" ;

WhileStatement = "(", Expr, ")", "{", Statement, "}" ;

IfStatement =
	"(", Expr, ")", "{", Statement, "}"	
	| "(", Expr, ")", "{", Statement, "} else {", Statement, "}"
	| "(", Expr, ")", "{", Statement, "} else if", IfStatement ;

(* Updating a variable *)
AssignValue = Var, "=", Expr ;

(* Declaring a new variable *)
Assign = AssignBinding, Expr ;

AssignBinding = ["mut"], Var, ":", LitType, "=" ;

Expr = 
	Expr, ExprOp, Factor 
	| Expr, ExprRelOp, Factor 
	| Factor ;

Factor = 
	Factor, FactorOp, Term 
	| Factor, FactorLogOp, Term 
	| UnaryOp, Term
	| Term ;

Term = 
	Num 
	| Var 
	| Bool
	| _String
	| FuncCall 
	| "(", Expr, ")" ;

ExprOp = "+" | "-" ;

ExprRelOp = "==" | "!=" | ">" | "<" | ">=" | "<=" ;

FactorLogOp = "&&" | "||" ;

FactorOp = "*" | "/" ;

UnaryOp = "-" ;

LitType = "bool" | "i32" ;

(* Used to help with printing when debugging *)
_String = """, Identifier, """ ;

Bool = "true" | "false" ;

Var = Identifier ;

Identifier: = [a-zA-Z][a-zA-Z0-9_]* ;

Num = [0-9]+ ;
```	
- Give an example that showcases all rules of your EBNF. The program should "do" something as used in the next excercise.

```rust
fn fibonacci(n: i32) -> i32 {
	if (n == 0) {
		return 0;
	} else if (n == 1) {
		return 1;
	} else {
		return fibonacci(n - 2) + fibonacci(n - 1);
	}
}

fn main() {
	let fib_20: i32 = fibonacci(20);
	if (fib_20 == 6765) {
		print("fib_20=6765");
	}

	if (fib_20 > 7000) {
		print("fib_20>7000");
	} else {
		print("fib_20<7000");
	}

	let mut and_res: bool = and_op(true, true);

	while (and_res) {
		and_res = false;
	}
	print("and_res_after_while_loop");
	print(and_res);

	let unary_res: i32 = unary_op(1);
	if (unary_res == -1) {
		print("unary_ok");
	} else {
		print("unary_failed");
	}

	let prec_test: i32 = prec_test();
	if (prec_test == 27) {
		print("prec_ok");
	} else {
		print("prec_failed");
	}
}

fn and_op(a: bool, b: bool) -> bool {
	return a && b;
}

fn unary_op(x: i32) -> i32 {
	return -x;
}

fn prec_test() -> i32 {
	return 2 * 10 - 3 + 2 * 5;
}
```

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

The parser was implemented using the [LALRPOP](https://github.com/lalrpop/lalrpop) parser generator. By following the LALRPOP tutorial to learn the syntax and some of the functionality a expression parser was implemented. Thus I didn't implemented the expression parser myself but I built upon it and extended it to implement the rest of the parser for my language. 

When it comes precedence of the expression parser the tutorial already had it implemented for numerical expressions. When I extended the parser to support logical and relational operations the precedence of those operations works if you don't mix logical and relation operations together. So for the most part in my language sub expressions have to be parantesized if they contain those operations.

No error messages, except the already existing error messages from LALRPOP, have been implemented in the parser. And thus no error recovery is possible since the parser just forwards the LALRPOP error message and stops if an error occurs while parsing the program.

## Your semantics
- Give an as complete as possible Structural Operetional Semantics (SOS) for your language

Addition:\
$\frac{\lang e1, \sigma \rang \ \Downarrow \ n1 \; \lang e2, \sigma \rang \ \Downarrow \ n2}{\lang e1 + e2, \sigma \rang \ \Downarrow \ n1 + n2}$

Subtraction:\
$\frac{\lang e1, \sigma \rang \ \Downarrow \ n1 \; \lang e2, \sigma \rang \ \Downarrow \ n2}{\lang e1 - e2, \sigma \rang \ \Downarrow \ n1 - n2}$

Multiplication:\
$\frac{\lang e1, \sigma \rang \ \Downarrow \ n1 \; \lang e2, \sigma \rang \ \Downarrow \ n2}{\lang e1 * e2, \sigma \rang \ \Downarrow \ n1 * n2}$

Division:\
$\frac{\lang e1, \sigma \rang \ \Downarrow \ n1 \; \lang e2, \sigma \rang \ \Downarrow \ n2}{\lang e1 / e2, \sigma \rang \ \Downarrow \ n1 / n2}$

AND:\
$\frac{\lang e1, \sigma \rang \Downarrow \ b1 \; \lang e2, \sigma \rang \Downarrow \ b1}{\lang e1 \ \&\& \ e2, \sigma \rang \ \Downarrow \ b1 \ \text{AND} \ b2}$

OR:\
$\frac{\lang e1, \sigma \rang \Downarrow \ b1 \; \lang e2, \sigma \rang \Downarrow \ b2}{\lang e1 \ || \ e2, \sigma \rang \ \Downarrow \ b1 \ \text{OR} \ b2}$

Less than (<):\
$\frac{\lang e1, \sigma \rang \Downarrow \ b1 \; \lang e2, \sigma \rang \Downarrow \ b2}{\lang e1 \ < \ e2, \sigma \rang \ \Downarrow \ b1 \ < \ b2}$

Greaten than (>):\
$\frac{\lang e1, \sigma \rang \Downarrow \ b1 \; \lang e2, \sigma \rang \Downarrow \ b2}{\lang e1 \ > \ e2, \sigma \rang \ \Downarrow \ b1 \ > \ b2}$

Less than or equals (<=):\
$\frac{\lang e1, \sigma \rang \Downarrow \ b1 \; \lang e2, \sigma \rang \Downarrow \ b2}{\lang e1 \ <= \ e2, \sigma \rang \ \Downarrow \ b1 \ <= \ b2}$

Greater than or equals (>=):\
$\frac{\lang e1, \sigma \rang \Downarrow \ b1 \; \lang e2, \sigma \rang \Downarrow \ b2}{\lang e1 \ >= \ e2, \sigma \rang \ \Downarrow \ b1 \ >= \ b2}$

Equals (==):\
$\frac{\lang e1, \sigma \rang \Downarrow \ b1 \; \lang e2, \sigma \rang \Downarrow \ b2}{\lang e1 \ == \ e2, \sigma \rang \ \Downarrow \ b1 \ == \ b2}$

Not equals (!=):\
$\frac{\lang e1, \sigma \rang \Downarrow \ b1 \; \lang e2, \sigma \rang \Downarrow \ b2}{\lang e1 \ != \ e2, \sigma \rang \ \Downarrow \ b1 \ != \ b2}$