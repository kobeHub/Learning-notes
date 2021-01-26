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
ANO_LOOP:
for _, l := range LastName {
	for i, _ := range FirstName {
			fmt.Printf("last name: %s, first index: %v\n", l, i)
			if l == "Kobe" {
				break ANO_LOOP
			}
		}
	}
	fmt.Println("Done")
}
