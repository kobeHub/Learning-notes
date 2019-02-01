package main


/*Custom errorString usage*/
import (
  "fmt"
  //"errors"
  "math"
)

// areaError for `circleArea` calculation
type areaError struct {
  err string
  radius float64
}

// rectAreaError
type rectAreaError struct {
  err string
  length float64
  width float64
}

func (e *areaError) Error() string {
  return fmt.Sprintf("radius %0.2f: %s", e.radius, e.err)
}

func (r *rectAreaError) Error() string {
  return r.err
}

func (e *rectAreaError) lengthNegative() bool {
  return e.length < 0
}

func (e *rectAreaError) widthNegative() bool {
  return e.width < 0
}

// use the errors.New get a errorString error
func circleArea(radius float64) (float64, error) {
  if radius < 0 {
    return 0, &areaError{"is nagative", radius}
  }
  return math.Pi * radius * radius, nil
}

func rectArea(length, width float64) (float64, error) {
  var err string
  if length < 0 {
    err = "length is less than zero"
  }
  if width < 0 {
    if err == "" {
      err = "width is less than zero"
    } else {
      err += "width is less than zero"
    }
  }
  if err != "" {
    return 0, &rectAreaError{err, length, width}
  }

  return length * width, nil
}

// use the fmt.Errorf to get same function
func rectanglArea(leng, wid float64) (float64, error) {
  if leng < 0 {
    return 0, fmt.Errorf("Given length of rectangle %v is not correct", leng)
  }
  if wid < 0 {
    return 0, fmt.Errorf("Given width of rectangle %v is not correct", wid)
  }
  return leng * wid, nil
}

func main() {
  radius := -20.
  var holder error
  area, err := circleArea(radius)
  if err, ok := err.(*areaError); ok {
    holder = err     // 此处得到的err是一个指针类型，使用指针类型是为了避免不必要的参数复制
    fmt.Printf("%v, %T\n", holder, holder)
  } else {
    fmt.Println("Area of the circle:", area)
  }

  r1, err1 := rectanglArea(12., -6.)
  if err1 != nil {
    fmt.Println(err1)
    fmt.Printf("%T\n", err1)
  } else {
    fmt.Println("Area of rect1:", r1)
  }

  r2, err2 := rectanglArea(12, -2)
  if err2 != nil {
    fmt.Println(err2)
    fmt.Printf("%T\n", err2)
  } else {
    fmt.Printf("Area of rext2:\n", r2)
  }

  length, width := -9., -5.
  area, err = rectArea(length, width)
  if err != nil {
    if err, ok := err.(*rectAreaError); ok {
      // 属于该类型的错误
      if err.lengthNegative() {
        fmt.Printf("The length %0.2f, is less than 0\n", err.length)
      }
      if err.widthNegative() {
        fmt.Printf("The width %0.2f, is less than 0\n", err.width)
      }
      return
    }
    fmt.Println(err)
  }
  fmt.Println("Area of rect", area)
}

