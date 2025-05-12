use hashbrown::HashMap;
use std::{ptr, sync::Mutex};

static MTX_VARS: Mutex<Option<HashMap<String, u64>>> = Mutex::new(None);

/// Initializes a global variable with the given key and value.
/// 
/// # Arguments
/// 
/// * `key` - A string slice that holds the identifier for the global variable
/// * `val` - A generic value of type T to be stored globally
/// 
/// # Safety
/// 
/// This function uses unsafe operations internally to store values in global state.
/// The value is leaked into a static lifetime and its memory address is stored.
pub fn init_global_var<T>(key: &str, val: T) {
    if let Ok(mut guard) = MTX_VARS.lock() {
        if guard.is_none() {
            *guard = Some(HashMap::new());
        }
        let boxed = Box::new(val);
        let boxed_ptr = Box::leak(boxed);
        guard
            .as_mut()
            .unwrap()
            .insert(key.to_string(), ptr::addr_of_mut!(*boxed_ptr) as u64);
    }
}

/// Removes and drops a global variable of type T stored with the given key.
///
/// # Arguments
/// * `key` - The string key used to identify the global variable
///
/// # Safety
/// This function contains unsafe code for deallocating raw pointers.
/// Caller must ensure the type T matches the actual stored type.
pub fn drop_global_var<T>(key: &str) {
    if let Ok(mut guard) = MTX_VARS.lock() {
        let opt_ptr = guard.as_mut().unwrap().remove(key);
        if let Some(ptr) = opt_ptr {
            unsafe {
                drop(Box::from_raw(ptr as *mut T));
            }
        }
    }
}

/// Retrieves a reference to a global variable of type T stored with the given key.
/// 
/// # Arguments
/// 
/// * `key` - The string key used to identify the global variable
/// 
/// # Returns
/// 
/// * `Ok(&'static T)` - A static reference to the stored value if found
/// * `Err(String)` - Error message if mutex lock fails or key not found
/// 
/// # Safety
/// 
/// This function contains unsafe code when dereferencing the stored pointer.
/// Caller must ensure the type T matches the actual stored type.
pub fn fetch_global_var<T>(key: &str) -> Result<&'static T, String> {
    if let Ok(guard) = MTX_VARS.lock() {
        if guard.is_none() {
            Err("Failed to lock mutex".to_string())
        } else if let Some(boxed_ptr) = guard.as_ref().unwrap().get(key) {
            Ok(unsafe { &*(*boxed_ptr as *const T) })
        } else {
            Err("Failed to find key".to_string())
        }
    } else {
        Err("Failed to lock mutex".to_string())
    }
}

/// Retrieves a mutable reference to a global variable of type T by its key.
///
/// # Arguments
///
/// * `key` - The string key associated with the global variable
///
/// # Returns
///
/// * `Ok(&'static mut T)` - A mutable static reference to the global variable
/// * `Err(String)` - Error message if mutex lock fails or key not found
///
/// # Safety
///
/// This function contains unsafe code for raw pointer dereferencing.
/// Caller must ensure type T matches the stored variable type.
pub fn fetch_global_var_mut<T>(key: &str) -> Result<&'static mut T, String> {
    if let Ok(guard) = MTX_VARS.lock() {
        if guard.is_none() {
            Err("Failed to lock mutex".to_string())
        } else if let Some(boxed_ptr) = guard.as_ref().unwrap().get(key) {
            Ok(unsafe { &mut *(*boxed_ptr as *mut T) })
        } else {
            Err("Failed to find key".to_string())
        }
    } else {
        Err("Failed to lock mutex".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{drop_global_var, fetch_global_var, fetch_global_var_mut, init_global_var};

    struct Foo {
        id: u64,
        name: String,
    }

    #[test]
    fn test_global_var_u64() {
        init_global_var("test1", 42_u64);
        let result = fetch_global_var("test1");
        assert_eq!(result, Ok(&42_u64));
        drop_global_var::<u64>("test1");
    }

    #[test]
    fn test_global_var_u64_mut() {
        init_global_var("test2", 42_u64);
        if let Ok(val) = fetch_global_var_mut::<u64>("test2") {
            *val += 1;
        }
        let result = fetch_global_var("test2");
        assert_eq!(result, Ok(&43_u64));
        drop_global_var::<u64>("test2");
    }

    #[test]
    fn test_global_var_struct() {
        init_global_var(
            "test3",
            Foo {
                id: 1,
                name: "bar".to_string(),
            },
        );
        if let Ok(foo) = fetch_global_var::<Foo>("test3") {
            assert_eq!(foo.id, 1);
            assert_eq!(foo.name, "bar");
        }
        drop_global_var::<Foo>("test3");
    }

    #[test]
    fn test_global_var_struct_mut() {
        init_global_var(
            "test4",
            Foo {
                id: 1,
                name: "bar".to_string(),
            },
        );
        if let Ok(foo) = fetch_global_var_mut::<Foo>("test4") {
            foo.id += 1;
            foo.name += "1";
        }
        if let Ok(foo) = fetch_global_var::<Foo>("test4") {
            assert_eq!(foo.id, 2);
            assert_eq!(foo.name, "bar1");
        }
        drop_global_var::<Foo>("test4");
    }
}
