# Documentation
<h2> Sections </h2>

- `_VARS: <?>.... EOS!` section:-
  - All variables must be declared here.
  - **?** :- The number of variables declared should be stated.
  - All variables must have a value while declaration.
  - The section must end with `EOS!`
 
    
- `_MAIN: ...... _END:` section:-
  - This is the main block like other languages.
  - All the code execution occurs here.
  - Each statement generally follows the following order :--> ``COMMAND <Variable/Const>+...``.
  - The main section must end with `_END:`.


<h2> Variables </h2>

- The following is the structure for declaring variables :--> ` <MemType> <Name> <Value>`
    - MemType --> `int`  , `float` , `str` or `bool`

- To declare Strings , use ' ' (single-inverted commas)
- **Variables are immutable by default!**
- To make a mutable variable , just add a '?' at the start of its name! :)
- Boolean variables can be `true` or `false`


<h2> ARITHMETICS </h2>

- **Syntax** --> `OPERATION NUM1 NUM2 .....` (any arbitrary number of operands)
- The following operations are currently supported --> `ADD , SUB , MULL , DIV`
- If the expression is standalone , the expression is evaluated and directly printed.
- If not , the expression **Returns the value** which can be used by other functions.

<h2> STANDARD LIBRARY </h2>

- `PRNT` - used to print a value
  - usage --> `PRNT <?>`    **?** -> *Value, Variable, Expression, another function's return value etc.* 
- More functions will be added soon!!


<h2> EXPRESSIONS </h2>

- `Math_expression`:- syntax -> `OPERATION NUM1 NUM2 .....` (*each arithmetic statement is a math_expression*)
- `Conditional_expression`:-
- -  A conditional_expression can be used in **Loops** or can be PRINTED :)
  -  To declare a conditional expression , enclose it in `[..]`
  -  EG :- `PRNT [2 > 4]` -> `false` 
  -  
