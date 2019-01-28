package employee
/* The employee package Employee struct */

import (
  "fmt"
)

// Define the struct with all fields exported
type employee struct {
  firstName, lastName string
  totalLeaves, leavesTaken int
}

// Exported method with a receiver of Employee
func (e employee) LeavesRemaining() {
  fmt.Printf("%s %s has %d leaves remaining.\n",
              e.firstName, e.lastName, (e.totalLeaves - e.leavesTaken))
}

// New function as a Constructor
func New(firstName, lastName string, totalLeaves, leavesTaken int)  employee {
  e := employee {firstName, lastName, totalLeaves, leavesTaken}
  return e
}

// Property of employee
func (e employee) GetFirstName() string {
  return e.firstName
}

func (e employee) GetLastName() string {
  return e.lastName
}

func (e employee) GetTotal() int {
  return e.totalLeaves
}

