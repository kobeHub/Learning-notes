# 文件操作

[TOC]

## 1. 文件读取

基本的文件操作，包含使用绝对路径或者相对路径，将整个文件直接读入内存。通过命令各行参数进行读取，将文件绑定到二进制文件中进行读取。

+ 可以将文件读出为比特流，然后再转化为相应的格式，

```go
func ReadFile(dirname string) ([]byte, error) 
```

```go
func abs_path(path string) string {
  data, err := ioutil.ReadFile(path)
  if err != nil {
    fmt.Println("file reading error", err)
    return ""
  }
  return string(data)
}
```

+ 使用`flag`包在命令行传入参数，使用`String`方法传入一个标志

  ```go
  func String(name string, value string, usage string) *string
      String defines a string flag with specified name, default value, and usage
      string. The return value is the address of a string variable that stores the
      value of the flag.
  ```

  参数使用`-`进行传入，可以使用等号赋值的方式，也可以使用空格方式。注意定义所有的参数之后需要使用`flag.Parrse()`方法将所有的参数进行转化，然后才可以使用传入的参数。如果传入没有定义的参数，会输出相应的usage.

  ```shell
  filehadling -fpath=../bike.go -chunk ../bike.go -line ../bike.go
  ```

+ 将文件按照小块读入

  可以使用`os.Open()`函数打开一个文件描述符，然后使用`bufio.NewReader(rd io.Reader)`函数返回一个`bufio.Reader`对象，使用该对象进行相应的文件读取。**注意返回值均为指针类型**。`io.Reader`是一个接口类型，凡是实现了`Read(p []byte)(n int, err error)`方法的类型都是该接口类型。`os.File`类型实现了该接口，所以可以将一个文件描述符传入`bufio.NewReader()`中.

  ```go
  /*read file into small chunks*/
  func small_chunks(fptr *string) {
  
    f, err := os.Open(*fptr)
    if err != nil {
      log.Fatal(err)
    }
    defer func() {
      if err = f.Close(); err != nil {
        log.Fatal(err)
      }
    }()
    r := bufio.NewReader(f)
    b := make([]byte, 3)
    for {
      _, err := r.Read(b)
      if err != nil {
        fmt.Println("Error reading file", err)
        break
      }
      fmt.Println(string(b))
    }
  }
  ```

+ 按行读入文件

  `Scanner`提供了在一个文件中按行读取的的方便的接口。使用`Scan`方法将文件扫描符，移到下一个token，当扫描结束时返回false，否则返回true。

  ```go
  func read_lines(fptr *string) {
  
    f, err := os.Open(*fptr)
    if err != nil {
      log.Fatal(err)
    }
    defer func() {
      if err = f.Close(); err != nil {
        log.Fatal(err)
      }
    }()
    s := bufio.NewScanner(f)
    for s.Scan() {
      fmt.Println(s.Text())
    }
    if err = s.Err(); err != nil {
      log.Fatal(err)
    }
  }
  ```

## 2. 文件写入

