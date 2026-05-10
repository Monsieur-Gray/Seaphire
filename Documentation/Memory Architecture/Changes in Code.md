1. New file for memory related structures : <i>mem_layout.rs</i>
2. Updated Parser.rs
3. Updated fetch_data.rs

# <span style="color:rgb(146, 208, 80);  ">Structures</span> 
### **<u>MemCell</u>** :- 
A unit block of memory that contains the attributes of a variable
```rust
struct MemCell {
	pub data: Builtins,
	pub scope: Scope,
	pub is_mutable: bool
}
```
	
- **data** :- Holds the data
 - **scope** :- Stores the scope (`Scope::GlobalScope or Scope::Local(u32)`)
 - **is_mutable** := Self - explanatory

<hr style="border: none; border-top: 2px dashed #666; margin: 20px 0">

### <u >Memory</u> :-
*THE MEMORY*. this is where the data is stored sequentially in a vector and can be accessed via its <i>Pointer</i>
```rust
struct Memory {
	pub cells: Vec< Option<MemCell> >
	pub freelist: Vec<usize>
}
```

- **cells** :- Stores the data of the type MemCell.
- **freelist** :- Stores the pointers of the variables that have been recently freed. Saves memory by reusing freed cells.
 #### <span style="color:rgb(0, 176, 240);"><u>Implementations / Methods</u></span>:-
 1.  **Memory :: new()**
```rust
pub fn new() -> Self {
	Self {
		cells: Vec::new(),
        freelist: Vec::new()
    }
}
```
Creates a new Memory object.
<hr style="border: none ; border-top: 1px solid rgb(150, 140, 140); margin: 5px 0">

2. ** Memory :: alloc(...)**
```rust
pub fn alloc(&mut self, cell: MemCell) -> usize {
	if self.freelist.is_empty() {
		self.cells.push(Some(cell));
		return self.cells.len() - 1;
    }
	else {
		let ptr = self.freelist.remove(0);
		self.cells[ptr] = Some(cell);
        return ptr;
    }
}
```

Parameters :
		- &mut self => The object must be a mutable reference.
		- cell: MemCell => Data to be allocated.

Output : Pointer to the allocated data (usize)

Working:
- First `freelist` is checked whether it has any empty slot or not. If true then the data is allocated at that slot and the pointer (index) is returned.
- If `freelist` does not have an empty slot, then the data is pushed at the end of the `self.cells` and the pointer (index) is returned.

<hr style="border:none ; border-top: 1px solid rgb(150, 140, 140); margin: 5px 0">

3. **Memory :: get(...)**
```rust
pub fn get(&self, ptr: usize) -> &MemCell {
	if let Some(cell) = self.cells.get(ptr) {
		return cell.as_ref().unwrap();
    }
    
	else {
		Throw!(format!("Memory::get_error :> Invalid Pointer. Variable not found at index {}.", ptr));
    }
}
```

Parameters :
- &self => The object must be reference.
- ptr: usize => `pointer` to the data.

Output : Reference to the Memory Cell at the given pointer of type `&MemCell`

Working:
- If the data exists at the given pointer i.e. cell is not None / empty then return a reference to the MemCell => `&MemCell.
- Else throw Invalid Pointer Error.
<hr style="border:none ; border-top: 1px solid rgb(150, 140, 140); margin: 5px 0">

4. Memory :: get_mut(...)

```rust
pub fn get_mut(&mut self, ptr: usize) -> &mut MemCell {
	if let Some(cell) = self.cells.get_mut(ptr) {
		return cell.as_mut().unwrap();
	}
	else {
		Throw!(format!("Memory::get_error >> Variable not found at index {}", ptr));
	}    
}
```
Parameters :
- &mut self => The object must be a mutable reference.
- ptr: usize => pointer to the data.

Output : Mutable - reference to the Memory Cell at the given pointer of type `&mut MemCell`

Working:
- If the data exists at the given pointer i.e. cell is not None / empty then return a mutable reference to the MemCell => `&mut MemCell`.
- Else throw Invalid Pointer Error.

<hr style="border:none ; border-top: 1px solid rgb(150, 140, 140); margin: 5px 0">

5.  **Memory :: dealloc(...)**
```rust
pub fn dealloc(&mut self, ptr: usize) {
	self.cells[ptr] = None;
	self.freelist.push(ptr);
}
```
Parameters :
- &mut self => The object must be a mutable reference.
- ptr: usize => pointer to the data.

Output : No return type. Sets the cell at the given pointer as None, and pushes the pointer into freelist to be reused later.

Working:
- Set the cell at given pointer position to `None`.
- Pushes the freed pointer in `freelist` for future use.




