# IR

## Goal

- can construct all differentiable syntax
- 要能表示所有可微分語法
- Can modify a lot of node (use link list)
- can convert from `syn`

## Design

- ir
  - [Scope](#scope)
  - [Method](#method)
  - [Call](#call)
  - [Ret](#ret)
  - [Assign](#assign)
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
    content: LinkList<Ir>
}
```

```rust
enum ScopeTy {
    If,
    Else,
    Loop,
    Closure,
    Fn,
}
```

## Method

call `method`

```rust
struct Method<MP, G> {
    receiver: Var,
    path: MP,
    generic: Option<G>,
    args: Vec<Var>,
}
```

## Call

call `fn`

```rust
struct Call<CP, G> {
    path: CP,
    generic: Option<G>,
    args: Vec<Var>,
}
```

## Ret

return value, `ScopeId` is required

```rust
struct Ret {
    var: Option<Var>,
    scope: ScopeId,
}
```

## Assign

assign variable, `A` can be a destruct pattern

```rust
struct Assign<A:AsRef<[Var]>> {
    vars: A,
    value: Ir,
}
```

## Lit

Literal. e.g. `"Hello World"`、`1234`

```rust
struct Lit<L> (L);
```

## Code

Raw Code

```rust
struct Code<C>(C);
```

## Id

Internal Id. Cannot be create.

## Var

variable. A variable without var field is view as temp variable, it may be removed by optimizer.

```rust
struct Var<V> {
    id: VarId,
    var: Option<V>
}
```
