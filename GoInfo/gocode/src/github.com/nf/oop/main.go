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

  fmt.Println("Post composition:")

  po := post.Post{

  }
}

