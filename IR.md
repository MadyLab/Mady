# IR

## 訴求

- 要能表示所有可微分語法
- 可接受大量變更，可以不用隨機存取
- 要能與 `syn` 做轉換
- 基本的 Optimizer

## 設計

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

## Method

呼叫 `method`

```rust
struct Method<MP, G> {
    receiver: Var,
    path: MP,
    generic: Option<G>,
    args: Vec<Var>,
}
```

## Call

呼叫函數

```rust
struct Call<CP, G> {
    path: CP,
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

內部Id，可能會變動，不能自行創建

## Var

變數，Tmp會由IR Optimizer決定是否生成

```rust
enum Var<N> {
    Tmp(VarId),
    Named(N)
}
```
