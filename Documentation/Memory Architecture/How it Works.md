
## <span style="color:rgb(146, 208, 80)">How it works❔</span>

^cac33e

>[!abstract] For Graph View
>  See [[Memory Structure.canvas | Implementation]] for more info

#### Allocation
1. Variables are arranged in the `VarDecl {name, value, scope, isMut}` in PARSER.rs -> returns `collect_vars: Vec<VarDecl>`.
2. Iterate over vec collect_vars.
3. Add the variables in the memory --> `Memory::alloc(MemCell)` which returns index of the data in the memory vector.
4. Store the variable name, and pointer in a `Hashmap<name, ptr>` in the **`Env`**

---

#### Accessing
1. Get the variable name.
2. Lookup the variable name in `Env` --> returns the ptr, or the index of the cell.
3.  Go to the cell at the given index in `Memory`.
4. Do necessary operation.

---
#### Deallocation
1. Get the variable name.
2. Lookup the variable name in `Env` --> returns the ptr, or the index of the cell.
3.  Go to the cell at the given index in Memory.
4. Free the Cell. 
5. Remove the variable name from the `Env`.

```booknav
[[MOC | Home Page 🏠]]
[[Changes in Code| next]]
```
