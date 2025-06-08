# GlobalVar

GlobalVar 是一个 Rust 库，提供了两种不同的全局变量管理实现方式。该库允许你在 Rust 程序中安全地管理全局状态，支持任意类型的全局变量。

## 功能特点

- 支持两种全局变量管理方式：
  - 基于键值对的全局变量管理
  - 基于指针的全局变量管理
- 支持任意类型的全局变量
- 线程安全的实现
- 提供可变和不可变引用访问
- 内存安全的资源管理

## 使用方式

### 基于键值对的全局变量

这种方式使用字符串键来标识全局变量：

```rust
use globalvar::global_kv::{init_global_var, fetch_global_var, fetch_global_var_mut, drop_global_var};

// 初始化全局变量
init_global_var("counter", 42_u64);

// 获取不可变引用
if let Ok(value) = fetch_global_var::<u64>("counter") {
    println!("Counter value: {}", value);
}

// 获取可变引用并修改
if let Ok(value) = fetch_global_var_mut::<u64>("counter") {
    *value += 1;
}

// 删除全局变量
drop_global_var::<u64>("counter");
```

### 基于指针的全局变量

这种方式直接管理全局指针：

```rust
use globalvar::global_ptr::{def_global_ptr, get_global, get_global_mut, undef_global_ptr};

// 创建全局变量并获取指针
let ptr = def_global_ptr(42_u64);

// 获取不可变引用
let value = get_global::<u64>(ptr);
println!("Value: {}", value);

// 获取可变引用并修改
let value = get_global_mut::<u64>(ptr);
*value += 1;

// 删除全局变量
undef_global_ptr::<u64>(ptr);
```

## 安全性说明

该库使用了 unsafe Rust 代码来实现全局状态管理，但提供了安全的公共 API。使用时需要注意：

1. 确保正确匹配存储和获取时的类型
2. 在不再需要时及时清理全局变量
3. 注意多线程环境下的同步访问

## 实现细节

### 键值对实现 (global_kv)

- 使用 `Mutex` 保证线程安全
- 使用 `HashMap` 存储键值对
- 支持动态添加和删除全局变量
- 提供错误处理机制

### 指针实现 (global_ptr)

- 直接管理内存指针
- 更轻量级的实现
- 适用于固定的全局状态
- 需要更谨慎的内存管理

## 注意事项

1. 该库主要用于需要全局状态管理的场景
2. 建议优先考虑其他状态管理方案，只在确实需要全局变量时使用
3. 使用基于键值对的实现可以获得更好的安全性和便利性
4. 使用基于指针的实现可以获得更好的性能，但需要更谨慎的处理

## 许可证

[待补充]

## 贡献

欢迎提交 Issue 和 Pull Request！
