# [StructTag](https://golang.org/pkg/reflect/#StructTag)



## 1. 结构体标记

按照惯例，StructTag字符串是一个可选的，在类型后由空格分隔的键值对。StructTag类型是string的别名，一般使用反引号。Go的字符串是一个任意字节的字符串常量，双引号以及反引号都可以创建一个常量的字符串，区别在于：

+ 双引号创建可解析的字符串字面量，支持转义，不可以引用多行
+ 反引号，用以表示原生字符串，通常用于多行、正则表达式、结构体标记
+ 单引号用于表示`rune`，码点字面量

Go对于结构体的字段标记可以通过反射机制得到，所以通常用于在对struct的编码转换过程中提供一些规则转换信息，例如常用的JSON转换。可以用于存储所需的所有元信息。

## 2. StructTag 操作

+ ```go
  func (tag StructTag) Get(key string) string
  ```

  get 方法得到一个结构体标记内的key对应的value，如果对应的key，没有定义，会得到一个 undefined错误

+ ```go
  func (tag StructTag) Lookup(key string) (value string, ok bool)
  ```

  在tag中寻找与key关联的value，value可以为空，ok返回tag中是否有该键值

## 3. JSON tag

这里着重举出 JSON 的 Struct Tag（1. JSON 输出很常见; 2. 可以以此类推其他如 XML’s Tag）。 我想知道 JSON 的 tag 有哪些，去哪找？去官网 [JSON.Marshal](https://link.juejin.im?target=https%3A%2F%2Fgolang.org%2Fpkg%2Fencoding%2Fjson%2F%23Marshal) 函数文档中找。

> The encoding of each struct field can be customized by the format string stored under the "json" key in the struct field's tag. The format string gives the name of the field, possibly followed by a comma-separated list of options. The name may be empty in order to specify options without overriding the default field name. 我们会发现，在 JSON 编码过程中会去获取每一个 Struct field 的标记，从中拿取 key 为 `json` 的值，然后进行相应处理。

注意解析规则：value 的第一个字符串一定表示覆盖后的新字段名，后面如果有解析选项，则以英文逗号分隔。

比如 `Name string json:"name,omitempty"`，第一个字符串 name 表示在编码后 Name 属性名就变成了 name。然后紧跟逗号分隔符，接着是 omitempty 选项。

1. 如果我不想覆盖，只想加选项怎么办？`Name string json:",omitempty"`，直接英文逗号打头。
2. 极端一点，如果我的字段名就叫 Omitempty 呢？`Omitempty string json:"omitempty,omitempty"`，记住第一个字符串表示的是新变量名，而不是选项，所以重名就重名好了，不怕🤪。

> 思考一下：`- string json:"-,"` 和 `- string json:",-"` 有什么区别🧐？

1. **omitempty**：如果字段的值为空（Defined as false, 0, a nil pointer, a nil interface value, and any empty array, slice, map, or string），那么在编码过程中就忽略掉这个字段。
2. **-**：二话不说直接忽略该字段。
3. **string**：将字段值在编码过程中转换成 JSON 中的字符串类型，只有当字段类型是 string, floating point, integer, or boolean 的情况下才会转换。