# rusty-lake

这个 repo 记录我学习 rust 的一些笔记。

1。浮点数比较

其实不光是 rust 其他语言都一样。浮点数判等不要用 ==，要用差值小于 eps。只不过 rust 会 panic。
