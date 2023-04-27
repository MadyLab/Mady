# Edge Case

## 規範

### Lit

```rust
lit!();
```

常數類型，不與fn輸入值有關
包含 `call(lit!())`

### Closure

```rust
closure!(...)
```

`...`為closure輸入值

## 測試

```rust
fn test_1(a: T) -> T {
    let a = lit!();
    take(a);
    a + lit!()
}

fn take(_: T) {}
```

```rust
fn test_2(a: T) -> T {
    a.map(closure!(x))
}
```
