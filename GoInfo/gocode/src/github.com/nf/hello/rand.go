package main

import (
	"fmt"
	"time"
	"math/rand"
)

func main() {
	pseudo := rand.Perm(16)
	fmt.Println(pseudo)

	r := rand.New(rand.NewSource(time.Now().Unix()))
	fmt.Println(r.Perm(16))
}
