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
<h3> Declaration </h3>

- The following is the structure for declaring variables :--> ` <MemType> <Name> <Value>`
  - MemType ==> `int`  , `float` , `str` or `bool`

- To declare Strings , use ' ' (single-inverted commas)
- **Variables are immutable by default!**
- To make a mutable variable , just add a '?' at the start of its name! :)
