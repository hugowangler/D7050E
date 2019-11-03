# Home Exam D7050E
Answers to the questions of the home exam.

## 1. Syntax
- EBNF grammar of the language
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
	| Term ;

Term = 
	Num 
	| UnaryOp, Num 
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
- Example code that showcases all of the rules of the EBNF

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
	return fibonacci(20);
}
```