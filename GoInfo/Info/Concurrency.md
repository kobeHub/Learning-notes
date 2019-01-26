# 并发

[TOC]

## 1. 并发与并行

Golangz是一个典型的并发型语言**（Concurrency）**,而不是一个并行语言**(Parallelism)**。并发与并行的概念有着本质上的差异。

### 1.1 Concurrency

并发是指同时处理多个任务的能力。和操作系统多任务的概念相似，可以处理多种任务。可以使用一个现实中的例子：一个人正在慢跑，发现鞋带开了，现在他可以停止跑步，系上鞋带后继续跑下去。这个对象可以处理多个事件。这就是并发。

例如一个浏览器具有很多component，每一个部分可以处理一件任务，而且都可以独立执行。现在假设有两个部分，分别用于处理网页以及文件下载。当该浏览器运行在一个单核处理器上时，处理器在执行过程中会不断地进行上下文切换，来让两个任务同时进行。这就是并发，并发任务在不同的时间点开始，处理过程可以重叠。

![concurrency](http://media.innohub.top/190125-con.png)



### 1.2 Parallelism

并行是指同时做很多事情，例如一个人可以边跑步边听音乐。通常使用多进程实现。

对于以上的浏览器的例子，若果运行在多核处理器上，每一个任务分别在一个核上单独进行，不同任务间需要通信进行同步，这就是并行。

并行操作并不一定会有更好的运行时间，因为不同核间的通信需要大量的时间。在并发系统中，通信的开销很小，相比于并行系统的高通信开销，有时并发系统的性能更好。

## 2. Goroutines

在Golang中，实现并发的基本方式是`Goroutines`和`channel`配合使用。Goroutines用于创建更加轻量级的线程，channel用于goroutines间的通信。

Goroutines是指函数和方法与其它函数和方法并发执行的过程。用于创建Goroutine的开销远小于创建thread，因此对于一个go程序而言，上千个Goroutines并发执行是很常见的。

### 2.1 使用Goroutines的原因

+ 相比于线程而言，更加轻量级。在堆中只有即kb的大小，而且堆可以根据需要适当的增大或者缩小。
+ Goroutines被多路复用到操作系统中几个少数的线程上。可能在一个线程上由上千个Goroutines,如果说任何一个属于该线程的Goutines因为用户输入而阻塞，那么就创建另一个线程，将其他Goroutines准一道新的线程上去。这些操作都是在运行时完成的，不需要程序员参与。
+ 使用channel来避免访问共享内存是的竞争条件的发生

### 2.2 Goroutine 基本用法

在一个函数前使用关键字`go`,即并发执行了该函数。相当于在调用该函数时，创建了一个Goroutine，并且执行该函数。**Goroutine创建后立刻返回**，也就是说，不需要等待并发函数返回，就继续向下执行。

```go
package main

import (  
    "fmt"
)

func hello() {  
    fmt.Println("Hello world goroutine")
}
func main() {  
    go hello()
    fmt.Println("main function")
}

```

对于以上代码，只会输出`main function`。这是因为每一个Goroutine都是依赖于main Goroutine的，当main函数执行完毕，`main Goroutine`的生命周期结束，那么没有执行完的`Hello() Goroutine`也不会继续执行。可以使用`time.Sleep(t time.Duration)`，使得主routine等待其他返回。

## 3. Channels

Channel可以被认为是管道pipe，用于进行Goroutine间的通信。对于一个channel而言可以向指定方向发送数据，也可以接受指定的数据。同时注意channel中传输的数据类型必须和其定义使用的数据类型一致。声明一个隧道需要使用`chan`关键字,并且后面跟上传输的数据类型。

```go
var a chan int
if a == nil {
    a = make(chan int)
    fmt.Printf("Tyoe of a:%T\n", a)
}

// Type of a is chan int 
```

以上代码`make(chan type, size)`用以初始化一个channel，如果没有size参数，那么就没有缓冲空间。

使用channels发送和接收数据是**阻塞(blocked)**的，也就是说如果一个发送方已经发送，没有接收方接收时，该channel就一直被该发送发方占有。接收方阻塞与之相同。这种机制使得使用channels避免了复杂的锁机制，或者条件变量等。这也是go的一大特色。

```go
func Hello(a chan bool) {
  fmt.Println("In the Hello function")
  a <- true
}
fmt.Println("Main initialized!")
func main() {
  a := make(chan bool)
  go Hello(a)

  if <-a {
    fmt.Println("The hello run successfully")
  } else {
    fmt.Println("Hello error")
  }
}
// Main initialized!
// In the Hello function
// The hello run successfully
```

对于channel中的数据使用`<-`操作符进行读写，如果`chan`对象位于操作符前，数据位于其后，则是将数据写入，否则是读出。使用channel配合Goroutine可以进行不同routine间的协同工作。同时需要注意其阻塞机制，使用channel传输数据是阻塞的。

```go
package main

import (
  "fmt"
)

// go routine to compute cube
func ComCubes(num int, cubeop chan int) {
  sum := 0
  for num != 0 {
    digit := num % 10
    sum += digit * digit * digit
    num /= 10
  }
  cubeop <- sum
}

// go routine to compute square
func ComSquare(num int, squareop chan int) {
  sum := 0
  for num != 0 {
    digit := num % 10
    sum += digit * digit
    num /= 10
  }
  squareop <- sum
}

func main () {
  //test for compute square + cube
  num := 123
  squc := make(chan int)
  cubech := make(chan int)
  go ComCubes(num, cubech)
  go ComSquare(num, squc)

  squares, cubes := <-squc, <-cubech
  fmt.Println("Final output:", squares + cubes)
}
```

使用并发程序，创建两个routines,然后分别计算平方和、立方合，最后主程序等待两个程序返回后，再计算最终值。

## 4. 死锁

只要涉及到进程同步的概念，就避不开死锁。使用Goroutines与channels与普通的进程间通信机制一样都会出现死锁。

```go
package main


func main() {  
    ch := make(chan int)
    ch <- 5
}

atal error: all goroutines are asleep - deadlock!

goroutine 1 [chan send]:  
main.main()  
    /tmp/sandbox249677995/main.go:6 +0x80
```

以上代码设置了一个channel，并且送入数据，由于没有接受者，那么程序会一直阻塞下去。编译器会检测死锁并报错。

### 4.1 单方向的channel

那么如果需要建立一个只发送数据的channel该如何是好呢？可以使用单方向的channel，声明时采用`make(chan<- type)`

```go
./channel.go:14:15: invalid operation: <-chanl (receive from send-only type chan<- int)
```

但是如果在定义时就把一个channel定义为单向的，不可以读出，也就没有其存在的意义了。所以需要将其定义为双向的channel，然后可以在某些routines中隐式转换为单向的.

````go
package main

import "fmt"

func sendData(sendch chan<- int) {
  sendch <- 10
  fmt.Printf("Inner routine: %T\n", sendch)
}

func main() {
  chanl := make(chan int)
  go sendData(chanl)
  fmt.Printf("%v, %T\n", <-chanl, chanl)
}


Inner routine: chan<- int
10, chan int
````

### 4.2 channel的关闭

对于使用`for`循环向隧道发送或者接收数据，发送发可以主动关闭隧道，接收方可以根据接受的状态判断隧道是否已经关闭。对于接收方每次读一个数据，可以返回两个值

```go
v, ok := <- ch
```

`ok`存放了数据读取的状态，如果为false则表示该隧道已经关闭，接收方也会停止接受数据，此时`v`的值变为隧道传输数据类型的默认值。

```go
package main

import (  
    "fmt"
)

func producer(chnl chan int) {  
    for i := 0; i < 10; i++ {
        chnl <- i
    }
    close(chnl)
}
func main() {  
    ch := make(chan int)
    go producer(ch)
    for {
        v, ok := <-ch
        if ok == false {
            break
        }
        fmt.Println("Received ", v, ok)
    }
}
```

使用接收状态和for循环，可以优化前面的`square.go`代码.

```go
package main

import (
  "fmt"
)

// extract common digit operation from the two function
func DigitOps(num int, digitch chan int) {
  for num != 0 {
    digit := num % 10
    num /= 10
    digitch <- digit
  }
  close(digitch)
}

// go routine to compute cube
func ComCubes(num int, cubeop chan int) {
  sum := 0
  digitch := make(chan int)
  go DigitOps(num, digitch)

  for digit := range digitch {
    sum += digit * digit * digit
  }
  cubeop <- sum
}

// go routine to compute square
func ComSquare(num int, squareop chan int) {
  sum := 0
  sch := make(chan int)
  go DigitOps(num, sch)

  for d := range sch {
    sum += d * d
  }
  squareop <- sum
}

func main () {
  //test for compute square + cube
  num := 123
  squc := make(chan int)
  cubech := make(chan int)
  go ComCubes(num, cubech)
  go ComSquare(num, squc)

  squares, cubes := <-squc, <-cubech
  fmt.Println("Final output:", squares + cubes)
}
```

使用Goroutines的主要思想在于将不同函数的共同部分，提取出来，创建一个新的routines，并且执行。实现并发的操作。使用`close（ch chan type）`,用已关闭一个channel，对于一个channel可以配合`range`使用，获取该channel中所有的数据。

## 5. 带有缓冲区的channel

可以定义带有缓冲区的channel由于对于传输的数据进行缓冲，由于缓冲区为0的channel传输数据是阻塞的。对于有缓冲区的的channel，如果缓冲区满则发送方阻塞，如果缓冲区为空，则接收方阻塞。

```go
package main

import (  
    "fmt"
    "time"
)

func write(ch chan int) {  
    for i := 0; i < 5; i++ {
        ch <- i
        fmt.Println("successfully wrote", i, "to ch")
    }
    close(ch)
}
func main() {  
    ch := make(chan int, 2)
    go write(ch)
    // time.Sleep(2 * time.Second)
    for v := range ch {
        fmt.Println("read value", v,"from ch")
        time.Sleep(2 * time.Second)

    }
}

/*
output：
successfully wrote 0 to ch  
successfully wrote 1 to ch  
read value 0 from ch  
successfully wrote 2 to ch  
read value 1 from ch  
successfully wrote 3 to ch  
read value 2 from ch  
successfully wrote 4 to ch  
read value 3 from ch  
read value 4 from ch  
*/
```

以上代码演示了发送方向一个缓冲区大小为2的channel发送数据，接收方每隔2s接受一个数据。刚开始缓冲区为空，所以此时接收方阻塞，发送方可以写入数据，写入两个后，发送方阻塞，伺候接收方每取走一个数据，发送发可以再发送一个数据。如果将`write()`函数中的`close（）`注释掉，那么就会造成死锁。因为接收方一直在等待接收数据而且channel也没有关闭。

### 5.1 len and cap

与切片中的概念一样，`capacity`代表了缓冲区的大小，是可以缓冲数据的最大数量。`length`代表了当前缓冲队列的长度。