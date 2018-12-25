package main

import (
  "os";
  "flag";
  Str "github.com/nf/string";
  "time";
  "math/rand";
  "fmt"
)

var omitNewline = flag.Bool("n", false, "Don't print final newline")

const (
  Space = " "
  Newline = "\n"
)

func Random(i int)  {
  rand.Seed(time.Now().UnixNano())
  a := rand.Intn(i)
  fmt.Println(a)
}

func main() {
  flag.Parse()   // Scan the arg list and sets up flags
  var s string = ""
  for i := 0; i < flag.NArg(); i++ {
    if i > 0 {
      s += Space
    }
    s += flag.Arg(i)
  }

  if !*omitNewline {
    s += Newline
  }
  os.Stdout.WriteString(Str.Reverse(s))

  for i := 0; i < 5; i++ {
    Random(100)
  }
}
