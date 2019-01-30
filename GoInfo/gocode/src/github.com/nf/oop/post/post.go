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

func Newauthor(firstName, lastName, bio string) author {
  return author{firstName, lastName, bio}
}

// struct of post, `author` field promoted
// so Post type can use `AuthorName` directly
type Post struct {
  Title string
  Content string
  author
}

func NewPost(title, content string, a author) Post {
  return Post{title, content, a}
}

func (p Post) Detail() {
  fmt.Println("Post:", p.Title)
  fmt.Println("Content:", p.Content)
  fmt.Println("Author:", p.AuthorName())
  fmt.Println("Bio:", p.bio)
}



