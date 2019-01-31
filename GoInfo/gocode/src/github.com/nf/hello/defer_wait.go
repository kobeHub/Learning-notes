package main

/*Pratical usage cases of the defer statement */
import (
  "fmt"
  "sync"
  "time"
)

type rect struct {
  length, width float64
}

func (r rect) area(wg *sync.WaitGroup) {
  defer wg.Done()
  if r.width <= 0 {
    fmt.Println("The width of rect must be positive")
    return
  }
  if r.length <= 0 {
    fmt.Println("The length of rect must be positive")
    return
  }
  area := r.length * r.width
  fmt.Println("The area of rect:", area)
}

func main() {
  r1 := rect{12.0, -9.2}
  r2 := rect{11.2, 2}
  r3 := rect{11., -90.}
  rects := []rect{r1, r2, r3}
  var wg sync.WaitGroup
  for _, item := range rects {
    time.Sleep(1 * time.Second)
    wg.Add(1)
    go item.area(&wg)
  }
  wg.Wait()
  fmt.Println("All the routine finished")
}



