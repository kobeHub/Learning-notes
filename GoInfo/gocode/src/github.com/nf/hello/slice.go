package main

import (
  "fmt"
  "reflect"
)

func clear(item []int) {
  item = item[:0]
  fmt.Printf("After clear:%v, %v, %v\n", item, len(item), cap(item));
}

func subtactOne(num []int) {
  for i := range num {
    num[i]--
  }
}

func main() {
  arr := [...]int{2, 23, 65, 67, 787, 54, 66}

  slice := arr[2:]
  fmt.Println("Array:", arr, "\nSlice", slice)
  for i := range slice {
    slice[i]++
  }

  fmt.Println("After change:", arr, slice)

  var test []int = []int{1, 2, 3}
  fmt.Println("\nOrigin slice:", test)
  subtactOne(test)
  fmt.Println("After process:", test)


  pls := [][][]string{
    {
      {"My", "programming"},
      {"Language"},
    },
    {
      {"c", "c++"},
      {"Python", "JavaScript"},
      {"Go", "Rust"},
    },
  }
  for _, v0 := range pls {
    for _, v1 := range v0 {
      for _, v2 := range v1 {
        fmt.Printf("%s ", v2)
      }
    fmt.Println()
  }

  fmt.Println(reflect.ValueOf(pls).Kind())
  }

  clear(arr[:])
  test1 := make([]byte, 12)
  fmt.Println("The length:", len(test1), cap(test1))
}
