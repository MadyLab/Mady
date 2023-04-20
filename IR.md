# IR 設計

- ir
  - [Scope](#scope)
  - [Method](#method)
  - [Call](#call)
  - [Ret](#ret)
  - [Dec](#dec)
  - [Lit](#lit)
  - [Code](#code)
- type
  - [Id](#id)
  - [Var](#var)

## Scope

```rust
struct Scope {
    id: ScopeId,
    ty: ScopeTy,
    content: Ir
}
```

## Method

呼叫 `method`

```rust
struct Method<N> {
    receiver: Var,
    name: N,
    args: Vec<Var>,
}
```

## Call

呼叫函數

```rust
struct Call<P, G> {
    path: P,
    generic: Option<G>,
    args: Vec<Var>,
}
```

## Ret

回傳值，要指定 `scope`

```rust
struct Ret {
    var: Option<Var>,
    scope: ScopeId,
}
```

## Assign

將變數賦值

```rust
struct Assign {
    var: Var,
    value: Ir,
}
```

## Lit

常數類型，不會計算偏微分且會被ADG優化掉，像是非與函數輸入值有關係的值

```rust
struct Lit<L> (L);
```

## Code

就是忽略的程式碼，不會參與ADG生成，只會在IR生成時補回

```rust
struct Code<C>(C);
```

## Id

內部Id，可能會變動

## Var

```rust
enum Var<N> {
    Tmp(VarId),
    Named(N)
}
```
