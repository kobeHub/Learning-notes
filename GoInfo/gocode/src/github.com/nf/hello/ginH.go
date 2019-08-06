package main

import (
	"fmt"
	"github.com/gin-gonic/gin"
)

func main() {
	a := gin.H{"sa": 12, "as": "as"}
	fmt.Println(a["as"], a["sa"])
}
