package main


import (
  "fmt"
  "github.com/nf/oop/employee"
  "github.com/nf/oop/post"
)


func main() {
  emp := employee.New("Leborn", "James", 35, 20)

  fmt.Println("Opp examlpe 1:")
  emp.LeavesRemaining()
  fmt.Printf("Type:%T, name: %s %s, total:%d\n",
    emp, emp.GetFirstName(), emp.GetLastName(), emp.GetTotal())

  fmt.Println("\nPost composition:")

  author1 := post.Newauthor("John", "James", "Golang enthusiast")
  po1 := post.NewPost(
    "There would be a war between AI and human",
    "It maybe a war or something else",
    author1)
  po2 := post.NewPost(
    "Struct instead class in GO",
    "Go does not support class, but methods can be added to struct.",
    author1,
  )
  po3 := post.NewPost(
    "Concurrency",
    "Golang is a concurrency language and not a parallel one",
    author1,
  )

  website := post.WebSite{
    Posts: []post.Post{po1, po2, po3},
  }
  website.Display()

}

