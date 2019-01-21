package main

import (
  "fmt"
)


type myString string

// interface derfination, contains the method to be implemented
type VowelsFinder interface {
  FindVowels() []rune
}

const VOWELS = "aeiouAEIOU"

// myString implement the VowelsFinder interface
func (s myString) FindVowels() []rune {
  var vowels []rune
  for _, char := range s {
    for _, target := range VOWELS {
      if char == target {
        vowels = append(vowels, char)
      }
    }
  }

  return vowels
}

func main() {
  var s myString = "Just a test, and there is vowels, BELIEVE U."
  var v VowelsFinder = s
  vowels := s.FindVowels()   // 定义了基于myString类型的method，所以可以通过该变量调用
  fmt.Println(s)
  fmt.Printf("%c ", vowels)
  fmt.Printf("% c", v.FindVowels())    // 也可以使用基于接口的对象调用，所以并不完全符合OOP
}
