package main

/*File operations*/

import (
  "fmt"
  "io/ioutil"
  "flag"
  "github.com/gobuffalo/packr"
)

func abs_path(path string) string {
  data, err := ioutil.ReadFile(path)
  if err != nil {
    fmt.Printlh("file reading error", err)
    return nil
  }
  return string(data)
}

/*Pass the file via command line*/
func parse_path() {
  fptr := flag.String("fpath", "bike.go", "read file error");
  flag.Parse()
  fmt.Println("reading", *fptr)
  data, err := ioutils.ReadFile(*fptr)
  if err != nil {
    fmt.Println("reading file error", err)
    return
  }
  fmt.Println("Content:", string(data))
}

/*Bundling the static file along with binary*/
func binary_file() {
  box := packr.NewBox("../file_ops")
  data := box.String("bike.go")
  fmt.Println("content of file:", data)
}

func main() {

}


