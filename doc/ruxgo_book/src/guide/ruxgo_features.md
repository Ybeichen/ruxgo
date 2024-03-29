# ​设计特性

Ruxgo 设计充分考虑构建过程的效率、可靠性和可维护性，同时充分适应 RuxOS 的构件化特性：

- **多样化的编译选项**：Ruxgo 支持多种编译器（如 GCC、Clang、MSVC）和多样化的中间过程（如生成静态库、动态库、目标文件等），为最佳性能提供灵活选择。
- **增量与并行构建**：通过增量构建，Ruxgo 只编译修改过的文件或模块，大幅缩短构建时间。同时利用多核处理器同时编译多个文件，实现并行构建，进一步提高效率。
- **简化的配置文件**：完全使用 Toml 文件来描述构建参数，避免使用复杂的语法和依赖规则的构建流程。开发者只专注于构建逻辑，而非配置文件的复杂性
- **依赖管理与可移植性**：结合包管理功能和 crate.io 生态来简化外部库和组件的依赖管理。同时代码尽量使用平台无关的 API，增强跨平台兼容性。

- **构建过程模块化**：构建过程以 Target 为中心，按照不同的 Target 模块来配置源文件和依赖库。构建应用程序可以像搭积木一样进行。

- **源码和构建目录分离**：构建过程中生成的中间文件都会被放置在构建目录`ruxgo_bld/`中，防止构建过程中文件冲突和混乱。

- **友好的编译错误诊断**：使用更友好的编译器错误输出显示，包括清晰的错误提示、指向源代码行数等信息。