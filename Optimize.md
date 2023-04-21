# 優化

## LLVM

- [rust#58329](https://github.com/rust-lang/rust/issues/58329)

    LLVM需要確保是pure function才能優化，但 `extern C` 無法被確認

    [範例](https://rust.godbolt.org/z/YT4cM4Kd1)
