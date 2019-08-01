package main

import (
	"fmt"
	"os"
	"os/signal"
	"syscall"
)

func main() {
	// go signal Notify 通过接收一个只读channel上的 `os.Signal`
	// 获取通知信号，用于进程间的通信
	sigs := make(chan os.Signal, 1)
	done := make(chan bool, 1)

	// 使用signal.Notify函数注册一个channel上可以接受的信号类型
	// 当接收到对应的信号时，写入该channel
	signal.Notify(sigs, syscall.SIGINT, syscall.SIGTERM)

	// Goroutines 执行一个阻塞调用，直到接收到系统信号，然后可以
	// 退出
	go func() {
		sig := <-sigs
		fmt.Println()
		fmt.Println(sig)
		done <- true
	}()

	fmt.Println("Waiting for signals...")
	<-done
	fmt.Println("exiting")

}
