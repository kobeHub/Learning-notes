package main

import "fmt"

func main() {
	FirstName := []string{"Leborn", "Michale", "Kobe"}
	LastName := []string{"James", "Jordan", "Bryant"}

LOOP:
	for _, first := range FirstName {
		for _, last := range LastName {
			fmt.Printf("Name: %s %s\n", first, last)

			if first == "Kobe" && last == "James" {
				break LOOP
			}
		}
	}
	fmt.Println("Done")
}
