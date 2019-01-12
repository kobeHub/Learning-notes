package main

import (
  "fmt"
  "time"
)

func time_test() {
  const longForm = "Jan 2, 2006 at 3:04pm (MST)"
  t, _ := time.Parse(longForm, "Feb 12, 2019 at 12:00pm (PST)")
  fmt.Println(t)

  const shortForm = "2006-Jan-02"
  t, _ = time.Parse(shortForm, "2019-Jan-08")

  t, _ = time.Parse(time.RFC3339, "2019-01-08T15:04:05Z")
  fmt.Println(t)
  t, _ = time.Parse(time.RFC3339, "2019-01-15T15:04:05+08:00")
  fmt.Println(t)
  _, err := time.Parse(time.RFC3339, time.RFC3339)
  fmt.Println(err)
}

func main() {
  time_test()
}
