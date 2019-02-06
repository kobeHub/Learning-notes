package main

import (
  "fmt"
  "os"
  "bufio"
)

//write string into files
func write_file(fpath, content string) {
  f, err := os.Create(fpath)
  if err != nil {
    fmt.Println(err)
    return
  }

  defer func() {
    err := f.Close()
    if err != nil {
      fmt.Println(err)
    }
  }()

  l, err := f.WriteString(content)
  if err != nil {
    fmt.Println(err)
    return
  }
  fmt.Println(l, "bytes written successfully!")
}

// write bytes into files
func write_byte(file string, content []byte) {
  f, err := os.Create(file)
  if err != nil {
    fmt.Println(err)
    return
  }

  defer func() {
    err := f.Close()
    if err != nil {
      fmt.Println(err)
      return
    }
  }()

  num, err := f.Write(content)
  fmt.Println(num, "bytes written into the file", file)
}

func append_string(file string, content string) {
  f, err := os.OpenFile(file, os.O_APPEND|os.O_WRONLY, 0644)
  if err != nil {
    fmt.Println(err)
    return
  }
  defer func() {
    err := f.Close()
    if err != nil {
      fmt.Println(err)
      return
    }
  }()

  if _, err = fmt.Fprintln(f, content); err != nil {
    fmt.Println(err)
    return
  }
  fmt.Println("file append successfully!")
}


func main() {
  var _ = func() {
    var file, content string
  fmt.Println("Enter the file path:")
  fmt.Scanln(&file)
  fmt.Println("The content to be written with a newline \\quit to quit:")

  inputReader := bufio.NewReader(os.Stdin)

  for {
    input, err := inputReader.ReadString('\n')
    if err != nil {
      fmt.Println(err)
      return
    }
    // Caution the break condition, the newline will be read in.
    if input == "\\quit\n" {
      break
    } else {
      content += input
    }
  }

  write_file(file, content)}
  data := []byte{104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100}
  write_byte("byte", data)
  append_string("byte", "Append somthing")
}
