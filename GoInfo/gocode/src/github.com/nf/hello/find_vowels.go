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
  vowels := s.FindVowels()
  fmt.Println(s)
  fmt.Printf("%c ", vowels)
}
