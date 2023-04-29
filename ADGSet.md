# 有集合自動微分圖

## 訴求

- Lazy

    移除節點或邊要能延後一次處理以減少不必要的延遲，主要用於 Optimizer

- 有 Set

    Node 要有 Parent Set且能快速找到自己的 Sub Set，對應[IR的Scope](./IR.md#scope)

- 能接受大量節點變更
- 要有Optimizer

## 理論

在基本的ADG的實現上加入Set的概念，所有集合只有 $A\subset B$ 或 $A\cap B = \empty$ 的關係，函數本身是 $U$，當BP時只會處理  $\forall \subset S$ ，利用分別處理Subset的BP以減少大量的全域變數(註: 放在函數開頭)

實作中會有一個變數映射表，用於查找變數，當一個可變變數賦值時會更改指向的節點為賦值的節點，ADG中變數節點只有函數輸入值，本地變數只會如上所述，在變數映射表中出現

## 測試

### [test1](./EdgeCase.md#test1)

無法處理

### [test1](./EdgeCase.md#test2)

TODO

### [test3](./EdgeCase.md#test3)

![test3](images/set-test3.png)

### [test4](./EdgeCase.md#test4)

TODO

### [test5](./EdgeCase.md#test5)

![Alt text](images/set-test5.png)
