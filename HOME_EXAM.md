# Home Exam D7050E
My answers to the questions of the home exam.

## Your syntax
### EBNF
```ebnf
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
	| FuncCall, ";" ;

FuncDef = "fn", Identifier, "(", FuncParams, ")", ["->", LitType], "{", Statement, "}" ;

FuncCall = Identifier, "(", FuncArgs, ")" ;

FuncArgs = VectorizeComma<Expr> ;

FuncParams = VectorizeComma<Param> ;

(* Creates vector of comma-seperated list of type T*)
VectorizeComma<T> = {T, ","}, [T] ;

Param = ["mut"], Var, ":", LitType ;

Return = "return", Expr ;

WhileStatement = "(", Expr, ")", "{" Statement "}" ;

IfStatement =
	"(", Expr, ")", "{", Statement	
	| "(", Expr, ")", "{", Statement, "} else {", Statement
	| "(", Expr, ")", "{", Statement, "} else if", IfStatement ;

(*Updating a variable*)
AssignValue = Var, "=", Expr ;

(*Declaring a new variable*)
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
	| FuncCall 
	| "(", Expr, ")" ;

ExprOp = "+" | "-" ;

ExprRelOp = "==" | "!=" | ">" | "<" | ">=" | "<=" ;

FactorLogOp = "&&" | "||" ;

FactorOp = "*" | "/" ;

UnaryOp = "-" ;

LitType = "bool" | "i32" ;

Bool = "true" | "false" ;

Var = Identifier ;

Identifier: = [a-zA-Z][a-zA-Z0-9_]* ;

Num = [0-9]+ ;
```	