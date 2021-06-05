# go

## Vscode 安装 go 插件失败

挂代理, 设置 go -env

## go: cannot find main module

项目根目录下没有 go mod

## 不在 gopath 下写代码不让跑测试

[gopath · Issue #1441 ](https://github.com/golang/dep/issues/1441)
这 issue 被人关了,看来不是只有我自己被这问题困扰.
最后需要在 target-path init 一下,就可以正常跑通 hello.go 的测试了

```bash
go mod init github.com/kscarrot/tdd-lang/go
```

## gopls 会爆红

提示去[这找解决方案](https://github.com/golang/tools/blob/master/gopls/doc/workspace.md)

这是因为目标工作区没有设置在 go 的根目录,gopls 会误认为你在搞`Multiple modules`

如果真要搞的话,给了一个实验性的解决方案,即在 vscode 添加一个 setting 项目

```json
"gopls": {
  "experimentalWorkspaceModule": true,
}
```

或者最简单的切换一下工作区,把写 go.mod 的地方变成 vscode 的 workspace.

## 函数问题

1. 函数名后面接 , 形参 space type ,用括号包裹,用逗号间隔.

2. 返回值也是入参,并且当 return 不接东西的时候可以做到裸返回,返回对应返回变量的值.(注意裸返回有块级作用域).

   > 感觉复杂类型 named return value 有点扯淡,不过有 defer 不好说.

3. 返回值可以用 \_ 解构元祖的时候用占位顶掉,不污染命名空间

4. 返回函数/表达式会先把栈锤平

5. 函数参数不能给默认值,为的是调用时显式声明,不给隐藏行为(config 之类的这种需求都需要显式传,好事儿)

## pakage 问题

新建了另一个 go 文件,标记为不同的包,报错
因为每个目录中的 Go 源文件只能属于同一个包（package），确保你的文件在组织上是分离的.
[相关参考](https://dave.cheney.net/2014/12/01/five-suggestions-for-setting-up-a-go-project)

## 单元测试的问题

> 单测在 vscode 里可是直接点击执行,很友好

fmt 打印的可以用注释进行单元测试

```go
func ExampleAdd() {
  sum := Add(1, 5)
  fmt.Println(sum)
  // Output: 6
}
```

删除` // Output: 6` ,run `go test -v`测试函数不会被执行
添加以后 Output 会和 print 做比较,此时是会执行测试的.把 6 换成 5 会测试失败.Hack 行为.

## 迭代的问题

go 没有 while do util,只有 for 或者说 loop

for 循环条件框没有括号(不像一个 fn)
变量可以用 := 声明初始化变量(替代 var,更多的用于 err)

可以用 testing.B 来运行基准测试,会根据 cpu 信息跑出性能.
用 cover 可以生成测试覆盖率

```bash
go test -bench=.
go test -cover
```

## interface 的问题

interface 里只允许有方法,这个方法的形参就是实现了这个 interface 的类型.实现了 interface 的类型,就可以调用 interface 定义的方法(Duck Type).

```go
func(s Struct) Method() type {
    //do something
}
```

`interface{ }`也是一个类型,因为他没有方法,所以所有类型都实现了它.
即一个类型定义为`interface{ }`,这个类型就类似于`any`,但是他的类型不是 any,是`interface{ }`.同样的,我们可以定义一个 类似`any[]`的东西,like`vals []interface{}`,注意,这个 vals 不是一个 slice,就像 js 函数参数不是一个数组,需要手动转换.

另外,上述函数的参数是一个结构类型,形参默认为类型首字母小写,被称作`receiver`,可以理解为`class`里的 this.但是`receiver`可以传值也可以传引用,要注意区分.

## 其他 feature

- 没有枚举
- 没有泛型 (手写多类型 bind,或者 interface{})
- 没有模式匹配
- 没有构造函数
