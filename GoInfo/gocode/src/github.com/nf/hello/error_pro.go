package main

/*Usage of error processing in go*/
import (
	"fmt"
	"net"
	"os"
	"path/filepath"
)

// interface to present the Error type
type error interface {
	Error() string
}

// use the type asserting to get more information
func open_error() {
	f, err := os.Open("tet")
	if err, ok := err.(*os.PathError); ok {
		fmt.Printf("Err:%v, type:%T, path: %v\n", err, err, err.Path)
		return
	}
	fmt.Println("Open succeccfully", f.Name())
}

//asserting the underlying struct type and get more information
func dns_test() {
	addr, err := net.LookupHost("google.com")
	if err, ok := err.(*net.DNSError); ok {
		if err.Timeout() {
			fmt.Println("operation time out")
		} else if err.Temporary() {
			fmt.Println("temporary error")
		} else {
			fmt.Println("generic error:", err)
		}
		return
	}
	fmt.Println(addr)
}

// get error message by direct comprasion
func direct() {
	files, error := filepath.Glob("[")
	if error != nil && error == filepath.ErrBadPattern {
		fmt.Println(error)
		return
	}
	fmt.Println("File matched:", files)
}

func main() {
	open_error()
	dns_test()
	direct()
}
