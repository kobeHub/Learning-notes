# Select 语句使用

[TOC]

## 1. `select` 介绍

`select`语句用于从多个发送方/接收方的channel中选取需要接受的数据。`select`语句是阻塞的，直到一个case满足条件，或者定义了`default`语句。语法与`switch`相似。

使用`select`的主要场景可以是一个多服务器的服务。每一个服务器都提供相同的服务，并且具有重复的数据库，对于用户的一个请求，多个服务器都可以做出回应但是最终用户只接收到一个返回。因为负载以及网络延迟等，不同服务器做出反应的时间可能不同，此时选取反应最快的。以达到集群的最优用户体验。

```go
func server1(ch chan string) {
  time.Sleep(1 * time.Second)
  ch <- "from server 1"
}

func server2(ch chan string) {
  time.Sleep(3 * time.Second)
  ch <- "from server 2"
}

func main() {
  out1 := make(chan string)
  out2 := make(chan string)
  go server1(out1)
  go server2(out2)

  select {
  case s1 := <-out1:
    fmt.Println(s1)
  case s2 := <-out2:
    fmt.Println(s2)
  //default:
  //  fmt.Println(nil)
  }
}
//去掉注释，则返回 nil
```

### 1.1 default case

default case可以避免系统陷入阻塞。当没有一个case就绪时，默认执行default case。

```go
package main

import (  
    "fmt"
    "time"
)

func process(ch chan string) {  
    time.Sleep(10500 * time.Millisecond)
    ch <- "process successful"
}

func main() {  
    ch := make(chan string)
    go process(ch)
    for {
        time.Sleep(1000 * time.Millisecond)
        select {
        case v := <-ch:
            fmt.Println("received value: ", v)
            return
        default:
            fmt.Println("no value received")
        }
    }

}
```

在`process`routine中等待10.5s，在`main`routine中，每一秒检查一次是否有可以接受的channel数据。前10s都没有，所以输出default。接收到process的数据后，程序退出.

### 1.2 死锁以及default case

可以使用default case来避免死锁。例如对于一个一直等待读数据的channel，可以添加default解除死锁.

```go
package main

func main() {  
    ch := make(chan string)
    var ch2 chan int
    select {
    case <-ch:
    case <-ch2:
    default:
    	fmt.Println("default case executed")
    }
}
```

即使是一个没有经过初始化，为`nil`的channel，也可以解除其死锁。

**对于一个cases为空的select语句，会造成死锁，引起一直阻塞**

```go
package main

func main() {  
    select {}
}
fatal error: all goroutines are asleep - deadlock!

goroutine 1 [select (no cases)]:  
main.main()  
    /tmp/sandbox299546399/main.go:4 +0x20
```

### 1.3 随机选择

当一个`select`语句中的多个cases都满足条件，那么这些语句被随机选择执行。