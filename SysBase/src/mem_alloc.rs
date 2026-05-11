use crate::defkeys::*;
use crate::memory_layout::*;
use crate::Throw;

//--------------------------------------------------------------------------------------------------------------------------------------
// fast forward 2 years. WHAT THE FUCK DOES THIS MEAN???? 😭😭🥲
//--------------------------------------------------------------------------------------------------------------------------------------

// REFACTORING TIME ! (08-05-2026 7:26pm)
/*
! Changes 
? OLD: 
    OLD : checks if the name exists and is it mutable
    OLD : Used later for checking if the new and old data type matches thus enforcing typecheck

? NEW : 
    Implicity checks if the name exist, is_mutable, and in future will enforce typesafety too.
    No need of fetch_str. Only IDs can be passed. fetch_str returns Ok() even for string_literals. 
    Implement `match line[1] {ID => Ok ; _ => Fuck you} 
*/

// TODO: ENFORCE TYPE SAFETY
pub fn mutate_mem(
    line: &Vec<Builtins>,
    mem: &mut Memory,
    env: &Env
) {

    let target_name = match &line[1] {
        Builtins::ID(id) => id,
        _ => Throw!("MutateMemError: Expected TARGET to be variable name. Got some other shit")
    };

    let source = match &line[2] {
        Builtins::ID(id) => id,
        _ => Throw!("MutateMemError: Expected SOURCE to be variable name. Got some other shit")
    };

    let source_data = {
        mem.get(
            *env.get(source)
        ).data.clone()
    };

    let target_ptr = env.get(target_name);
    let target_mem_cell = mem.get_mut(*target_ptr);

    let old_data = &target_mem_cell.data;

    if type_safety(old_data, &source_data) {
        if target_mem_cell.is_mutable {
            // resolve the id internally and assign its value to target variable
            // By defination line[2] is gonna be of type Builtins::ID() (enforced in EXECUTE.rs)
            target_mem_cell.data = source_data;
        }
        else {
           Throw!(format!("MutateMemError: The variable `{:?}` isn't mutable", target_name)); 
        }
    }
    else {
        (); // type_safety(...) throws the error automatically !
    }

}
// Completed at 09:55PM 8/5/26

//--------------------------------------------------------------------------------------------------------------------------------------

// insert_to_mem is called when the data is already resolved and is only required to be put in memory.
// direct_value is of type Builtins::D_Type
// TODO: ENFORCE TYPE SAFETY
pub fn insert_to_mem(
    line: &Vec<Builtins>,
    mem: &mut Memory,
    env: &Env,
    direct_value: &Builtins,
) {

    let target_name = match &line[1] {
        Builtins::ID(id) => id,
        _ => Throw!("MutateMemError: Expected variable name. Got some other shit")
    };

    let target_ptr = env.get(target_name);
    let mem_cell = mem.get_mut(*target_ptr);
    let old_data = &mem_cell.data;

    if type_safety(&old_data, direct_value) {
        if mem_cell.is_mutable {
            mem_cell.data = direct_value.to_owned() ;
        }
        else {
            Throw!(format!("MutateMemError: The variable `{:?}` isn't mutable", target_name));
        }
    }
    else {
        (); 
    }

 
}


// New addition to this file after i guess 2 years
/*
? Steps to remove a variable from memory
    1. Fetch the pointer using env.get(target_name)
    2. Fetch the MemoryCell using mem.get(*ptr)
    3. Remove the entry from the mem.cells. Let the vector adjust itself

! MAJOR ISSUE WITH THIS :
    After removing a value from the Cells (vec), the position of the elements after it is shifted too. 
    This alters the position i.e. the POINTER of it w.r.t. to the HashMap Env

* My Solution 
    1) follow steps 1 and 2 like before.
    2) Only set the data at mem.cells[*ptr] as NULL or similar shit
    3) Remove the pointer to it
    ? Kinda okay, yk works.

    4) Implementing a freelist that stores the deallocated memory slots free to use. O(1) operation as the first slot is used generally

*/
pub fn remove_from_mem(
    line: &Vec<Builtins>,
    mem: &mut Memory,
    env: &mut Env
) {
    let target_name = match &line[1] {
        Builtins::ID(id) => id,
        _ => Throw!("RemoveMemError: Expected variable name. Got some other shit")
    };

    let target_ptr = env.remove(target_name);
    mem.dealloc(target_ptr);
}

//--------------------------------------------------------------------------


fn type_safety(v1: &Builtins, v2: &Builtins) -> bool {
    match (v1, v2) {
        (Builtins::D_type(D_type::int(_)), Builtins::D_type(D_type::int(_))) => true,
        (Builtins::D_type(D_type::float(_)), Builtins::D_type(D_type::float(_))) => true,
        (Builtins::D_type(D_type::str(_)), Builtins::D_type(D_type::str(_))) => true,
        (Builtins::D_type(D_type::bool(_)), Builtins::D_type(D_type::bool(_))) => true,

        /* (Builtins::D_type(D_type::int(_)), Builtins::D_type(D_type::float(_)))
        | (Builtins::D_type(D_type::float(_)), Builtins::D_type(D_type::int(_))) => {
            if allowModif {
                true
            } else {
                false
            }
        } */
        _ => crate::Throw!(format!(
            "Can't insert a value of type {:?} into a variable of type {:?}",
            v2.get_data_type(),
            v1.get_data_type()
        )),
    }
}