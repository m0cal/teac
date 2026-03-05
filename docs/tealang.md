# TeaLang Grammar Specification

## Overview

TeaLang is a statically-typed programming language with syntax inspired by Rust. Each program consists of optional module imports, variable declarations, structure definitions, and function declarations/definitions.

```
program := useStmt* programElement*
programElement := varDeclStmt | structDef | fnDeclStmt | fnDef
```

---

## Basic Elements

### Keywords

TeaLang reserves the following keywords:
- **`let`** - variable declaration
- **`fn`** - function declaration/definition
- **`struct`** - structure definition
- **`if`** / **`else`** - conditional branching
- **`while`** - loop construct
- **`break`** / **`continue`** - loop control
- **`return`** - function return
- **`i32`** - 32-bit integer type
- **`use`** - module import

### Identifiers

Identifiers begin with a letter or underscore, followed by any combination of letters, digits, or underscores. Identifiers cannot be keywords.

```
identifier := [a-zA-Z_][a-zA-Z0-9_]*
```

Examples: `x`, `count`, `my_getint`, `quickread`, `arr_1`

### Numeric Literals

TeaLang supports decimal integer literals. A number is either zero or starts with a non-zero digit.

```
num := 0 | [1-9][0-9]*
```

Examples: `0`, `1`, `42`, `1005`

### Whitespace

Spaces, tabs, newlines, and carriage returns are automatically skipped between tokens.

---

## Module System

### Use Statement

Import external modules (typically the standard library) using the `use` keyword.

```
useStmt := < use > identifier < ; >
```

Example:
```rust
use std;
```

---

## Type System

### Type Specifications

TeaLang supports primitive types and user-defined types.

```
typeSpec := < i32 > | identifier
```

Examples: `i32`, `Node`, `Queue`

### Variable Declarations

Variables can be declared with or without type annotations, and can be scalars, arrays, or slice references.

```
varDecl := identifier < : > < & > < [ > typeSpec < ] >                           // slice type (reference)
         | identifier < : > < [ > typeSpec < ; > num < ] >                        // array with size
         | identifier < : > typeSpec                                              // scalar with type
         | identifier < [ > num < ] >                                             // array without type
         | identifier                                                             // scalar without type
```

Examples:
```rust
n:i32                    // scalar integer
arr: [i32; 100]         // array of 100 integers
input: &[i32]           // slice reference (pointer to array)
que[1005]               // array without explicit type
count                   // scalar without type
```

### Variable Declaration Statements

Declare variables with the `let` keyword, optionally initializing them.

```
varDeclStmt := < let > varDef < ; >
             | < let > varDecl < ; >

varDef := identifier < : > < [ > typeSpec < ; > num < ] > < = > < { > rightValList < } >  // array with initializer
        | identifier < : > typeSpec < = > rightVal                                         // scalar with initializer
        | identifier < [ > num < ] > < = > < { > rightValList < } >                        // array without type, with initializer
        | identifier < = > rightVal                                                        // scalar without type, with initializer
```

Examples:
```rust
let n:i32;                          // declare integer
let x:i32 = 0;                      // declare and initialize
let arr: [i32; 3] = {1, 2, 3};     // declare and initialize array
let count = 0;                      // type inference
```

---

## Structure Definitions

Define custom types using the `struct` keyword with named fields.

```
structDef := < struct > identifier < { > varDeclList < } >
varDeclList := varDecl (< , > varDecl)*
```

Example:
```rust
struct Node {
    value:i32,
    next:i32
}
```

---

## Functions

### Function Declarations

Declare function signatures with optional return types.

```
fnDeclStmt := fnDecl < ; >
fnDecl := < fn > identifier < ( > paramDecl? < ) > < -> > typeSpec  // with return type
        | < fn > identifier < ( > paramDecl? < ) >                   // without return type
paramDecl := varDeclList
```

Examples:
```rust
fn quickread() -> i32;              // declaration with return type
fn move(x:i32, y:i32);             // declaration without return type
fn init();                          // no parameters
```

### Function Definitions

Provide implementation by adding a code block to the declaration.

```
fnDef := fnDecl < { > codeBlockStmt* < } >
```

Example:
```rust
fn add(x:i32, y:i32) -> i32 {
    return x + y;
}

fn main() -> i32 {
    let result:i32 = add(5, 3);
    return result;
}
```

### Function Calls

Functions can be called with module prefixes (for external functions) or locally.

```
fnCall := modulePrefixedCall | localCall
modulePrefixedCall := identifier < :: > identifier < ( > rightValList? < ) >
localCall := identifier < ( > rightValList? < ) >
```

Examples:
```rust
std::getint()               // standard library function
quickread()                 // local function
addedge(x, y)              // local function with arguments
std::putch(10)             // standard library with argument
```

---

## Statements

### Code Block Statements

Statements that can appear within function bodies:

```
codeBlockStmt := varDeclStmt
               | assignmentStmt
               | callStmt
               | ifStmt
               | whileStmt
               | returnStmt
               | continueStmt
               | breakStmt
               | nullStmt
```

### Assignment Statement

Assign values to variables, array elements, or structure fields.

```
assignmentStmt := leftVal < = > rightVal < ; >
leftVal := identifier leftValSuffix*
leftValSuffix := < [ > indexExpr < ] >                                 // array indexing
               | < . > identifier                                       // member access
indexExpr := num | identifier
```

Examples:
```rust
x = 5;                      // simple assignment
arr[i] = 10;               // array element assignment
node.value = x;            // struct field assignment
tail[i].next = head;       // chained access
```

### Call Statement

Execute a function and discard its return value.

```
callStmt := fnCall < ; >
```

Examples:
```rust
init();
std::putch(10);
addedge(x, y);
```

### Return Statement

Exit a function with or without a return value.

