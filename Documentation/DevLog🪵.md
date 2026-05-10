# Seaphire Development Log

---

# 📅 26/02/25

> ⚠️ You cannot use MOV [logical / conditional] , use your own brain for that. Might add in teh fututre, but not now.

### 🎯 OBJECTIVE-1 (FIXED)

**Fix issue regarding 'Parsing of conditional and logical expressions'**

```
[[a] && [b]]
```

- **Expected:**  
    `LOGICAL { CONDITIONAL: [a], AND, CONDITIONAL: [b] }`
    
- **Actual:**  
    `LOGICAL { LOGICAL {CONDITIONAL: [a]}, AND , LOGICAL {CONDITIONAL:[b]}}`
    

> ❌ Improper parsing is occurring.  
> Similar issue happens with singular conditions too.

Example:

```
[X]
```

- **Expected:** `Conditional`
    

---

### 📝 Notes

- Where are logical/conditional used in Seaphire?
    
    - If-elif-else
        
    - Jumpif  
        → All can directly parse conditions.
        

---

# 📅 7/8/25

1. (12:27AM) {change}  
    Changing 'PRINT' to 'PRINT' cos I fucking feel like it duh  
    12:29am, deed has been done.
    
2. (12:27AM) {future, game_changing}  
    Update ARITHMETIC such that operations can be performed on same datatypes only !
    
3. (12:27AM) {implementation, solved}  
    Added the <= & >= comparators
    
4. (6:12PM) {bug}  
    Big fucking error when parsing statement like:
	```python
	IF [a="hi"] PRINT "fuck off"
	```

---

# 📅 31/3/26  03:14pm Tue (pi)
c 31/3/26 03:14pm Tue (pi)

I finally have my own desktop now. My HP Victus 15 laptop. Thank you dada <3.  
Lets start working again !

---

# 📅 11/4/26 --:-- Sat

1. Modified the PRINT function (the og function). It prints the given data without adding a newline character in the end.  
    felt the need to add it while creating the "Right Triangle Pyramid" pattern problem.
    
2. Added a function "PRINTLN" which prints the given data and adds a newline character in the end.
    
3. Fixed issues regarding Memory in scopes, especially during iteration
    

---

# 📅 15/4/26 02:15pm Wed

1. Adding While loops and For_loops. Added a new enum in defkeys.rs named Loops with 2 attributes -> WHILE_LOOP, FOR_LOOP.
    

---

# 📅 20/4/26 5:23pm Mon

1. Gave InnerScope its own Struct Defination rather than being part of the enum Builtins. Increases modularity and helps with loops and functions.
    

- TO-DO :  
    Improve the defination of InnerScope so that it works for Function, Loops and other blocks too.  
    Improve how InnerScope (blocks) are executed without causing excess memory overhead.
    

---

# 📅 20/4/26 5:23pm Mon

### To Fix :-

Unable to use IF true format. Issue with EXECUTE(_2) file.  
Fix in the section for if_exp

---

# 📅 26/4/26 11:48am Sun

** Critical! Complete reworking of Memory Management will start now. **
### [[Memory Structure.canvas| Approach]] ➡️
- Storing data contiguously in a vector `Vec<Builtins>`.
- A hashmap stores the name of the variable , and the **pointer to the data**.
	`HashMap<String, usize>`
- Remove / reduce the use of those nasty .clone() shits


---
# 📅 09/5/26 07:05pm Sat
** New Memory Architecture has been implemented successfully ! **
> [!abstract] Refer
> [[How it Works]] for more info


---
# 📅 10/5/26 12:43pm Sun

#### Final Changes to memory architecture
1. What's Added ?
>> [!abstract] Refer
>> [[Changes in Code]] for more info
