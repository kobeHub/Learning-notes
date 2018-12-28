package main

import (
  "fmt"
//  "bufio"
//  "os"
//  "strconv"
)


func Permutate(list []int, begin, end int) {
  if begin == end {
    for i := 0; i <= end; i++ {
      fmt.Printf("%v ", list[i]);
    }
    fmt.Println();
  } else {
    for i := begin; i <= end; i++ {
      list[begin], list[i] = list[i], list[begin]
      Permutate(list, begin+1, end)
      list[begin], list[i] = list[i], list[begin]
    }
  }
}


func AllSubsets(list, mark []int, begin, end int) {
  if begin == end {
    fmt.Printf("{")
    for i := 0; i < end; i++ {
      if mark[i] == 1 {
        fmt.Printf("%v, ", list[i])
      }
    }
    fmt.Println("}")
    return
  }
  mark[begin] = 0
  AllSubsets(list, mark, begin+1, end)
  mark[begin] = 1
  AllSubsets(list, mark, begin+1, end)
}


func main() {
  /*
  var list = [3]int{1, 2, 3}
  mark := [3]int{}
  */
  var (
    num int
  )
  //inputReader = bufio.NewReader(os.Stdin)
  fmt.Println("The length of array:")
  fmt.Scanln(&num)
  fmt.Println("The int element:")
  list := make([]int, num)
  mark := make([]int, num)
  for i := range list {
    fmt.Scan(&list[i])
  }

  fmt.Println("Permutation:")
  Permutate(list, 0, 2)
  fmt.Println("All subsets:")
  AllSubsets(list, mark, 0, 3)
}
