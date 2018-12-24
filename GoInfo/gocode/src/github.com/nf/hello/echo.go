package main

import (
  "os";
  "flag";
  Str "github.com/nf/string"
)

var omitNewline = flag.Bool("n", false, "Don't print final newline")

const (
  Space = " "
  Newline = "\n"
)

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
}