```
returnStmt := < return > rightVal < ; >
            | < return > < ; >
```

Examples:
```rust
return 0;
return x + y;
return;                     // void return
```

### If Statement

Conditional branching with optional else clause.

```
ifStmt := < if > boolExpr < { > codeBlockStmt* < } > < else > < { > codeBlockStmt* < } >
        | < if > boolExpr < { > codeBlockStmt* < } >
```

Example:
```rust
if x > 0 {
    return x;
}

if ch == 45 {
    f = 1;
} else {
    f = 0;
}
```

### While Statement

Loop with a boolean condition.

```
whileStmt := < while > boolExpr < { > codeBlockStmt* < } >
```

Example:
```rust
while i < n {
    i = i + 1;
}

while (ch >= 48) && (ch <= 57) {
    x = x * 10 + ch - 48;
    ch = std::getch();
}
```

### Break Statement

Exit from the innermost loop.

```
breakStmt := < break > < ; >
```

Example:
```rust
while 1 > 0 {
    if done {
        break;
    }
}
```

### Continue Statement

Skip to the next iteration of the loop.

```
continueStmt := < continue > < ; >
```

Example:
```rust
while i < n {
    if inq[temp] == 0 {
        continue;
    }
    i = i + 1;
}
```

### Null Statement

An empty statement (just a semicolon).

```
nullStmt := < ; >
```

---

## Expressions

### Right Values

Values that can appear on the right side of assignments.

```
rightVal := boolExpr | arithExpr
rightValList := rightVal (< , > rightVal)*
```

### Arithmetic Expressions

Arithmetic expressions support addition, subtraction, multiplication, and division with standard precedence.

```
arithExpr := arithTerm (arithAddOp arithTerm)*
arithTerm := exprUnit (arithMulOp exprUnit)*
arithAddOp := < + > | < - >
arithMulOp := < * > | < / >
```

Examples:
```rust
x + 1
n - 1
x * 10 + ch - 48
num / base
```

### Expression Units

Primary expressions that form the building blocks of larger expressions.

```
exprUnit := < ( > arithExpr < ) >
          | fnCall
          | < - > num                                       // negative literal
          | num
          | identifier exprSuffix*
exprSuffix := < [ > indexExpr < ] >                        // array indexing
            | < . > identifier                              // member access
```

Examples:
```rust
42
x
arr[i]
node.value
std::getint()
0-x                     // negative number
(a + b) * c            // parenthesized expression
list[cnt].next         // chained access
```

### Boolean Expressions

Boolean expressions support logical AND, OR, NOT, and comparison operators.

```
boolExpr := boolAndTerm (< || > boolAndTerm)*
boolAndTerm := boolUnitAtom (< && > boolUnitAtom)*
boolUnitAtom := boolUnitParen
              | boolComparison
              | < ! > boolUnitAtom
boolUnitParen := < ( > boolExpr < ) >
               | < ( > exprUnit compOp exprUnit < ) >
boolComparison := exprUnit compOp exprUnit
compOp := < <= > | < >= > | < == > | < != > | < < > | < > >
```

Examples:
```rust
x > 0
x == 1
i != -1
(x >= 48) && (x <= 57)
(ch < 48) || (ch > 57)
!done
```

---

## Operators

### Arithmetic Operators

| Operator | Description       | Example |
|----------|-------------------|---------|
| `+`      | Addition          | `x + 1` |
| `-`      | Subtraction       | `n - 1` |
| `*`      | Multiplication    | `x * 10`|
| `/`      | Division          | `n / 2` |

### Comparison Operators

| Operator | Description              | Example  |
|----------|--------------------------|----------|
| `==`     | Equal to                 | `x == 1` |
| `!=`     | Not equal to             | `i != -1`|
| `<`      | Less than                | `i < n`  |
| `>`      | Greater than             | `a > max`|
| `<=`     | Less than or equal to    | `ch <= 57`|
| `>=`     | Greater than or equal to | `ch >= 48`|

### Logical Operators

| Operator | Description    | Example              |
|----------|----------------|----------------------|
| `&&`     | Logical AND    | `(x >= 0) && (x < 10)`|
| `\|\|`     | Logical OR     | `(ch < 48) \|\| (ch > 57)`|
| `!`      | Logical NOT    | `!done`              |

### Other Operators

| Operator | Description          | Example           |
|----------|----------------------|-------------------|
| `=`      | Assignment           | `x = 5;`          |
| `->`     | Function return type | `fn main() -> i32`|
| `::`     | Module separator     | `std::getint()`   |
| `&`      | Reference            | `&[i32]`          |

---

## Complete Example

```rust
use std;

struct Node {
    value:i32,
    next:i32
}

let head:i32;
let nodes: [Node; 100];
let count:i32 = 0;

fn init() {
    head = 0-1;
    count = 0;
}

fn add_node(val:i32) {
    nodes[count].value = val;
    nodes[count].next = head;
    head = count;
    count = count + 1;
}

fn main() -> i32 {
    init();
    
    let n:i32 = std::getint();
    let i:i32 = 0;
    
    while i < n {
        let val:i32 = std::getint();
        add_node(val);
        i = i + 1;
    }
    
    return 0;
}
```

---

## Notes

1. **Type Annotations**: Type annotations are optional in many contexts but recommended for clarity.
2. **Array Syntax**: Arrays use Rust-style syntax: `[type; size]`.
3. **Slice References**: Use `&[type]` to pass arrays to functions by reference.
4. **Module System**: Currently only supports importing the `std` module.
5. **No Implicit Conversions**: All type conversions must be explicit.
6. **Operator Precedence**: Standard mathematical precedence applies (multiplication/division before addition/subtraction).
7. **Chained Access**: Array indexing and member access can be chained: `arr[i].field[j]`.
