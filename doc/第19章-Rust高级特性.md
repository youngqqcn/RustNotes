

## 第19章  Rust高级特性

本章内容:
- 不安全Rust :  用于当需要舍弃Rust的某些保证并手动维持这些保证
- 高级trait: 与trait相关的关联类型, 默认类型参数, 
    完全限定语法(fully qualified syntax), 
    超(父) trait (supertraits)和 newtype

- 高级类型: 关于newtype模式的更多内容, 类型别名, never类型和动态大小类型
- 高级函数和闭包: 函数指针和返回闭包
- 宏: 在编译时定义(展开)更多代码的方式



Rust 还隐藏有第二种语言，它不会强制执行这类内存安全保证：这被称为 不安全 Rust（unsafe Rust）

使用不安全代码好比告诉编译器，“相信我，我知道我在干什么。”这么做的缺点就是你只能靠自己了：
如果不安全代码出错了，比如解引用空指针，可能会导致不安全的内存使用。

另一个 Rust 存在不安全一面的原因是：底层计算机硬件固有的不安全性。

五个超能力:
- 解引用裸指针
- 调用不安全的函数或方法
- 访问或修改可变静态变量
- 实现不安全trait
- 访问union的字段



### 裸指针(raw pointers)

裸指针也分为 可变和不可变:  *const T  和 *mut T  ,   其中星号不是解引用运算符,它是类型的一部分.

裸指针与引用和智能指针的区别在于:

- 允许忽略借用规则, 可以同时拥有不可变和可变的指针, 或多个指向相同位置的可变指针
- 不保证指向有效的内存
- 允许为空
- 不能实现任何自动清理功能


裸指针的应用场景:
- 一个主要的应用场景便是调用 C 代码接口
- 另一个场景是构建借用检查器无法理解的安全抽象

使用 extern 函数调用外部代码

```rust

//使用 C 中的函数
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```



#### 访问或修改可变静态变量

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```


#### 实现不安全 trait


#### 访问联合体中的字段



### 高级trait

#### 关联类型(associate types)

关联类型(是一个而将类型占位符与trait相关联的方式, 这样trait的方法签名中就可以使用这些占位符类型.

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

为什么不适用泛型而使用关联类型?  因为关联类型只能实现一次, 泛型可以多次(不同)实现trait



#### 默认泛型类型参数和运算符重载


```rust
trait Add<RHS=Self> { //默认参数类型
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

```

运算符重载例子

```rust
use std::ops::{
    Add,
    Mul, 
};
use std::fmt;
// use std::fmt::{Display, Formatter};


#[derive(Debug)]
struct Complex {
    real: f64,
    image: f64,
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex{
        Complex {
            real: self.real + rhs.real,
            image: self.image + rhs.image,
        }
    }
}



#[derive(Debug)]
struct Matrix(Vec<Vec<i32>>);

impl Add for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Matrix) -> Matrix {
        assert_eq!(self.0.len(), rhs.0.len());

        let mut new_matrx = Vec::new();
        for row in 0..self.0.len() {
            let mut new_row = Vec::new();
            for col in 0..self.0[row].len() {
                new_row.push( self.0[row][col] + rhs.0[row][col] );
            }
            new_matrx.push(new_row);
        }
        Matrix(new_matrx)
    }

}


// 实现矩阵的乘法
impl Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        assert_eq!(self.0.len(), rhs.0[0].len()); 
        assert_eq!(self.0[0].len(), rhs.0.len());

        let a = &self.0;
        let b = &rhs.0;

        let mut new_matrx = Vec::new();
        for row in 0..a.len() {
            let mut tmp_row = Vec::new();
            for col in 0..b[0].len() {
                tmp_row.push(|| -> i32 {
                    let mut sum = 0;
                    for c in 0..b.len() {
                        sum += a[row][c] * b[c][col];
                    }
                    sum
                }());
            }
            new_matrx.push(tmp_row);
        }

        Matrix(new_matrx) 
    }
}



impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for row in 0..self.0.len() {
            s.push_str("| ");
            for col in 0..self.0[row].len() {
                s.push_str( &self.0[row][col].to_string()[..] );
                s.push_str(" ")
            }
            s.push_str("|\n");
        }
        s.push_str("\n");
        write!(f, "\n{}", s)
    }
}



fn main() {

    let c1 = Complex { real:5.1, image:-1.2 };
    let c2 = Complex { real:0.123 , image: 9.19};

    let c3 = c1 + c2;
    println!("c3 is {:?}", c3);

    let mtx1 = Matrix(vec![
        vec![1, 2, 3], 
        vec![1, 0, 2],
        vec![1, 0, 2],
    ]);

    let mtx2 = Matrix(vec![
        vec![0, 2, 3], 
        vec![1, 0, 2],
        vec![1, 1, 2],
    ]);

    // let mtx3 = mtx1 + mtx2;
    let mtx3 = mtx1 * mtx2;
    println!("mtx3 is {}", mtx3);


    let mtxa = Matrix(vec![
        vec![1, 3], 
        vec![5, 2],
        vec![1, 2],
    ]);

    let mtxb = Matrix(vec![
        vec![5, 2, 3], 
        vec![4, 1, 2],
        // vec![1, 1, 2],
    ]);

    // println!("mtxa * mtxb = {}", mtxa * mtxb);
    println!("mtxb * mtxa = {}", mtxb * mtxa);
}

