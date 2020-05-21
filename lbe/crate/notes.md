当执行`rustc abc.rs`时，`abc.rs`被当作`crate`文件，是rust的**编译单元**，如果`abc.rs`中含由`mod`声明，那么这个mod的内容就被插到这个地方，**mod不会被单独编译**。

使用`extern crate rary`来导入crate，使用时`crate_name::mod1::mod2::function()`。(不知道crate可不可以嵌套)