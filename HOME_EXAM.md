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
- Give an example that showcases all rules of your EBNF. The program should "do" something as used in the next exercise.

```rust
fn main() -> i32 {
	return test(1, true); // Should return 12
}

fn test(mut a: i32, b: bool) -> i32 {
	a = a + 1;
	let c: bool = b && true;

	if (c) {
		let a: i32 = 100;
	}

	if (a == 100) {
		a = 0;
	} else {
		a = a + 1;
	}

	a = prec(a);

	if (a <= 2) {
		a = -1;
	} else if (a == 0) {
		a = -2;
	} else {
		a = a + 1;
	}

	return a;
}

fn prec(a: i32) -> i32 {
	return 2 + a * 3;
}
```

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

The parser was implemented using the [LALRPOP](https://github.com/lalrpop/lalrpop) parser generator witch uses LR(1) by default. By following the LALRPOP tutorial, to learn the syntax and some of the functionality, an expression parser was implemented. Thus I didn't implemented the expression parser myself but I built upon it and extended it to implement the rest of the parser for my language. 

When it comes precedence of the expression parser the tutorial already had it implemented for numerical expressions. When I extended the parser to support logical and relational operations the precedence of those operations will not always work, so for the most part in my language sub expressions have to be parenthesized if they contain those operations.

No error messages, except the already existing error messages from LALRPOP, have been implemented in the parser. And thus no error recovery is possible since the parser just forwards the LALRPOP error message and stops if an error occurs while parsing the program.

# Your semantics
- Give an as complete as possible Structural Operetional Semantics (SOS) for your language
 
### Operators
Given a expression $`e`$ in the state $`\sigma`$  that evaluates to a value $`n`$ or boolean $`b`$ the following can be said for the operators in the language

#### Arithmetic
Addition:
```math
\frac{\lang e1, \sigma \rang \ \Downarrow \ n1 \; \lang e2, \sigma \rang \ \Downarrow \ n2}{\lang e1 + e2, \sigma \rang \ \Downarrow \ n1 + n2}
```

Subtraction:
```math
\frac{\lang e1, \sigma \rang \ \Downarrow \ n1 \; \lang e2, \sigma \rang \ \Downarrow \ n2}{\lang e1 - e2, \sigma \rang \ \Downarrow \ n1 - n2}
```

Multiplication:
```math
\frac{\lang e1, \sigma \rang \ \Downarrow \ n1 \; \lang e2, \sigma \rang \ \Downarrow \ n2}{\lang e1 * e2, \sigma \rang \ \Downarrow \ n1 * n2}
```

Division:
```math
\frac{\lang e1, \sigma \rang \ \Downarrow \ n1 \; \lang e2, \sigma \rang \ \Downarrow \ n2}{\lang e1 / e2, \sigma \rang \ \Downarrow \ n1 / n2}
```

Unary:
```math
\frac{\lang e1, \sigma \rang \ \Downarrow \ n1}{\lang -e1, \sigma \rang \ \Downarrow \ -n1}
```

#### Boolean
AND:
```math
\frac{\lang e1, \sigma \rang \Downarrow \ b1 \; \lang e2, \sigma \rang \Downarrow \ b1}{\lang e1 \ \&\& \ e2, \sigma \rang \ \Downarrow \ b1 \ \text{AND} \ b2}
```

OR:
```math
\frac{\lang e1, \sigma \rang \Downarrow \ b1 \; \lang e2, \sigma \rang \Downarrow \ b2}{\lang e1 \ || \ e2, \sigma \rang \ \Downarrow \ b1 \ \text{OR} \ b2}
```

Less than (<):
```math
\frac{\lang e1, \sigma \rang \Downarrow \ n1 \; \lang e2, \sigma \rang \Downarrow \ n2}{\lang e1 \ < \ e2, \sigma \rang \ \Downarrow \ n1 \ < \ n2}
```

Greaten than (>):
```math
\frac{\lang e1, \sigma \rang \Downarrow \ n1 \; \lang e2, \sigma \rang \Downarrow \ n2}{\lang e1 \ > \ e2, \sigma \rang \ \Downarrow \ n1 \ > \ n2}
```

Less than or equals (<=):
```math
\frac{\lang e1, \sigma \rang \Downarrow \ n1 \; \lang e2, \sigma \rang \Downarrow \ n2}{\lang e1 \ <= \ e2, \sigma \rang \ \Downarrow \ n1 \ <= \ n2}
```

Greater than or equals (>=):
```math
\frac{\lang e1, \sigma \rang \Downarrow \ n1 \; \lang e2, \sigma \rang \Downarrow \ n2}{\lang e1 \ >= \ e2, \sigma \rang \ \Downarrow \ n1 \ >= \ n2}
```

Equals (==):
```math
\frac{\lang e1, \sigma \rang \Downarrow \ n1 \; \lang e2, \sigma \rang \Downarrow \ n2}{\lang e1 \ == \ e2, \sigma \rang \ \Downarrow \ n1 \ == \ n2}
```

Not equals (!=):
```math
\frac{\lang e1, \sigma \rang \Downarrow \ n1 \; \lang e2, \sigma \rang \Downarrow \ n2}{\lang e1 \ != \ e2, \sigma \rang \ \Downarrow \ n1 \ != \ n2}
```

#### Commands

**if** statement:

```math
\frac{\lang b, \sigma \rang \ \Downarrow \ \text{true} \quad \lang c1, \sigma \rang \ \Downarrow \ \sigma'}{\lang \text{if } b \text{ then } c1 \text{ else } c2, \sigma \rang \ \Downarrow \ \sigma'}
```

```math
\frac{\lang b, \sigma \rang \ \Downarrow \ \text{false} \quad \lang c2, \sigma \rang \ \Downarrow \ \sigma''}{\lang \text{if } b \text{ then } c1 \text{ else } c2, \sigma \rang \ \Downarrow \ \sigma''}
```

Not sure how to do this for elseif

**while** statement:
```math
\frac{\lang b, \sigma \rang \ \Downarrow \ \text{false} \quad \lang c, \sigma \rang \Downarrow \sigma' }{\text{while } b \text{ do } c, \sigma \rang \ \Downarrow \ \sigma}
```

```math
\frac{\lang b, \sigma \rang \ \Downarrow \ \text{true} \quad \lang c, \sigma \rang \ \Downarrow \ \sigma' \quad \lang \text{while } b \text{ do } c,\sigma' \rang \ \Downarrow \ \sigma''}{\lang \text{while } b \text{ do } c, \sigma \rang \ \Downarrow \ \sigma''}
```

**let** statement:\
```math
\frac{}{\lang \text{let }x := n, \sigma \rang \ \Downarrow \ \sigma [x := n]}
```

**return** statement:
```math
\frac{\lang e, \sigma \rang \Downarrow n}{\lang \text{return } e, \sigma \rang \Downarrow n }
```

assignment:
```math
\frac{}{\lang x := n, \sigma \rang \ \Downarrow \ \sigma [x := n]}
```

functions:
Given a list of arguments $`\overrightarrow{v} = [v_1, \ldots, v_n]`$ the call of function $`p`$ will evaulate to a value $`n`$ (the return value, if any) according to

```math
\frac{\lang p, \sigma \rang}{\lang p(\overrightarrow{v}), \sigma \rang \ \Darr \ \lang n, \sigma \rang}
```

- Explain (in text) what an interpretation of your example should produce, do that by dry running your given example step by step. Relate back to the SOS rules. You may skip repetions to avoid cluttering.

The example program will be started by calling the main function. The main function contanins a return statement which according to the SoS will be evaluated and returned as the result of calling the main function. Since the expression of the return statement is a function call the function will be called with the arguments supplied according to the SoS. In this case the arguments are 1 and true.

The function `test` will then be called and since it takes a mutable i32 `a` and boolean `b` the functions context will start of with containing a scope consisting of these variables with the values of the arguments. Then `a` is assigned a new value according to the SoS which will be evaluated to `a = 2`. A new variable `c` is then declared with a let statement which according to the SoS will be assigned the value of the expression `b && true`. 

Then an if statement will be evaluated by computing the condtion `c` which evaluates to true. Then, according to the SoS, the then statement will be executed which contains a new variable declaration of the same name as the earlier declared variable `a`. But since the then statement is executed inside another scope the variable will not replace the value of `a` outside this scope.

Another if statement is then evaluated but since the value in a did not get changed in this scope the else statement is executed according to the SoS which updates variable `a` to `a = 3`. Then `a` is once again assigned a new value which this time is a function call. After this `a = 11`. And another if statement is evaluated but since the conditions of if and else if won't be true the else statement will be executed. Now `a = 12` and a is returned.

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

I implemented the interpreter myself and the interpreter executes programs according to the SOS defined above. Additionally it can also handle else-if statements, scoping and contexts. In my implementation a context is simply a vector of scopes which allows for shadowing etc. 

The interpreter also panics when encountering a evaluation error. For example, if the program contains 
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

### Expressions
#### Arithmetic
Addition:
```math
\frac{\Gamma \ \vdash \ e1 \ : \ i32 \quad \Gamma \ \vdash \ e2 \ : \ i32}{\Gamma \ \vdash (e1 + e2) \ : \ i32}
```

Subtraction:
```math
\frac{\Gamma \ \vdash \ e1 \ : \ i32 \quad \Gamma \ \vdash \ e2 \ : \ i32}{\Gamma \ \vdash (e1 - e2) \ : \ i32}
```

Division:
```math
\frac{\Gamma \ \vdash \ e1 \ : \ i32 \quad \Gamma \ \vdash \ e2 \ : \ i32}{\Gamma \ \vdash (e1 / e2) \ : \ i32}
```

Multiplication:
```math
\frac{\Gamma \ \vdash \ e1 \ : \ i32 \quad \Gamma \ \vdash \ e2 \ : \ i32}{\Gamma \ \vdash (e1 * e2) \ : \ i32}
```

Unary:
```math
\frac{\Gamma \ \vdash \ e1 \ : \ i32}{\Gamma \ \vdash (-e1) \ : \ i32}
```

#### Boolean
AND:
```math
\frac{\Gamma \ \vdash \ e1 \ : \ bool \quad \Gamma \ \vdash \ e2 \ : \ bool}{\Gamma \ \vdash (e1 \ \&\& \ e2) \ : \ bool}
```

OR:
```math
\frac{\Gamma \ \vdash \ e1 \ : \ bool \quad \Gamma \ \vdash \ e2 \ : \ bool}{\Gamma \ \vdash (e1 \ || \ e2) \ : \ bool}
```

Less than (<):
```math
\frac{\Gamma \ \vdash \ e1 \ : \ i32 \quad \Gamma \ \vdash \ e2 \ : \ i32}{\Gamma \ \vdash (e1 \ < \ e2) \ : \ bool}
```

Greaten than (>):
```math
\frac{\Gamma \ \vdash \ e1 \ : \ i32 \quad \Gamma \ \vdash \ e2 \ : \ i32}{\Gamma \ \vdash (e1 \ > \ e2) \ : \ bool}
```

Less than or equals (<=):
```math
\frac{\Gamma \ \vdash \ e1 \ : \ i32 \quad \Gamma \ \vdash \ e2 \ : \ i32}{\Gamma \ \vdash (e1 \ <= \ e2) \ : \ bool}
```

Greater than or equals (>=):
```math
\frac{\Gamma \ \vdash \ e1 \ : \ i32 \quad \Gamma \ \vdash \ e2 \ : \ i32}{\Gamma \ \vdash (e1 \ >= \ e2) \ : \ bool}
```

Equals (==):
```math
\frac{\Gamma \ \vdash \ e1 \ : \ bool \quad \Gamma \ \vdash \ e2 \ : \ bool}{\Gamma \ \vdash (e1 \ == \ e2) \ : \ bool}
```
or
```math
\frac{\Gamma \ \vdash \ e1 \ : \ i32 \quad \Gamma \ \vdash \ e2 \ : \ i32}{\Gamma \ \vdash (e1 \ == \ e2) \ : \ bool}
```

Not equals (!=):
```math
\frac{\Gamma \ \vdash \ e1 \ : \ bool \quad \Gamma \ \vdash \ e2 \ : \ bool}{\Gamma \ \vdash (e1 \ != \ e2) \ : \ bool}
```
or
```math
\frac{\Gamma \ \vdash \ e1 \ : \ i32 \quad \Gamma \ \vdash \ e2 \ : \ i32}{\Gamma \ \vdash (e1 \ != \ e2) \ : \ bool}
```

### Assignment
Given the type $`\tau`$, variable $`x`$ and value $`n`$

```math
\frac{\Gamma \ \vdash \ x \ : \ \tau \quad \Gamma \ \vdash \ n \ : \ \tau}{\lang x := n, \sigma \rang \ \Darr \ \Gamma \ \vdash x \ : \ \tau}
```

### **let** assignment
```math
\frac{\Gamma \ \vdash \ n \ : \ \tau}{\lang \text{let} \  x \ : \ \tau \ := \ n, \sigma \rang \ \Darr \ \Gamma \ \vdash x \ : \ \tau}
```

### **while** statement
The condition, $`b`$, of the while statement  

$`
\frac{}{\Gamma \ \vdash b \ : \ bool}
`$

### **if/elseif** statement
The conditions, $`b_i`$, of the if- and elseif-statements

$`
\frac{}{\Gamma \ \vdash b_i \ : \ bool}
`$

### Functions
Given the function $`p`$ with the type of its return type

$`
\frac{\Gamma \ \vdash p \ : \ \tau \quad \lang p, \sigma \rang \ \Darr \ n}{\Gamma \ \vdash n \ : \ \tau}
`$

#### Parameters and arguments
Given a function with parameters $`p1, \dotsb, p_i`$ and arguments $`a1, \dotsb, a_i`$ then for every parameter and argument

$`
\frac{\Gamma \ \vdash p_i \ : \ \tau}{\Gamma \ \vdash a_i \ : \ \tau}
`$

- Demonstrate each "type rule" by an example

### Arithmetics
The following shows the type rules for arithmetic operations, the only difference between the type rules is the operator and thus it's not repeated for every arithmetic operation.
```rust
5 + 5	// ok
true + 5	// error
true + false	// error
```

### Boolean
Examples of type rules for boolean operations
```rust
true && false	// ok
true && 4	// error

true || false 	// ok
5 || false	// error

4 < 3 // ok
4 < true	// error
true < false	// error

4 > 3 	// ok
4 > true 	// error
true > false 	// error

4 <= 3 	// ok
4 <= true 	// error
true <= false 	// error

4 >= 3	// ok
4 >= true	// error
true >= false 	// error

4 == 3 // ok
false == true	// ok
5 == false 	// error

4 != 3 // ok
false != true	// ok
5 != false 	// error
```

In the following examples any integers and booleans could be replaced by arbitrary expression which evaluates to the same type and the result would be the same.
### Assignments
If variable `x` and `y` has been defined as a `i32`
```rust
x = 10 // ok
x = y // ok
x = true  // error
```

### **let** assignment
```rust
let x: i32 = 10; 	// ok
let x: i32 = true; 	// error
let y: bool = true; // ok
let y: bool = 5; 	// error
```

### **while** statement
```rust
while(true) {	// ok
	...
}

while(5) {	// error
	...
}
```

### **if/elseif** statements
```rust
if (true) {	// ok
	...
}

if (5) {	// error
	...
}

if (false) {
	...
} else if (true) {	// ok
	...
}

if (false) {
	...
} else if (123) {	// error
	...
}
```

### Functions
```rust
fn int_func() -> i32 {	// ok
	return 4;
}

fn int_func() -> i32 {	// error
	return true;
}

fn bool_func() -> bool {	// ok
	return false;
}

fn bool_func() -> bool {	// error
	return 5;
}
```
#### Parameters and arguments
```rust
fn param_func(a: i32, b: bool) {
	...
}

param_func(5, true);	// ok
param_func(true, true);	// error
param_func(5, 5);	// error
param_func(true, 4);	// error
param_func(4);	// error
```

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

The type checker was fully implemented by me and it rejects ill-typed programs according to the above type checking rules. 

It is also able to report multiple type errors while at the same time not report other errors originating from an earlier detected error. This was done by maintaining a vector containing all the detected type errors during the type checking of the program. Then every time a type error is detected the error is pushed onto the vector and the type checker determines, if possible, the type that would have resulted if the expression was correctly typed. To demonstrate this, consider the following code 

```rust
let a: i32 = 5 + 5 * true;
let b: i32 = a + 10;
let c: bool =  b == true;
``` 
running this code snippet will generate the following result

```
Error: Binary operation '*' cannot be applied to type 'bool'
Error: Mismatched type for operation '==' 
                Note: expected i32, found bool
Could not compile 'input.rs'
```
By this it can be seen that the type checker assumes that the type of `a` is correct when checking the assignment of `b` which indeed would be correctly typed if the former variable would have been correctly assigned a i32 value. This is done by concluding that `5 * true` would evaluate to a i32 value if it was correctly typed and then the entire expression `5 + 5 * true` will be correctly typed.

Thus by following this pattern the type checker is able to continue type checking and reporting new errors, which can be seen when it reports that the assignment of `c` is incorrect (boolean equals requires the same type on each side).

The type checker also makes sure that functions with a specified return type always returns. This is done by going through each function and making sure that a tailing return statement or a return inside an else-statement exists. If this is not the case the error is reported along with all other type errors found.

# Your borrow checker
- Give a specification for well versus ill formed borrows. (What are the rules the borrow checker should check).

The borrow checker needs to make sure that a borrow lasts for a scope no greater than the owners scope by keeping track of lifetimes which describe the scope that a reference is valid for.

It also needs to keep track of the kind of borrow that have been made to a resource, since different kinds of borrows of the same resource cannot be made at the same time. The two kinds of borrows are references (&T) and mutable references (&mut T). If a reference has been made one or more references are allowed, but if it is a mutable reference exactly one is allowed in order to assure that no data race occurs.

**I did not implement a borrow checker for my language so the next rest of the questions regarding the borrow checker could not be answered.**

# Your LLVM backend
- Let your backend produces LLVM-IR for your example program.

```llvm
ModuleID = 'program'
source_filename = "program"
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"

define i32 @main() {
entry:
  %tmp = call i32 @test(i32 1, i32 1)
  ret i32 %tmp
}

define i32 @test(i32 %a, i32 %b) {
entry:
  %c = alloca i32
  %b2 = alloca i32
  %a1 = alloca i32
  store i32 %a, i32* %a1
  store i32 %b, i32* %b2
  %a3 = load i32, i32* %a1
  %add = add i32 %a3, 1
  store i32 %add, i32* %a1
  %b4 = load i32, i32* %b2
  %and = and i32 %b4, 1
  store i32 %and, i32* %c
  %c5 = load i32, i32* %c
  br i32 %c5, label %then, label %ifcont

then:                                             ; preds = %entry
  %a6 = alloca i32
  store i32 100, i32* %a6
  br label %ifcont

ifcont:                                           ; preds = %then, %entry
  %iftmp = phi i32 [ 2, %then ], [ 2, %ifcont ]
  %a7 = load i32, i32* %a1
  %eq = icmp eq i32 %a7, 100
  br i1 %eq, label %then8, label %else

then8:                                            ; preds = %ifcont
  store i32 0, i32* %a1
  br label %ifcont9

else:                                             ; preds = %ifcont
  %a10 = load i32, i32* %a1
  %add11 = add i32 %a10, 1
  store i32 %add11, i32* %a1
  br label %ifcont9

ifcont9:                                          ; preds = %else, %then8
  %iftmp12 = phi i32 [ 2, %then8 ], [ 2, %else ]
  %a13 = load i32, i32* %a1
  %tmp = call i32 @prec(i32 %a13)
  store i32 %tmp, i32* %a1
  %a14 = load i32, i32* %a1
  %leq = icmp sle i32 %a14, 2
  br i1 %leq, label %then15, label %else16

then15:                                           ; preds = %ifcont9
  store i32 -1, i32* %a1
  br label %ifcont17

else16:                                           ; preds = %ifcont9
  %a18 = load i32, i32* %a1
  %eq19 = icmp eq i32 %a18, 0
  br i1 %eq19, label %then20, label %else21

ifcont17:                                         ; preds = %ifcont22, %then15
  %iftmp26 = phi i32 [ 2, %then15 ], [ 2, %else16 ]
  %a27 = load i32, i32* %a1
  ret i32 %a27

then20:                                           ; preds = %else16
  store i32 -2, i32* %a1
  br label %ifcont22

else21:                                           ; preds = %else16
  %a23 = load i32, i32* %a1
  %add24 = add i32 %a23, 1
  store i32 %add24, i32* %a1
  br label %ifcont22

ifcont22:                                         ; preds = %else21, %then20
  %iftmp25 = phi i32 [ 2, %then20 ], [ 2, %else21 ]
  br label %ifcont17
}

define i32 @prec(i32 %a) {
entry:
  %a1 = alloca i32
  store i32 %a, i32* %a1
  %a2 = load i32, i32* %a1
  %mul = mul i32 %a2, 3
  %add = add i32 2, %mul
  ret i32 %add
}
```

- Describe where and why you introduced allocations and phi nodes.

Phi nodes were introduced in order to assign the correct value to a variables dependent on what control flow the execution took. This was used while compiling if-, elseif-, else- and while statements. Since the same variable could be used inside the different blocks of these satements and the resulting value of the variable should depend on what block the execution came from when entering the phi node.

Allocations were used whenever new variables were defined using let statements and when a function with parameters were defined. This is because all the local variables has to be allocated on the stack in order to use and manipulate them later in the code. The allocation also returns the pointer to the variables memory on the stack so its value can be updated using that pointer.

- If you have added optimization passes and/or attributed your code for better optimization (using e.g., noalias).

I did not add any optimization.

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

When implementing the LLVM backend I started of by implementing the `crust.rs` example provided by you. From this I adapted it a bit to work with my AST in order to compile expressions. Then I started added functionality and during this I took a lot of inspiration from the inkwell [kaleidoscope](https://github.com/TheDan64/inkwell/tree/master/examples/kaleidoscope) example. But once again it had to be adapted to my AST and also add some other functionality such as while loops and else-if statements. The example only compiles one function so it also had to be reworked in order to compile entire programs containing multiple functions.

The LLVM backend can generate and compile code from my language but no optimization have been implemented. Also, since my language does not have a borrow checker aliasing is possible so noalias could not be passed anywhere.

# Overal course goals and learning outcomes
In this course I have learned a lot when it comes to programming since I have implemented a quite large project myself. Comparing this to other courses I enjoyed the heavy practical work required to implement the compiler. I also learned a lot of Rust since I have no prior experiance of the language. Although it should be noted that the learning curve in the beginning of the course was quite big since I had to learn Rust.

Regarding the course goals and what I have done:

- Parser:
  - A lot of work with creating a suitable AST
  - Maybe not so much learnt about lexical and syntax analysis since I used LALRPOP which basically did this for me without having to "know" a lot.
  - EBNF
- Interpreter
  - Had to come up with a way of handling variables in order to allow for shadowing etc.
- Type checker
  - Worker a lot with figuring out a way to report multiple errors
  - How to display errors in a suitable and easy way
- LLVM IR and JIT compilation

In general it feels like I learned a lot more about the practical parts of a compiler than the theoretical part. Mostly because I spent the most of the course on implementing the compiler and not really thinking or creating the theoretical analysis like SoS and EBNF.
