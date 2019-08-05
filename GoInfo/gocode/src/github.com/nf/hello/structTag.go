package main

import (
	"fmt"
	"reflect"
)

type FormatedStr struct {
	Data string `color:"blue" spe:"gorm"`
}

type Alias struct {
	Name string `alias:"name" alias:"test"`
	age int64 `alias:""`
	time float64 
}

func main() {
	s := FormatedStr{"Balla"}
	st := reflect.TypeOf(s)
	field := st.Field(0)
	fmt.Println(s, st, field)
	fmt.Println(field.Tag.Get("color"), field.Tag.Get("spe"))

	a := Alias{}
	at := reflect.TypeOf(a)
	for i := 0; i < at.NumField(); i++ {
		field := at.Field(i)
		if alias, ok := field.Tag.Lookup("alias"); ok {
			fmt.Println("alias:", alias)
		} else {
			fmt.Println("Not specified")
		}
	} 
}
