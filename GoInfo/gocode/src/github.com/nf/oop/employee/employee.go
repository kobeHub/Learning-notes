package employee
/* The employee package Employee struct */

import (
  "fmt"
)

// Define the struct with all fields exported
type Employee struct {
  FirstName, LastName string
  TotalLeaves, LeavesTaken int
}

// Exported method with a receiver of Employee
func (e Employee) LeavesRemaining() {
  fmt.Printf("%s %s has %d leaves remaining.\n",
              e.FirstName, e.LastName, (e.TotalLeaves - e.LeavesTaken))
}

