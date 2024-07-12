# 入门教程：Rust基础.md
本教程旨在帮助有其它编程语言基础（C++、Python等）的同学，快速熟悉Rust的语法和基本特性。

更详细的版本（强烈推荐各位入门Rust语言的都去看！！！）：https://course.rs

也可以关注一下：https://beatai.cn/

## 一、第一个Rust程序：Hello World
```rust
/// 和C/C++一样，熟悉的main函数
fn main() {
    /// 类似C/C++中的printf函数和Python中的print函数
    println!("Hello World");
    /// 当然你也可以：
    println!("{} {}", "Hello", "World");
    /// 更多用法后面再说
}
```

## 二、先来点熟悉的
以下部分，几乎所有其它编程语言都有。下面仅展示这些部分在Rust中的使用方式，以及与C/C++的不同之处。

### 定义变量
定义不可变值（定义后不可修改）：
```rust
let var = 0;
```
定义变量（定义后可修改）：
```rust
let mut var = 0;
```
修改变量：
```rust
var = 1;
```
定义布尔值变量：
```rust
// Rust会自动推断变量类型
let mut var_1 = true;
// 当然，你也可以手动指定
let mut var_2: bool = true;
```

### 基本类型
> 原帖：https://course.rs/basic/base-type/numbers.html<br>
> 包含处理整型溢出的方式、以及使用浮点数时的一些注意事项（如NaN、测试相等性），推荐阅读！

- 有符号整数（可表示负值）：`i8`、`i16`、`i32`、`i64`、`i128`、`isize`。
- 无符号整数（不可表示负值）：`u8`、`u16`、`u32`、`u64`、`u128`、`usize`。
- 浮点数：`f32`、`f64`。
- 字符：`char`，表示一个Unicode字符，占用4字节。
- 布尔值：`bool`，只有两种可能值`true`和`false`。
- 单元类型：`()`，只有一种可能值`()`。

> 对于整数，未指定且推断不出类型时，默认使用`i32`；浮点数则为`f64`。

注意：
1. 运算时，必须保证两边类型相同。对于基本类型，可以使用`as`来转换类型，如下所示：
```rust
let num_1: u64 = 1;
let num_2: usize = 2;
// let num_3 = num_1 + num_2; // 此行无法通过编译
let num_3 = num_1 + num_2 as u64; // 使用as示例1
let num_3 = num_1 as usize + num_2; // 使用as示例2
```
2. 请使用`u8`表示单个字节，不同于C/C++中的`unsigned char`。
3. 数组、切片等只能使用`usize`类型的数值作为下标。

### 控制流程
> 原帖：https://course.rs/basic/flow-control.html

#### If分支语句和While循环语句
```rust
if condition_1 {
    ctrl_1
}
else if condition_2 {
    ctrl_2
}
else {
    ctrl_3
}

while condition_loop {
    ctrl_loop
}
```
其实就是C/C++的用法，只不过判断条件那里不用打括号了。

#### Match分支语句
```rust
let num = 1;
match num {
    1 => {
        println!("One!");
    }
    2 => {
        println!("Two!");
    }
    other_num => {
        println!("Other num: {}", other_num);
    }
}
```
此处Match语句的含义：对数值`num`，按顺序匹配下面的3个分支：
- 如果`num == 1`：进入第一个分支，输出`One!`。
- 如果`num == 2`：进入第二个分支，输出`Two!`。
- 如果以上两个分支都没进入，则将`num`赋值给临时变量`other_num`，然后进入第三个分支。

对于一个Match语句：
1. 必须包括传入的值的类型所有情况，不能重复，也不能遗漏。如果不希望为每个可能的值写一个分支，可以在Match语句的最后，写一个分支来捕获传入的变量。
2. 每次执行Match语句时，会且只会进入Match语句的其中一个分支。

#### For循环语句
Rust的For循环语句作用于迭代器类型，当然如果我们希望按下标迭代的话，以下是一个例子：

```rust
for i in 0..5 {
    println!("{}", i);
}
```
以上程序将输出5行，第一行为0，第5行为4。

For循环语句更多用法后面细说。

#### Loop循环语句
表示一个无限循环。
```rust
loop {
    ctrl
}
```

#### 循环跳过语句
以下循环跳过语句可用于所有循环语句（For、While、Loop）。

1. `break`：跳出当前循环。
```rust
for id in 1..=5 {
    if id % 2 == 0 {
        break;
    }
    println!("ID: {}", id);
}
```
`id == 2`时触发break，循环退出，最终输出的ID只有1。

