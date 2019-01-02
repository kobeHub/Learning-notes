package main

import "fmt"

func map_test() {
  var personAge map[string]int
  if personAge == nil {
    fmt.Println("The map is nil, not create one")
    personAge = make(map[string]int)
  }

  // Add ele
  personAge["Make"] = 21
  personAge["Inno"] = 21
  personAge["Jack"] = 23
  fmt.Println("The map:", personAge)

  binaryMap := map[float32]string {
    0.123: "First",
    3.124: "Second",
    5.7667: "Nothing",
  }
  for i, v := range binaryMap {
    fmt.Printf("binary[%f] = %s\n", i, v)
  }

  person := "Tim"
  value, ok := personAge[person]
  if ok {
    fmt.Println("Age of", person, value)
  } else {
    fmt.Println("There is no one named Tim")
  }

  delete(binaryMap, 0.123)
  fmt.Println("Delete the 0.123", binaryMap, "Length:", len(binaryMap))
}

func equal(a, b map[string]int) bool {
  if len(a) == len(b) {
    for key, value := range a {
      got, ok := b[key]
      if ok {
        if got != value {
          return false
        }
      } else {
        return false
      }
    }
  } else {
    return false
  }

  return true
}

func main() {
  map_test()
  a := map[string]int {
    "test": 1,
    "go": 2,
    "just": 3,
  }
  c := map[string]int {
    "just": 3,
    "go": 2,
    "test": 1,
  }
  fmt.Println(equal(a, c))

  no := make(map[complex128]int)
  fmt.Println(no, no == nil, len(no))

}
