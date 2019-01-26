# Work Pools

[TOC]

## 1. Wait Group

等待组是一个用以维护Groutines的一个集合，用以等待其中所有的routines执行结束并且返回。对于wait group的控制是阻塞的，当存在routine没有执行结束。对于单个routine的等待可以按照前面的方法，使用channel实现，对于多个routines，可以使用`sync.WaitGroup`结构体得到更为简洁的实现。假设现在有三个并发执行的Goroutines从`main routine`催生出来：

+ 使用`wg.Done()`结束一个routine的等待
+ 使用`wg.add(a int)`增加需要等待的routine数量
+ 在主进程使用`wg.wait()`,等待所有的routine返回
+ 每一个routine都需要使用`WaitGroup`的指针形式作为参数

```go
/*Simple program to use WaitGroup*/
package main

import (
  "fmt"
  "sync"
  "time"
)

// The routine spawned from main routine
func process(i int, wg *sync.WaitGroup) {
  fmt.Println("started Goroutine", i)
  time.Sleep(1 * time.Second)
  fmt.Println("Goroutine", i, "done.")
  wg.Done()
}

func main() {
  no := 5
  var wg sync.WaitGroup
  for i := 0; i < no; i++ {
    wg.Add(1)
    go process(i, &wg)
  }
  wg.Wait()
  fmt.Println("All the routines finished")
}



/*started Goroutine 4
started Goroutine 2
started Goroutine 3
started Goroutine 0
started Goroutine 1
Goroutine 0 done.
Goroutine 4 done.
Goroutine 1 done.
Goroutine 3 done.
Goroutine 2 done.
All the routines finished
*/
```

`WaitGroup`结构体对象等待一个routines执行结束集合返回，维护了一个计数器，调用`Add()`就增加计数器的数值，调用`Done()`就将该计数值减一，`Wait()`方法阻塞该对象直到计数器为0。由于先后开启了5个routines，输出时可能比较混乱，但是等待相同时间后，输出就按照顺序。

注意使用WaitGroup时，每一个routine必须传入其指针形式的参数，按值传递的话，进行了复制，主routine无法获得该对象的信息。

## 2. Worker Pool implementation

带有缓存区的channel的一个重要使用场景是实现线程池（Threads pool, work pool）.

### 2.1 线程池

线程池是操作系统或者计算机程序中用来维护多个线程，这些线程等待执行不同的任务，从而实现程序的并发操作。线程池由控制系统进行管理，主要负责线程数量的确定，线程的创建以及销毁。通过维护一个已经存在的线程池，可以减少线程创建以及销毁的开销，相比于每有一个任务就创建一个线程，结束即销毁的方式由很大优势。

为线程池调度任务的一个常用方法是维护一个同步队列。线程池中的线程移除任务队列中的任务，执行完毕后将其加入完成队列.一旦一个进程完成了任务就可以等待下一个任务。

![thread pool](https://upload.wikimedia.org/wikipedia/commons/0/0c/Thread_pool.svg)

我们需要实现的worker pool所需要的主要功能：

+ 创建一个Goroutines pool，用以监听一个有缓冲区的channel作为工作集的输入
+ 当一个任务结束后，将结果写出到一个buffered channel
+ 可以从output channel读取结果