```






#### 完全限定语法

如果多个作用域都有相同的方法, 可以通过加限定的方式指明调用哪个作用域中的方法

```rust

let person  = Human;
person.fly();
Pilot::fly(&person);
Wizard::fly(&person);

println!("a bady dog is called a {}", <Dog as Animal>::baby_name());

```




#### 父trait用于在另一个trait中使用某trait的功能

```rust
// 实现  supertrait
trait OutlinePrint: fmt::Display { // 相当于为 trait 增加 trait bound
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

```


newtype 模式用以在外部类型上实现外部trait


```rust


use std::fmt;

/*
只要 trait 或类型对于当前 crate 是本地的话就可以在此类型上实现该
 trait。一个绕开这个限制的方法是使用 newtype 模式（newtype pattern），它涉及到在一个元组结构体（第五章 “用没有命名字段的元组
结构体来创建不同的类型” 部分介绍了元组结构体）中创建一个新类型。
*/

// newtype 模式
// 没有运行时消耗
struct Wrapper(Vec<String>);


impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))  // 拼接 
    }
}

/*
// error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
impl fmt::Display for Vec<String> {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!(f, "hello world")  
    } 
}
*/


fn main() {

    let w = Wrapper(vec![
            String::from("hello"),
            String::from("world"),
            String::from("newboy"),
            ]);

    println!("w = {}", w);

}

```





### 高级类型

- 使用 newtype可以让类型名或参数类型可读性更好, 不容易混淆
- 隐藏实现细节
- 隐藏内部泛型类型



#### 类型别名

类似C中的 typedef 


```rust
type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;


// 引入类型别名 Thunk 来减少重复
type Thunk = Box<dyn Fn() + Send + 'static>;
fn takes_long_type(f: Thunk) {
    // --snip--
}



//类型别名也经常与 Result<T, E> 结合使用来减少重复
type Result<T> = std::result::Result<T, std::io::Error>;

```




#### 从不返回的never type
 
```rust
// bar 也称 发散函数（diverging functions）
fn bar() -> ! {
    // --snip--
}
```



```rust

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
//continue 的值是 !。也就是说，当 Rust 要计算 guess 的类型时，它查看这两个分支。前者是 u32 值，而后者是 ! 值。因为 ! 并没有一个值，Rust 决定 guess 的类型是 u32。

```



```rust
let guess = match guess.trim().parse() {
    Ok(_) => 5,
    Err(_) => "hello", // ERROR!!
}
```




#### 动态大小类型(DST  dynamically sized types) 和 Sized trait

规则: 必须将动态大小类型的值置于某种指针之后


#### 函数指针与闭包

函数指针

```rust


fn add_one(x: i32) -> i32 {
    x + 1
}


// 函数指针
type fnt = fn(i32) -> i32;

fn add_twice(fun: fnt, arg: i32 ) -> i32 {
    fun(arg) + fun(arg)
}


fn main() {

    let a = add_twice(add_one, 5);
    println!("a = {}", a);

}

```




#### 闭包作为函数返回值

```rust

// Fn(i32) -> i32
// doesn't have a size known at compile-time
// the trait `std::marker::Sized` is not implemented for `(dyn std::ops::Fn(i32) -> i32 + 'static)`
//fn return_closure() -> Fn(i32) -> i32 {
//    |x| x + 1
//}


fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new( |x| x + 1 )
}

fn main() {

    let cls = return_closure();

    println!("{}", cls(666));

}
```




### 宏

- 声明（Declarative）宏
- macro_rules!
- 3种过程(Procedural)宏
    - 自定义 `#[derive]` 宏在结构体和枚举上指定通过 derive 属性添加的代码
    - 类属性（Attribute）宏定义可用于任意项的自定义属性
    - 类函数宏看起来像函数不过作用于作为参数传递的 token



宏和函数的区别
- 宏在编译时期展开 
- 宏可以在给定类型上实现trait , 函数不行
- 宏的可读性不好
- 在调用宏 之前 必须定义并将其引入作用域，而函数则可以在任何地方定义和调用。



vec! 宏展开

```rust

#[macro_export]   // 可以被导入作用域
macro_rules! vec {
    ( $( $x:expr ),* ) => {  //单边模式, 模式匹配, 匹配成功则执行下面代码
    // $() 内则是 $x:expr ，其匹配 Rust 的任意表达式或给定 $x 名字的表达式。
    // 紧随逗号之后的 * 说明该模式匹配零个或多个 * 之前的任何模式
    // 以 vec![1, 2, 3]; 调用宏时，$x 模式与三个表达式 1、2 和 3 进行了三次匹配。
        { 
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

```

如果 `vec![1, 2, 3]` 展开之后:

```rust
let mut temp_vec = Vec::new();
temp_vec.push(1);
temp_vec.push(2);
temp_vec.push(3);
temp_vec

```


derive 只能用于结构体和枚举；


类属性宏

```rust

#[route(GET, "/")]
fn index() {
}

```


类函数宏

```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```
