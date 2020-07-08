
## 第14章 Cargo 和 Crates.io


### 模块系统（the module system）包括：

包（Packages）： Cargo 的一个功能，它允许你构建、测试和分享 crate。
Crates ：一个模块的树形结构，它形成了库或二进制项目。
模块（Modules）和 use： 允许你控制作用域和路径的私有性。
路径（path）：一个命名例如结构体、函数或模块等项的方式


一个包中至多 只能 包含一个库 crate(library crate)；
包中可以包含任意多个二进制 crate(binary crate)；
包中至少包含一个 crate，无论是库的还是二进制的。


Cargo 遵循的一个约定：src/main.rs 就是一个与包同名的二进制 crate 的 crate 根。同样的，Cargo 知道如果包目录中包含 src/lib.rs，则包带有与其同名的库 crate，且 src/lib.rs 是 crate 根。crate 根文件将由 Cargo 传递给 rustc 来实际构建库或者二进制项目。

如果一个包同时含有 src/main.rs 和 src/lib.rs，则它有两个 crate：一个库和一个二进制项，且名字都与包相同。通过将文件放在 src/bin 目录下，一个包可以拥有多个二进制 crate：每个 src/bin 下的文件都会被编译成一个独立的二进制 crate

Rust 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的。父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项。


### Cargo 工作空间

工作空间 是一系列共享同样的 Cargo.lock 和输出目录的包。
我们的工作空间有一个二进制项目和两个库。二进制项目会提供主要功能，并会依赖另两个库。




> - 首页: [README.md](../README.md)
> - 上一章: [第13章-迭代器与闭包](./第13章-迭代器与闭包.md)
> - 下一章:  [第15章-智能指针](./第15章-智能指针.md)