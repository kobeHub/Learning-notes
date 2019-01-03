# Strings操作

[TOC]

## 1.类型定义

strings类型在go语言中是一个bytes的切片，可以使用双引号来构建strings.strings在golang中使用的是utf-8编码。使用byte类型切片的主要原因与ASCII码与unicode共存的原因类似，由于英文只使用一个字节就可以全部表示，但是其他字符表示可能需要多个字节，所以为了节省存储空间，默认采用byte。

注意strings是一个只读类型的byte切片，所以是不可更改类型。strings实际保存的是二进制byte而不是unicode码，或者其他的编码形式。下面是一个字符串的字面量**(string literal)**,可以使用2位十六进制表示一个特定的byte

```go
const sample = "\xbd\xb2\x3d\xbc\x20\xe2\x8c\x98"
```



由于strings是一个byte类型的切片，所以可以使用

可以使用一个byte的切片进行显式的定义string

## 2. 访问string 的每一个字符

因为strings是一个byte的切片，所以可以访问其中的每一个字符

```go
func printBytes(s string) {  
    for i:= 0; i < len(s); i++ {
        fmt.Printf("%x ", s[i])
    }
}

func printChars(s string) {  
    for i:= 0; i < len(s); i++ {
        fmt.Printf("%c ",s[i])
    }
}

func main() {  
    name := "Hello World"
    printBytes(name)
    printChars(name)
}
// 输出其十六进制编码
48 65 6c 6c 6f 20 57 6f 72 6c 64  
H e l l o   W o r l d  
```

对于格式化输出，使用`%x`输出十六进制编码，使用`%c`输出相应字符。注意对于英文使用的是ASCII编码方式，所以对于每一个byte直接按位输出就是其正确编码；但是对于unicode编码，使用的是utf-8格式，所以需要使用三个字节表示一个字符，对于中文而言，每一字符占用三个字节.

```go
func printBytes(s string) {
	for i := 0; i < len(s); i++ {
		fmt.Printf("%x ", s[i])
	}
}

func printChars(s string) {
	for i := 0; i < len(s); i++ {
		fmt.Printf("%c ", s[i])
	}
	for i, v := range s {
		fmt.Printf("%#U: %c, %d\n", v, v, i)
	}
}

func main() {
	name := "什么"
	printBytes(name)
	fmt.Printf("\n")
	printChars(name)
}
/*
e4 bb 80 e4 b9 88 
ä »  ä ¹  
U+4EC0 '什': 什, 0
U+4E48 '么': 么, 3
*/
```

使用`%#U` 输出字符的unicode编码，每个字符使用三个字节的编码。

## 3.rune

对于unicode编码，如果使用bytes编码会出现错误，可以使用rune，一个32位整数表示。

```go
func printChars(s string) {
    runes := []rune(s)  // 建立rune切片
    for i := 0; i < len(runes); i++ {
        fmt.Printf("%c ", runes[i])
    }
}
// 正常输出
```

由于string是不可更改类型，如果需要对其进行更改，可以使用rune的切片进行处理

```go
func mutate(s []rune, a rune) {
    s[0] = a
    return string(s)
}
str = mutate([]rune(str), 'a')
```

## 4.utf-8 string的长度

对于string类型的长度不可以直接使用`len()`函数进行计算，因为其包含unicode码，需要使用`unicode/utf8` 模块的`utf8.RuneCountInString()` 函数计算string的长度

```go
func leng_test(str string) {
  fmt.Println("The string:", str)
  fmt.Println("Use len():", len(str))
  fmt.Println("Acctually length:", utf8.RuneCountInString(str))
}
// 对于中文，每一个字符使用三个字节表示，所以len 会是长度的3倍
```

## 5. 使用`fmt.printf()` 输出[]rune

对于一个字符串的字面量(string literal)，可以使用十六进制表示特定的byte值，从而表示一个字符串，进行输出时可以使用多种格式化输出例如:

+ `%x` 输出十六进制代码， 可以在中间加空格，表示分隔
+ `%q` quoted 跳过所有不可打印的字符
+ `%+q` 跳过所有不可打印以及非ASCII码的字符
+ `%#U` 输出unicode代码

可以使用``表示一个raw string，字面string,里面只可以包含文字字符，而不可以包含其他字符。

```go
func main() {
    const placeOfInterest = `⌘`

    fmt.Printf("plain string: ")
    fmt.Printf("%s", placeOfInterest)
    fmt.Printf("\n")

    fmt.Printf("quoted string: ")
    fmt.Printf("%+q", placeOfInterest)
    fmt.Printf("\n")

    fmt.Printf("hex bytes: ")
    for i := 0; i < len(placeOfInterest); i++ {
        fmt.Printf("%x ", placeOfInterest[i])
    }
    fmt.Printf("\n")
}
//
plain string: ⌘
quoted string: "\u2318"
hex bytes: e2 8c 98 

```

   