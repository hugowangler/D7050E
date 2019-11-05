# Home Exam D7050E
Answers to the questions of the home exam.

# Your syntax
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

# Your semantics
- Give an as complete as possible Structural Operetional Semantics (SOS) for your language
 
## Operators
Given a expression $e$ in the state $\sigma$  that evaluates to a value $n$ or boolean $b$ the following can be said for the operators in the language

### Arithmetic
Addition:\
$\frac{\lang e1, \sigma \rang \ \Downarrow \ n1 \; \lang e2, \sigma \rang \ \Downarrow \ n2}{\lang e1 + e2, \sigma \rang \ \Downarrow \ n1 + n2}$

Subtraction:\
$\frac{\lang e1, \sigma \rang \ \Downarrow \ n1 \; \lang e2, \sigma \rang \ \Downarrow \ n2}{\lang e1 - e2, \sigma \rang \ \Downarrow \ n1 - n2}$

Multiplication:\
$\frac{\lang e1, \sigma \rang \ \Downarrow \ n1 \; \lang e2, \sigma \rang \ \Downarrow \ n2}{\lang e1 * e2, \sigma \rang \ \Downarrow \ n1 * n2}$

Division:\
$\frac{\lang e1, \sigma \rang \ \Downarrow \ n1 \; \lang e2, \sigma \rang \ \Downarrow \ n2}{\lang e1 / e2, \sigma \rang \ \Downarrow \ n1 / n2}$

Unary:\
$\frac{\lang e1, \sigma \rang \ \Downarrow \ n1}{\lang -e1, \sigma \rang \ \Downarrow \ -n1}$

### Boolean
AND:\
$\frac{\lang e1, \sigma \rang \Downarrow \ b1 \; \lang e2, \sigma \rang \Downarrow \ b1}{\lang e1 \ \&\& \ e2, \sigma \rang \ \Downarrow \ b1 \ \text{AND} \ b2}$

OR:\
$\frac{\lang e1, \sigma \rang \Downarrow \ b1 \; \lang e2, \sigma \rang \Downarrow \ b2}{\lang e1 \ || \ e2, \sigma \rang \ \Downarrow \ b1 \ \text{OR} \ b2}$

Less than (<):\
$\frac{\lang e1, \sigma \rang \Downarrow \ n1 \; \lang e2, \sigma \rang \Downarrow \ n2}{\lang e1 \ < \ e2, \sigma \rang \ \Downarrow \ n1 \ < \ n2}$

Greaten than (>):\
$\frac{\lang e1, \sigma \rang \Downarrow \ n1 \; \lang e2, \sigma \rang \Downarrow \ n2}{\lang e1 \ > \ e2, \sigma \rang \ \Downarrow \ n1 \ > \ n2}$

Less than or equals (<=):\
$\frac{\lang e1, \sigma \rang \Downarrow \ n1 \; \lang e2, \sigma \rang \Downarrow \ n2}{\lang e1 \ <= \ e2, \sigma \rang \ \Downarrow \ n1 \ <= \ n2}$

Greater than or equals (>=):\
$\frac{\lang e1, \sigma \rang \Downarrow \ n1 \; \lang e2, \sigma \rang \Downarrow \ n2}{\lang e1 \ >= \ e2, \sigma \rang \ \Downarrow \ n1 \ >= \ n2}$

Equals (==):\
$\frac{\lang e1, \sigma \rang \Downarrow \ n1 \; \lang e2, \sigma \rang \Downarrow \ n2}{\lang e1 \ == \ e2, \sigma \rang \ \Downarrow \ n1 \ == \ n2}$

Not equals (!=):\
$\frac{\lang e1, \sigma \rang \Downarrow \ n1 \; \lang e2, \sigma \rang \Downarrow \ n2}{\lang e1 \ != \ e2, \sigma \rang \ \Downarrow \ n1 \ != \ n2}$

## Commands

**if** statement:

$\frac{\lang b, \sigma \rang \ \Downarrow \ \text{true} \ \lang c1, \sigma \rang \ \Downarrow \ \sigma'}{\lang \text{if } b \text{ then } c1 \text{ else } c2, \sigma \rang \ \Downarrow \ \sigma'}$

$\frac{\lang b, \sigma \rang \ \Downarrow \ \text{false} \ \lang c2, \sigma \rang \ \Downarrow \ \sigma''}{\lang \text{if } b \text{ then } c1 \text{ else } c2, \sigma \rang \ \Downarrow \ \sigma''}$

Not sure how to do this for elseif

**while** statement:

$\frac{\lang b, \sigma \rang \ \Downarrow \ \text{false}}{\text{while } b \text{ do } c, \sigma \rang \ \Downarrow \ \sigma}$

$\frac{\lang b, \sigma \rang \ \Downarrow \ \text{true} \quad \lang c, \sigma \rang \ \Downarrow \ \sigma' \quad \lang \text{while } b \text{ do } c,\sigma' \rang \ \Downarrow \ \sigma''}{\lang \text{while } b \text{ do } c, \sigma \rang \ \Downarrow \ \sigma''}$

assignment:\
$\frac{}{\lang x := n, \sigma \rang \ \Downarrow \ \sigma [x := n]}$

functions: \
Given a list of arguments $\overrightarrow{v} = [v_1, \ldots, v_n]$ the call of function $p$ will evaulate to a value $n$ (the return value, if any) according to

$\frac{\lang p, \sigma \rang}{\lang p(\overrightarrow{v}), \sigma \rang \ \Darr \ \lang n, \sigma \rang}$

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

I implemented the interpreter myself and the interpreter executes programs according to the SOS defined above. Additionaly it can also handle else-if statements, scoping and contexts (different variable values depending on the scope and context etc.). 

The interpreter also panics when encountering a evaluation error. For example if the program contains 
```rust
let x: i32 = true + false;
```
the interpreter will panic with the message:
```rust
"Left or right part of number expression not a number found: left = Value(Bool(true)) and right = Value(Bool(false))"
 ```
 
 this at least gives the user some context of what went wrong while interpreting the program. Note that the error handling is solved better by the type checker since a program that passed the type checker always will be fully executed without evaluation errors.

 # Your type checker
- Give an as complete as possible set of Type Checking Rules for your language (those rules look very much like the SOS rules, but over types not values)

## Operators



- Demonstrate each "type rule" by an example

asdasd

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

The type checker was fully implemented by me and it rejects ill-typed programs according to the above type checking rules. 

It is also able to report multiple type erros while at the same time not report other errors originating from an earlier detected error. This was done by maintaining a vector containing all of the detected type errors during the type checking of the program. Then every time a type error is detected the error is pushed onto the vector and the type checker determines, if possible, the type that would have resulted if the expression was correctly typed. To demonstrate this consider the following code 

```rust
let a: bool = 5 + 5;
let b: bool = a && true;
let c: i32 =  b + 1;
``` 
running this code snippet will generate the following result

```
Error: Mismatched type for variable 'a'
    		Note: expected bool, found i32
Error: Binary operation '+' cannot be applied to type 'bool'
```
By this it can be seen that the type checker assumes that the type of ```a``` is correct when checking the assignment of ```b``` which indeed would be correctly typed if the former variable would have been correctly assigned a boolean value. Thus by following this pattern the type checker is able to continue type checking and reporting new errors, which can be seen when it reports that the assignment of ```c``` is incorrect (addition with boolean is not possible).