package main

/*File operations*/

import (
  "fmt"
  "io/ioutil"
  "flag"
  "github.com/gobuffalo/packr"
  "os"
  "bufio"
  "log"
)

func abs_path(path string) string {
  data, err := ioutil.ReadFile(path)
  if err != nil {
    fmt.Println("file reading error", err)
    return ""
  }
  return string(data)
}

/*Pass the file via command line*/
func parse_path(fptr *string) {
  fmt.Println("reading", *fptr)
  data, err := ioutil.ReadFile(*fptr)
  if err != nil {
    fmt.Println("reading file error", err)
    return
  }
  fmt.Println("Content:", string(data))
}

/*Bundling the static file along with binary*/
func binary_file() {
  box := packr.NewBox("../filehadling")
  data := box.String("test.txt")
  fmt.Println("content of file:", data)
}

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

func main() {
  entire := flag.String("fpath", "/home/kobe/test.txt", "directly read entirety");
  line := flag.String("line", "test.txt", "line reading")
  chunks := flag.String("chunk", "test.txt", "read into small chunks")
  flag.Parse()

  con := abs_path("/home/kobe/test.txt")
  fmt.Println("Absolute path read:", con)

  parse_path(entire)
  binary_file()
  small_chunks(chunks)

  read_lines(line)
}


