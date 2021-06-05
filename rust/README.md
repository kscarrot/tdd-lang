# rust
## mutable
变量用`let`声明,默认immutable,声明可变变量要用`mut`进行标注.常量用`const`声明,总是不可变,且常量可以覆盖整个生命周期.
```rust
let mut x = 5;
```

重复声明同名的变量,后者会将前者覆盖,称之为`隐藏`.可以多次隐藏.隐藏实际上是声明了一个新的变量覆盖了旧的名字,因此可以改变变量的类型.

## type
* 标量Scalar类型
  四种: 整形,浮点,布尔,字符.
  整形中isize和usize主要作为某些集合的索引,默认i32
  浮点ieee754.默认f64
  字符: 字符是单引号,字符串是双引号.
  一个char4个字节,并代表了一个unicode值.

* 复合类型
  元组 tuple
  用圆括号声明,长度固定,类型固定但是不必相同用 match 和 结构来取值.
  也可以用`.`号+索引直接访问.

数组 array
类型必须相同,T[],长度固定,变长需要用vector.初始化可以 类型;长度.
```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5]; //类似fill(3)
```

## 所有权
String可变,字面值字符串不可变

## 字符串
字符串合并只能
```rust
let s1 = String::from("Hello, "); let s2 = String::from("world!"); let s3 = s1 + &s2;
```

`+`的签名类似`fn add(self, s: &str) -> String {`

`String`是`Vue<u8>`的封装;


## 测试
cargo单元测试默认不显示输出信息,如果需要显示要加上参数
```bash
cargo test -- --nocapture
```