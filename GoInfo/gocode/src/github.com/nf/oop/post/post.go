package post

/* define the post struct */
import (
  "fmt"
)

// author of the post
type author struct {
  firstName, lastName string
  bio string
}

// get the author full name
func (a author) AuthorName() string {
  return fmt.Sprintf("%s %s", a.firstName, a.lastName)
}

// struct of post, `author` field promoted
// so Post type can use `AuthorName` directly
type Post struct {
  Title string
  Content string
  author
}

func (p Post) Detail() {
  fmt.Println("Post:", p.Title)
  fmt.Println("Content:", p.Content)
  fmt.Println("Author:", p.AuthorName())
  fmt.Println("Bio", p.bio)
}



