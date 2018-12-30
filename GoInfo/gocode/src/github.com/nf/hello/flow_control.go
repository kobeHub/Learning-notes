package main

import "fmt"
import "reflect"

func Testflow(list []int) {
  fmt.Println("The basic for test:")
  for i, no := 10, 1; i <= 20 && no <= 10; i, no = i + 1, no + 1 {
    fmt.Printf("%d * %d = %d\n", i, no, i * no)
  }

  // Use for as while, all the ; can bo omitted
  i := 2
  for i < 10 {
    fmt.Printf("The %d out\n", i)    // Format output is only for Printf
    i += 2
  }

  fmt.Println("Print the slice:")
  fmt.Println("Type of the slice:", reflect.TypeOf(list), reflect.ValueOf(list).Kind())
  for i, v := range list {
    fmt.Println("list[", i, "]", v, reflect.TypeOf(v), reflect.ValueOf(v).Kind())  // Do not support format output
  }
  fmt.Println()

}

func Testswitch() {
  switch finger := 8; finger {
  case 1, 2, 3, 4, 5:
    fmt.Println("Normal finger")
  default:
    fmt.Println("Unnormal finger!")
  }

  fmt.Println("\nSwitch flow control:")
  num := 75
  switch { // expression is omitted
  case num >= 0 && num <= 50:
    fmt.Println("num is greater than 0 and less than 50")
  case num >= 51 && num <= 100:
    fmt.Println("num is greater than 51 and less than 100")
  case num >= 101:
    fmt.Println("num is greater than 100")
  }

  switch finger := 2; finger {//finger is declared in switch
    case 1:
        fmt.Println("Thumb")
	fallthrough
    case 2:
        fmt.Println("Index")
        fallthrough
    case 3:
        fmt.Println("Middle")
        fallthrough
    case 4:
        fmt.Println("Ring")
        fallthrough
    case 5:
        fmt.Println("Pinky")
        fallthrough
    default: //default case
        fmt.Println("incorrect finger number")
    }

}

func main() {
  var list []int =[]int{1, 2, 34, 65, 675}
  Testflow(list)
  Testswitch()
}
