package main

import "fmt"

func main() {
	const (
		Ldate         = 1 << iota     // the date in the local time zone: 2009/01/23
		Ltime                         // the time in the local time zone: 01:23:23
		Lmicroseconds                 // microsecond resolution: 01:23:23.123123.  assumes Ltime.
		Llongfile                     // full file name and line number: /a/b/c/d.go:23
		Lshortfile                    // final file name element and line number: d.go:23. overrides Llongfile
		LUTC                          // if Ldate or Ltime is set, use UTC rather than the local time zone
		LstdFlags     = Ldate | Ltime // initial values for the standard logger
	)

	fmt.Println(Ldate, Ltime, Lmicroseconds, Llongfile, Lshortfile, LUTC, LstdFlags)
}