2. `continue`：结束当前轮循环。
```rust
for id in 1..=5 {
    if id % 2 == 0 {
        continue;
    }
    println!("ID: {}", id);
}
```
`id == 2`时触发continue，后面的部分不再进行，但不影响后面轮次循环。`id == 4`时同理，最终输出的ID有1、3和5。

### 函数
> If和While的用法像C/C++，For的用法和函数的定义方法像Python，好听点说，集百家之长。

这是一个加法函数，返回`a`和`b`之和：
```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```
这样写不够Rusty，换个写法：
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
感到一脸懵逼的话，过后就明白为什么可以这么写了。

对于没有返回值的函数：
```rust
fn print_add(a: i32, b: i32) {
    println!("{}", a + b);
}
```

### 结构体
> 类似C/C++的结构体，但是写法有很大不同，在各自语言中的地位也不同。

以下为演示结构体用法的伪代码。
```rust
// 定义结构体
// 里边只能有结构体成员
struct MyStruct {
    // 成员1
    member_1: MyMemberType1,
    // 成员2
    member_2: MyMemberType2,
}

// 定义结构体相关常量、函数和方法
// 一个类型可以有多个impl块，每个impl块可以定义多个常量/函数/方法
impl MyStruct {
    // 若第一个参数为self，则其通常称为方法，self表示调用方法的该类型对象。
    // self的三种表示的区别：
    // &self：可读不可写。
    // &mut self：可读可写。
    // self：拿走所有权，很少用。
    // 参考部分：所有权与借用
    fn func(&self) {
        ctrl
    }

    fn new(arg_1: MyMemberType1, arg_2: MyMemberType2) -> MyStruct {
        // 创建结构体对象：方法一：最直接的方式，逐个赋值
        MyStruct {
            member_1: arg_1,
            member_2: arg_2,
        }
    }
}

fn main() {
    // 创建结构体对象：方法二：调用函数间接创建
    let my_struct_obj_1 = MyStruct::new(arg_1, arg_2);
    // 创建结构体对象：方法三：利用已有对象，为部分成员重新赋值，创建新对象
    // 对于未重新赋值的成员，新旧对象的对应成员相等
    let my_struct_obj_2 = MyStruct {
        member_2: arg_2,
        ..my_struct_obj_1
    };
}
```

注意：
1. Rust不采用类的概念，而是遵循组合大于继承的原则。<br>因此，Rust只有结构体，没有类，且结构体不可继承。<br>至于“组合”的部分，即特征（Trait），之后的部分会提及。

## 三、Rust特色部分
以下部分就是带有纯正Rust风味的用法了，只有这样，你才能知道你用的是Rust。

### 语句与表达式
> course.rs版介绍：https://course.rs/basic/base-type/statement-expression.html

#### 语句块：作为表达式
```rust
let distance_spuare = {
    let distance_x = 3;
    let distance_y = 3;
    distance_x * distance_x + distance_y * distance_y
};
```

#### 函数：直接返回表达式，无需return语句
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
当当当当！

#### If分支语句：作为表达式
```rust
let bmi = 20.0;
let bmi_score = if bmi > 32.0 {
    60
}
else if bmi > 28.0 {
    80
}
else if bmi < 16.0 {
    80
}
else {
    100
};
```
要求：
1. 必须包含else分支。
2. 所有分支中，表达式的值的类型相同。

#### Match分支语句：作为表达式
```rust
let num = 1;
let num_tips = match num {
    1 => "One!".to_string(),
    2 => "Two!".to_string(),
    other_num => fotmat!("Other num: {}", other_num),
}
// 这几个都是String类型的
// 这里的String是啥？后边讲字符串的时候再说
```
要求：
1. 所有分支中，表达式的值的类型相同。

#### Loop循环语句：作为表达式
```rust
let mut data = 1;
let ret = loop {
    if data >= 1000 {
        break data;
    }
    data <<= 1;
}
```
要求：
1. 针对同一个loop所有break语句后，表达式的值的类型相同。

### 所有权
原帖：https://course.rs/basic/ownership/ownership.html

我们可以把所有的Rust类型分为两类：
1. 第一类：实现了`Copy`特征的。
2. 第二类：没实现`Copy`特征的。

> `Copy`特征：<br>
> 表示该类型的对象可以以很低的性能开销，在栈上进行复制。<br>
> 所有的基本类型都实现了该特征。

对于第一类，随便取值赋值，不用管所有权。重点在第二类。

TODO

### 引用与借用
原帖：https://course.rs/basic/ownership/borrowing.html

TODO
