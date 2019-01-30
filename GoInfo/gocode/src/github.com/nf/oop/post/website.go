package post

import "fmt"

// define the websites struct which contains posts
type WebSite struct {
  Posts []Post
}

// display the website msssage
func (w WebSite) Display() {
  fmt.Println("The content of the website:\n")
  for _, po := range w.Posts {
    po.Detail()
    fmt.Println()
  }
}
