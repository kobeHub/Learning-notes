package main


import (
  "fmt"
  "github.com/nf/oop/employee"
)


func main() {
  emp := employee.Employee{
    FirstName: "Leborn",
    LastName: "James",
    TotalLeaves: 30,
    LeavesTaken: 20,
  }

  fmt.Println("Opp examlpe 1:")
  emp.LeavesRemaining()
}

