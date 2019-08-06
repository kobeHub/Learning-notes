package main

import (
  "fmt"
  "unicode/utf8"
	"strings"
)

const sample = "\xbd\xb2\x3d\xbc\x20\xe2\x8c\x98"

func string_construct() {
  data := []byte{0x43, 0x61, 0x66, 0xc3, 0xa9}
  str1 := string(data)
  fmt.Println("string and cons:")
  fmt.Printf("%s\n", str1)
  for i, v := range str1 {
    fmt.Printf("Index:%d %#U, %c\n", i, v, v)
  }

  runes := []rune{0x0053, 0x0065, 0x00f1, 0x006f, 0x0072}
  str2 := string(runes)
  fmt.Println(str2)
}

func leng_test(str string) {
  fmt.Println("The string:", str)
  fmt.Println("Use len():", len(str))
  fmt.Println("Acctually length:", utf8.RuneCountInString(str))
}

func mutate(s []rune, a rune) string {
  // strings are immutable but can use runes slice
  s[0] = a
  return string(s)   // 注意golang强类型语言，必须进行显式的类型转化
}

func sample_test() {
  fmt.Println("\n\nNow test the string literal:")
  fmt.Println(sample)

  fmt.Println("Printf with %x, %q, %+q")
  for i := 0; i < len(sample); i++ {
    fmt.Printf("%x, %q, %+q\n", sample[i], sample[i], sample[i])
  }

  const place = `是`
  fmt.Println("\nplain text:", place)
  fmt.Printf("quoted tring:%+q\n", place)
  fmt.Printf("hex bytes: ")
  for i := 0; i < len(place); i++ {
    fmt.Printf("% x", place[i])
  }

  fmt.Println()
  const ano = "中国人"
  for index, runeValue := range ano {
    fmt.Printf("%#U starts at bytes position %d\n", runeValue, index)
  }

}

func main() {
	fmt.Println("String trim")
	task := "just a test and test"
	fmt.Println(strings.Trim(strings.Join(strings.Split(task, " "), "-"), "[]"))

  string_construct()
  str := "Señor"
  leng_test(str)
  str1 := "Hello"
  str1 = mutate([]rune(str1), 't')
  fmt.Println(str1)

  sample_test()
}
