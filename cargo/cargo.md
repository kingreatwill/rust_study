# cargo介绍

作为一门现代语言，rust自然要摒弃石器时代项目代码管理的方法和手段。rust项目组为各位猿提供了超级大杀器cargo，以解决项目代码管理所带来的干扰和困惑。用过node.js的猿们，应该对node.js中的神器npm、grunt、gulp等工具印象深刻。作为新一代静态语言中的翘楚，rust官方参考了现有语言管理工具的优点，于是就产生了cargo。

言而总之，作为rust的代码组织管理工具，cargo提供了一系列的工具，从项目的建立、构建到测试、运行直至部署，为rust项目的管理提供尽可能完整的手段。同时，与rust语言及其编译器rustc本身的各种特性紧密结合，可以说既是语言本身的知心爱人，又是rust猿们的贴心小棉袄，谁用谁知道。

# toml配置文件
https://github.com/toml-lang/toml

# 创建项目 hellorust
 cargo new hellorust --bin
# 编译和运行

cargo build

cargo build --release # 这个属于优化编译

./target/debug/hellorust.exe

./target/release/hellorust.exe # 如果前面是优化编译，则这样运行

cargo run # 编译和运行合在一起

cargo run --release # 同上，区别是是优化编译的

# 建议项目结构
```
.
├── Cargo.toml
├── Cargo.lock
├── tests
├── examples
├── benches
├── target
    ├── debug
    └── release
└── src
    ├── lib.rs
    └── main.rs
```

cargo.toml和cargo.lock文件总是位于项目根目录下。

源代码位于src目录下。

默认的库入口文件是src/lib.rs。

默认的可执行程序入口文件是src/main.rs。

其他可选的可执行文件位于src/bin/*.rs(这里每一个rs文件均对应一个可执行文件)。

外部测试源代码文件位于tests目录下。

示例程序源代码文件位于examples。

基准测试源代码文件位于benches目录下。

# 定义项目依赖

基于rust官方仓库crates.io，通过版本说明来描述：

基于项目源代码的git仓库地址，通过URL来描述：

基于本地项目的绝对路径或者相对路径，通过类Unix模式的路径来描述： 这三种形式具体写法如下：
````
[dependencies]
typemap = "0.3"
plugin = "0.2*"
hammer = { version = "0.5.0"}
color = { git = "https://github.com/bjz/color-rs" }
geometry = { path = "crates/geometry" }
````
