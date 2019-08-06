package main

import (
	"fmt"
	_ "github.com/go-sql-driver/mysql"
	"github.com/jinzhu/gorm"
)

func main() {
	const URL = "root:123456@tcp(127.0.0.1:3306)/annotation?charset=utf8&parseTime=True&loc=Local"
	db, err := gorm.Open("mysql", URL)
	if err != nil {
		fmt.Println("Open DB:", err)
	}

	err = db.DB().Ping()
	if err != nil {
		panic(err.Error())
	}
	fmt.Println("Connect successful")
}
