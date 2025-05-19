use std::ptr;

/// Creates a heap-allocated global pointer to the given value and returns its raw address.
/// The pointer will not be automatically deallocated (leaked intentionally).
/// 
/// # Safety
/// The caller is responsible for ensuring proper memory management of the leaked value.
/// The returned pointer must not be used after the value is dropped.
pub fn def_global_ptr<T>(value: T) -> u64 {
    let boxed = Box::new(value);
    let p = Box::leak(boxed);
    ptr::addr_of_mut!(*p) as u64
}

/// Returns a mutable reference to a global variable of type `T` from the given pointer.
///
/// # Safety
/// The caller must ensure:
/// - The pointer `p` is valid and points to a properly initialized value of type `T`.
/// - The returned reference has a static lifetime, meaning the pointed data must live forever.
/// - No other references (mutable or immutable) to the same data exist when using this function.
/// - Proper synchronization is used if accessed from multiple threads.
pub fn get_global_mut<T>(p: u64) -> &'static mut T {
    let gv_ref = p as *mut T;
    unsafe { &mut *gv_ref }
}

/// Returns a static reference to a value at the given memory address.
///
/// # Safety
///
/// This is highly unsafe as it dereferences a raw pointer without any checks.
/// The caller must ensure:
/// - The pointer is valid and properly aligned
/// - The memory contains a properly initialized value of type T
/// - The referenced data will remain valid for the 'static lifetime
/// - No mutable references exist to the same memory
/// - No data races can occur when accessing the reference
pub fn get_global<T>(p: u64) -> &'static T {
    let gv_ref = p as *mut T;
    unsafe { &*gv_ref }
}

/// Converts a raw pointer to a `Box<T>` and immediately drops it, effectively
/// deallocating the memory. This is unsafe as it assumes the pointer is valid
/// and properly aligned. Use with caution to avoid undefined behavior.
pub fn undef_global_ptr<T>(p: u64) {
    let gv_ref = p as *mut T;
    unsafe {
        let boxed = Box::from_raw(gv_ref);
        drop(boxed);
    };
}

#[cfg(test)]
mod tests {
    use super::{def_global_ptr, get_global, get_global_mut, undef_global_ptr};

    struct Foo {
        id: u64,
        name: String,
    }

    #[test]
    fn test() {
        let p = def_global_ptr(Foo {
            id: 1,
            name: "bar".to_string(),
        });
        let foo = get_global::<Foo>(p);
        assert_eq!(foo.id, 1);
        assert_eq!(foo.name, "bar");
        let foo = get_global_mut::<Foo>(p);
        foo.id += 1;
        let foo = get_global::<Foo>(p);
        assert_eq!(foo.id, 2);
        undef_global_ptr::<Foo>(p);
    }
}
