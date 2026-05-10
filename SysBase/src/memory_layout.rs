use crate::{Throw, defkeys::*};
use std::collections::HashMap;

/*  7th of May , 2026 AD (08:40 PM)
 ~~~~~~~~~~~~~~~~~ CORE MEMORY IS DEFINED HERE ~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/* 
? Final Memory Layout :- 
     The data of the variable is stored a Vector sequentially. 
      To access the data we need the index of that cell.

    # Another structure `ENV` stores vector of Hashmap<var_name: String, pointer: usize>. 
      Using Vector of hashmaps indirectly implements scope. The vector is actually the `STACK`. (more about it later)

    # The pointer is the index of the cell in the vector. 

? Process
    -- Allocation
        #1) Variables are arranged in the VarDecl {name, data, scope, isMut} in PARSER.rs -> returns collect_vars: Vec<VarDecl>
        #2) Iterate over vec collect_vars
        #3) Add the variables in the memory --> Memory::alloc(MemCell) which returns index of the data in the memory vector.
        #4) Store the variable name, and pointer in a Hashmap<name, ptr> in the Env

    -- Accessing
        #1) Get the variable name.
        #2) Lookup the variable name in Env --> returns the ptr, or the index of the cell.
        #3) Go to the cell at the given index in Memory.
        #4) Do necessary operation.

    -- Deallocation
        #1) Get the variable name.
        #2) Lookup the variable name in Env --> returns the ptr, or the index of the cell.
        #3) Go to the cell at the given index in Memory.
        #4) Free the Cell.
        #5) Remove the variable name from the Env.
*/


// Defination of the data. Scope attribute will be depricated in the future
pub struct MemCell {
    pub data: Builtins, 
    pub scope: Scope,
    pub is_mutable: bool
}

// this is the main memory, where all the data lives.
// ? Added freelist to contain the index of free places that can be used. O(1) time complexity as before
// ? Solves the issue of deallocation
pub struct Memory {
    pub cells: Vec<Option<MemCell>>,
    pub freelist: Vec<usize>
}

// Implicitly handles Scopes by implementing STACK
pub struct Env {
    pub scopes: Vec<HashMap<String, usize>>
}



impl Memory {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            freelist: Vec::new()
        }
    }

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

    pub fn get(&self, ptr: usize) -> &MemCell {
        if let Some(cell) = self.cells.get(ptr) {
            return cell.as_ref().unwrap();
        }
        else {
            Throw!(format!("Memory::get_error :> Invalid Pointer. Variable not found at index {}.", ptr));
        }
    }

    pub fn get_mut(&mut self, ptr: usize) -> &mut MemCell {
        if let Some(cell) = self.cells.get_mut(ptr) {
            return cell.as_mut().unwrap();
        }
        else {
            Throw!(format!("Memory::get_error >> Variable not found at index {}", ptr));
        }    
    }

    pub fn dealloc(&mut self, ptr: usize) {
        self.cells[ptr] = None;
        self.freelist.push(ptr);
    }

}


impl Env {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()]
        }
    }

    pub fn insert(&mut self, name: String, ptr: usize) {
        // inserts the name and pointer in the last Hashmap (stack structure)
        self.scopes
        .last_mut()     // returns a mutable reference to the last element of the Vec
        .unwrap()
        .insert(name, ptr);
    }

    pub fn get(&self, name: &str) -> &usize {
        for scope in self.scopes.iter().rev() {
            // Error is handled in
            if let Some(ptr) = scope.get(name) {
                return ptr;
            }
            else {
                Throw!(format!("Env_get Error: Variable `{}` not found in memory", name));
            }
        }
        // ! fix this later
        println!("BUG WARNING :- Env::get() function");
        println!("Bro this is an unhandled bug, alive since 09-05-2026 | 6:04PM");
        return &usize::MAX;
    }

    pub fn remove(&mut self, name: &str) -> usize {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(ptr) = scope.remove(name) {
                return ptr;
            }
            else {
                Throw!(format!("Env_remove Error: Variable `{}` not found in memory", name));
            }
        }
        // ! fix this later
        println!("BUG WARNING :- Env::remove() function");
        println!("Bro this is an unhandled bug, alive since 09-05-2026 | 6:04PM");
        return usize::MAX;
    }

    
}