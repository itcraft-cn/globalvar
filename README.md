# GlobalVar

GlobalVar is a Rust library that provides two different implementations for managing global variables. This library allows you to safely manage global state in Rust programs, supporting global variables of any type.

## Features

- Supports two global variable management approaches:
  - Key-value based global variable management
  - Pointer-based global variable management
- Supports global variables of any type
- Thread-safe implementation
- Provides both mutable and immutable reference access
- Memory-safe resource management

## Usage

### Key-Value Based Global Variables

This approach uses string keys to identify global variables:

```rust
use globalvar::global_kv::{init_global_var, fetch_global_var, fetch_global_var_mut, drop_global_var};

// Initialize a global variable
init_global_var("counter", 42_u64);

// Get an immutable reference
if let Ok(value) = fetch_global_var::<u64>("counter") {
    println!("Counter value: {}", value);
}

// Get a mutable reference and modify
if let Ok(value) = fetch_global_var_mut::<u64>("counter") {
    *value += 1;
}

// Remove the global variable
drop_global_var::<u64>("counter");
```

### Pointer-Based Global Variables

This approach directly manages global pointers:

```rust
use globalvar::global_ptr::{def_global_ptr, get_global, get_global_mut, undef_global_ptr};

// Create a global variable and get its pointer
let ptr = def_global_ptr(42_u64);

// Get an immutable reference
let value = get_global::<u64>(ptr);
println!("Value: {}", value);

// Get a mutable reference and modify
let value = get_global_mut::<u64>(ptr);
*value += 1;

// Remove the global variable
undef_global_ptr::<u64>(ptr);
```

## Safety Notes

This library uses unsafe Rust code to implement global state management but provides a safe public API. When using it, please note:

1. Ensure correct type matching between storage and retrieval
2. Clean up global variables when they are no longer needed
3. Be mindful of synchronized access in multi-threaded environments

## Implementation Details

### Key-Value Implementation (global_kv)

- Uses `Mutex` to ensure thread safety
- Uses `HashMap` to store key-value pairs
- Supports dynamic addition and removal of global variables
- Provides error handling mechanisms

### Pointer Implementation (global_ptr)

- Directly manages memory pointers
- Lighter-weight implementation
- Suitable for fixed global states
- Requires more careful memory management

## Important Notes

1. This library is primarily intended for scenarios requiring global state management
2. Consider other state management solutions first, and only use global variables when truly necessary
3. The key-value based implementation offers better safety and convenience
4. The pointer-based implementation offers better performance but requires more careful handling

## License

[To be added]

## Contributions

Issues and Pull Requests are welcome!