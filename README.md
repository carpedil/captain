# RUST+ Svelte5 + 数据库 + Linux学习记录

## Rust

### 基础

https://www.bilibili.com/video/BV1MGq5YQE4f/?p=52&spm_id_from=333.1007.top_right_bar_window_history.content.click&vd_source=beb59e72918195aa4fe8f96549b6824e

#### 001-hello Rust

```rust
fn main() {
   println!("hello Rust!")
}
```

> rust中，使用fn作为声明函数的关键字，后面跟函数名，main函数是rust程序的入口函数
> 
> 上面的示例中，在main函数里面调用了println！宏在终端打印`hello Rust！`，在项目文件根目录终端里面执行`cargo run`即可看到程序运行的结果。

#### 002-变量

```rust
// creation
let a = 5;
let b:i32 = 10;
// mutability
let mut c = 2;
c = 4;
// shadowing
let c = 10;
// scop
{
   let d = "inner scop";
   println!("d : {}",d);
}
// println!(d)//  // d is not valid here
```

### 

高级

https://www.bilibili.com/video/BV1MGq5YQE4f/?p=52&spm_id_from=333.1007.top_right_bar_window_history.content.click&vd_source=beb59e72918195aa4fe8f96549b6824e

### 第三方的库

### 日志

### 错误处理

### Oracle驱动

### orm

### excel读写

### Svelte5

#### 基础

#### 进阶

#### 第三方库

### 数据库

### 基础

### 进阶

### Linux

#### 常用命令

