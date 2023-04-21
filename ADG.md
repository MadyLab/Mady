# 自動微分圖

## 訴求

- Lazy

    移除節點或邊要能延後一次處理以減少不必要的延遲，主要用於 Optimizer

- 有 Set

    Node 要有 Parent Set且能快速找到自己的 Sub Set，對應[IR的Scope](./IR.md#scope)

- 能接受大量節點變更
- 要有Optimizer
