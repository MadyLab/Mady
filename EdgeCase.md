# Edge Case

## 規範

### Lit

```rust
lit!();
```

常數類型，不與fn輸入值有關
包含 `call(lit!())`

### Expr

```rust
expr!(...)
```

`...`為`expr`牽扯的變數

## 測試

### test1

```rust

fn test_1(a: T) -> T {
    let a = lit!();
    take(a);
    a + lit!()
}

fn take(_: T) {}
```

### test2

```rust

fn test_2(a: T) -> T {
    a.map(|x| expr!(x))
}
```

### test3

```rust

fn test_3(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}
```

### test4

```rust

fn test_4(a: T,b: T) -> T {
    a.call(b)
}
```

### test5

```rust

fn test_5(a: T, b: T) -> T {
    let c = a + b;
    let d = c * a;
    d
}
```
