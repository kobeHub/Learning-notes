package main

/*Simple use case for reflect*/

import (
  "fmt"
  "reflect"
)

// The order contains id, consumer
type order struct {
  ordId int
  customerId int
}

// employee struct
type employee struct {
  name string
  id int
  address string
  salary int
  country string
}

// create sql statements according to the struct
func createQuery(i interface{}) {
  if reflect.ValueOf(i).Kind() == reflect.Struct {
    v := reflect.ValueOf(i)
    /*
    fmt.Println("Number od fields:", v.NumField())
    for i := 0; i < v.NumField(); i++ {
      fmt.Printf("Field:%d, type:%T, %v, value:%v\n", i,
        v.Field(i), reflect.TypeOf(v), v.Field(i))
    }*/
    t := reflect.TypeOf(i).Name()
    query := fmt.Sprintf("insert into %s values(", t)
    for i := 0; i < v.NumField(); i++ {
      switch v.Field(i).Kind() {
      case reflect.Int:
        if i == 0 {
          query = fmt.Sprintf("%s%d", query, v.Field(i))
        } else {
          query = fmt.Sprintf("%s, %d", query, v.Field(i))
        }
      case reflect.String:
        if i == 0 {
            query = fmt.Sprintf("%s\"%s\"", query, v.Field(i))
          } else {
            query = fmt.Sprintf("%s, \"%s\"", query, v.Field(i))
          }
      default:
        fmt.Println("unsupported type")
      }
    }
    query = fmt.Sprintf("%s)", query)
    fmt.Println(query)
    return
  }
  fmt.Printf("unsupported type")
}

// display the basic usage of reflect
var display = func(i interface{}) {
  fmt.Println(i, reflect.TypeOf(i), reflect.ValueOf(i),
       reflect.ValueOf(i).Kind())
};

func reflect_test() {
  i1 := 12
  slice := []float64{12., 3.6, 67.}
  ord := order{
    188900998,
    678687989,
  }
  fmt.Println("Usage of TypeOf, ValueOf, ValueOf().Kind()")
  display(i1)
  display(slice)
  display(ord)
}

func main() {
  reflect_test()

  fmt.Println()
  emp1 := employee{
    "Kobe Bryant",
    215667868,
    "Los Angle",
    897868798,
    "America",
  }

  ord := order{
    45345345345,
    676876867876,
  }
  createQuery(emp1)
  createQuery(ord)
}

