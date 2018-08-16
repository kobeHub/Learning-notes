# Protocol Buffers

protocol buffers are a language-neutral, platform-neutral extensible mechnism for serializing structed data.

协议缓冲文件是google定义的类似于xml的简洁高效的结构化数据存储格式，可以用于结构数据的序列化（串行化）．方便文件的存储与网络传输　．XML是众所周知的空间密集型，编码/解码可能会对应用程序造成巨大的性能损失，protobuff 更小，快速，简单，只需要定义相应的数据结构，然后可以生成相应的源文件．通过建立`.proto` 文件描述符，协议缓冲符编译器会创建一个类，该类使用有效的二进制格式实现数据有效的自动编码和解码．生成的类为组成协议缓冲区字段提供`getter()` `setter()` ,并将协议缓冲区的读写细节作为一个单元来处理．

协议缓冲区格式支持随着时间的推移而扩展的想法，即代码仍然可以读取使用旧格式编码的数据

- 是一种序列化对象框架（或者说是编解码框架），其他功能相似的有Java自带的序列化、Facebook的Thrift和JBoss Marshalling等；
- 通过proto文件定义结构化数据，其他功能相似的比如XML、JSON等；
- 自带代码生成器，支持多种语言；

# Base syntax

## 1.定义一个消息类型

```protobuf
syntax = "proto3";		//首句明确使用的是proto3的语法

message SearchRequest {
  string query = 1;
  int32 page_number = 2;
  int32 result_per_page = 3;
}
```

在此例中，所有的数据都是标量，也可以采用其他的复杂类型作为数据域，包含枚举型(enumerations), 其他message类型．

**明确字段（field）的编号：**

在message的定义中，每一项都被赋予特定的唯一编号，并且在protobuff文件一旦使用后该编号即不可改变．注意域的编号一般要保留一定空间用于未来可能的扩充，并且不可以使用已经标记为`reserved` 的编号．

+ 1-15:采用一个字节编码，用于标识使用频率较高的message元素
+ 16-2047:采用两个字节编码
+ 最小的编号可以为１，最大可以为2**29-1
+ 编号19000到19999被作为协议缓冲区的实现，所以不可以作为字段编号

**消息字段类型以是以下两种之一：**

+ singular:一个格式正确(well-formed)的message可以至多包含一个该字段 
+ repeated: 该字段可以被重复任意次

**保留字段：(researed fields)**

考虑到升级时的问题，如果升级时完全移除一个字段，之后的用户可能会重用该字段的编号，这与字段编号的唯一性不符（如果用户调用旧版本的proto文件时会造成十分严重的错误）

而避免这种错误的方式可以是将需要删除的字段号或者名称定义为`reserved` ,之后的用户如果尝试再使用此字段时，编译器会报错

```protobuf
message Foo {
  reserved 2, 15, 9 to 11;
  reserved "foo", "bar";
}
```

*注意不可以将保留的　names 和　field number 放在同一个reserved 下*

## ２．标量值的类型

| .proto Type | Notes                                                        | C++ Type                                                     | Java Type  | Python Type[2] | Go Type | Ruby Type                      | C# Type    | PHP Type          |
| ----------- | ------------------------------------------------------------ | ------------------------------------------------------------ | ---------- | -------------- | ------- | ------------------------------ | ---------- | ----------------- |
| double      |                                                              | double                                                       | double     | float          | float64 | Float                          | double     | float             |
| float       |                                                              | float                                                        | float      | float          | float32 | Float                          | float      | float             |
| int32       | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32                                                        | int        | int            | int32   | Fixnum or Bignum (as required) | int        | integer           |
| int64       | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64                                                        | long       | int/long[3]    | int64   | Bignum                         | long       | integer/string[5] |
| uint32      | Uses variable-length encoding.                               | uint32                                                       | int[1]     | int/long[3]    | uint32  | Fixnum or Bignum (as required) | uint       | integer           |
| uint64      | Uses variable-length encoding.                               | uint64                                                       | long[1]    | int/long[3]    | uint64  | Bignum                         | ulong      | integer/string[5] |
| sint32      | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32                                                        | int        | int            | int32   | Fixnum or Bignum (as required) | int        | integer           |
| sint64      | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64The Any message type lets you use messages as embedded types without having their .proto definition. An Any contains an arbitrary serialized message as bytes, along with a URL that acts as a globally unique identifier for and resolves to that message's type. To use the Any type, you need to import google/protobuf/any.proto. | long       | int/long[3]    | int64   | Bignum                         | long       | integer/string[5] |
| fixed32     | Always four bytes. More efficient than uint32 if values are often greater than 228. | uint32                                                       | int[1]     | int/long[3]    | uint32  | Fixnum or Bignum (as required) | uint       | integer           |
| fixed64     | Always eight bytes. More efficient than uint64 if values are often greater than 256. | uint64                                                       | long[1]    | int/long[3]    | uint64  | Bignum                         | ulong      | integer/string[5] |
| sfixed32    | Always four bytes.                                           | int32                                                        | int        | int            | int32   | Fixnum or Bignum (as required) | int        | integer           |
| sfixed64    | Always eight bytes.                                          | int64                                                        | long       | int/long[3]    | int64   | Bignum                         | long       | integer/string[5] |
| bool        |                                                              | bool                                                         | boolean    | bool           | bool    | TrueClass/FalseClass           | bool       | boolean           |
| string      | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string                                                       | String     | str/unicode[4] | string  | String (UTF-8)                 | string     | string            |
| bytes       | May contain any arbitrary sequence of bytes.                 | string                                                       | ByteString | str            | []byte  | String (ASCII-8BIT)            | ByteString | string            |

You can find out more about how these types are encoded when you serialize your message in [Protocol Buffer Encoding](https://developers.google.com/protocol-buffers/docs/encoding).

### 默认值

在解析消息时，如果编码的消息不包含特定的singular元素，则解析对象中的对应的字段将设置为该字段的默认值：

+ string:默认为""
+ bytes: 空
+ bool: false
+ numeric type: 0
+ enums: 默认值是要保证第一个定义的值一定要是０
+ message: 依赖于具体语言

对于标量消息字段，一旦解析了消息，就无法判断字段是否明确设置为默认值（例如布尔值是否设置为false）或者根本没有设置：需要牢记定义的消息类型。例如，如果不希望该行为在默认情况下也发生，请将其设置为false时打开某些行为的布尔值。如果标量消息字段被设置为其默认值，则该值不会在连线上序列化

##3.Enumerations 枚举类型

为了在定义消息类型时，将某个字段的域明确在某些预定义的值．可以使用枚举类型

```protobuf
message SearchRequest {
  string query = 1;
  int32 page_number = 2;
  int32 result_per_page = 3;
  enum Corpus {
    UNIVERSAL = 0;   //  注意每个枚举类型必须保证第一个定义的数据为０
    WEB = 1;
    IMAGES = 2;
    LOCAL = 3;
    NEWS = 4;
    PRODUCTS = 5;
    VIDEO = 6;
  }
  Corpus corpus = 4;
}

//可以将多个枚举常量赋给同一个值，但是需要明确　allow_alias = true;
enum EnumAllowingAlias{
    optional allow_alias = true;
    unknown = 0;
    started = 1;
    running = 1;
}
```

**保留值：**

注意，枚举类型中的值也不可以造成未来的隐患，所以需要保留字，将需要删除的值进行reserved标识

```protobuf
enum Foo {
  reserved 2, 15, 9 to 11, 40 to max;
  reserved "FOO", "BAR";
}
```

## 4．其他消息类型的引入

对于同一个protobuff文件中的message可以直接引入并在其他message的定义中使用，对于其他文中的定义可以使用import语句进行引入

通常情况下．只可以从直接import的proto文件中使用定义．但是，有时需要将.proto文件移至新位置，这时不需要直接移动文件并且在这次变换中更新所有的调用点；可以在原来的位置上放置一个虚拟的`.proto` 文件，然后使用`import public ` 调用，将所有的导入转移到新位置．

```protobuf
// new.proto
// All definitions are moved here



// old.proto
// This is the proto that all clients are importing.
import public "new.proto";
import "other.proto";



// client.proto
import "old.proto";
// You use definitions from old.proto and new.proto, but not other.proto

```

proto编译器使用`-I/- proto_path` 标志在命令行中的指定目录中搜索导入的文件，如果没有给出标志，将在调用编译器的目录中查找，原则上讲，应该将 --proto_path　设置为项目的根目录，并且所有的导入均使用完全限定名称．

## 5. Nested Types 嵌套类型

可以再一个message中定义嵌套的message 并且可以进行多层嵌套

```protobuf
message Search{
    message Result {
        string url = 1;
        string titile = 2;
        repeated string sni = 3;
    }
    repeated Result results = 1;
}
```

如果需要调用内部的嵌套子类，可以使用`parent.child` 进行调用

```protobuf
message other {
    Search.Result result = 1;
}
```

当然，嵌套层数可以任意

```proto
message Outer {                  // Level 0
  message MiddleAA {  // Level 1
    message Inner {   // Level 2
      int64 ival = 1;
      bool  booly = 2;
    }
  }
  message MiddleBB {  // Level 1
    message Inner {   // Level 2
      int32 ival = 1;
      bool  booly = 2;
    }
  }
}
```

# Other syntax

## 1. Any 类型嵌套

any message 类型允许使用消息嵌套而不需要借助其`.proto` 定义，any 包含一个二进制序列化的messge 并作为`bytes` 类型，与一个url一起作为message type一个全局的统一定位符

使用前需要导入"google/protobuf/any.proto" 文件

```protobuf
import "google/protobuf/any.proto";

message ErrorStatus {
  string message = 1;
  repeated google.protobuf.Any details = 2;
}
```

默认的类型ＵＲＬ为：　`type.googleapis.com/packagename.messagename` 

## 2. oneof

如果有一个包含多字段的消息，并且最多只能同时设置一个字段，则可以使用此功能强制执行此操作可以节省内存

oneof字段与普通字段的一样除了所有被定义在`oneof` 中的字段共享一段内存资源，并且每次仅可以设置一个字段．设置oneof中的一个成员会自动清除其他成员．

```protobuf
message SampleMessage{
    oneof test{
        string name = 4;
        SubMessage sub_message = 9;
    }
}
```

现在可以添加oneof字段，可以添加到任何类型，但是不可以使用`repeated` 关键字

**c++实例：**

+ 设置oneof字段会自动清除其他字段，因此只有最后的设置有效

  ```c++
  SampleMessage message;
  message.set_name("Name);
  CHECK(message.has_name());
  message.mutable_sub_message();
  CHECK(!message.has_name())
  ```

+ 如果解析器在连线上遇到同一成员，则只有最后一个成员在解析消息时有用

+ 以下代码会导致memory crash因为sub_message已经被删除

  ```c++
  SampleMessage message;
  SubMessage *sub_message = message.mutable_sub_message();
  message.set_name("Clear");    //将会清除其他的所有字段
  sub_message -> set_name("sub");    //已经被删除，报错
  ```

+ 如果你用oneofs交换两个消息，每个消息都会以另一个case为结尾：在下面的例子中，msg1将有   一个sub_message，而msg2将有一个名字

  ```c++
  SampleMessage msg1;
  msg1.set_name("name");
  SampleMessage msg2;
  msg2.mutable_sub_message();
  msg1.swap(&msg2);
  CHECK(msg1.has_sub_message());
  CHECK(msg2.has_name());
  ```



## 3.　Maps 

可以再protobuf中使用 `map<>` 来创建两个相关联的二元组（字典），包含了`key_type` ,`value_type` 分别指定键值和实际值的类型，键值可以是string 或者数字，但是不可以是其他的map,比如说`enum` 

+ map 字段不可以 `repeated`
+ 线格式排序和map迭代排序的map值是不能确定的，所以不能认为map类型的每一项会有一个特定的顺序
+ 当为.proto生成文本格式时，map按照键值排序
+ 从线路解析或者合并时，如果有重复的键值，则使用的是所看到的最后一个键值；从文本格式解析映射时，如果有重复的键值，可能解析失败．
+ 如果对于同一个map只提供键值而不提供值，则根据使用的语言来自动填补如：java c++ python会填补默认值

```protobuf
map<string, Project> projects = 3;
```

# 组织构造

## Packages

在.proto文件中可以指定一个包名，指明message定义所属的组织结构，然后可应通过 packagename.message_name 进行调用．

```protobuf
package foo.bar;
message Open { ... }


//**********************************************//


message Foo{
    foo.bar.Open open = 1;
}
```

**在不同的编程语言的表现：**

+ c++: 产生的类将会被包装在`namespace` 本例中的Open将会被保证在`foo::bar` 
+ java: 作为ｊａｖａ文件的包
+ python: 被忽略，因为python的模块的组织架构根据文件系统

## 定义服务

可以用proto文件定义一RPC系统，可以再 .proto文件中定义RPC服务的所有接口，然后可以生成所需语言的RPC脚本．

```protobuf
service SearchService {
  rpc Search (SearchRequest) returns (SearchResponse);
}
```



## JOSN 转换

*//TODO*



## OPTION

可以再.proto文件中声明option语句进行可用性设定,这些设定包含  

file：即可以作用在最高层次的命名域,  

message：在一个message的定义的作用域起作用,  

field：在一个字段声明中起作用

+ `java_package` :指定生成java类的包，如果未指明，默认会在`package` 语句定义的路径，若不生成ｊａｖａ文件，该语句则不起作用
+ `java_mutable_files` : 导致在包级别定义顶级消息，枚举和服务，而不是在以.proto文件命名的外部类中定义
+ `java_outer_classname`: 要生成的最外面的Java类（以及因此的文件名）的类名称。如果没有在.proto文件中指定明确的java_outer_classname，则通过将.proto文件名称转换为camel-case[驼峰模式]（因此foo_bar.proto变为FooBar.java）来构造类名称。如果不生成Java代码，则此选项不起作用。
+ `optimize_for`: 可以对java c++起作用有三个选项
  + `SPEED`(default): 编译器将生成用于序列化，解析和执行消息类型的其他常见操作的代码。这段代码是高度优化的。
  + `CODE_SIZE`: 编译器将生成最少的类，并将依靠共享的基于反射的代码来实现序列化，解析和各种其他操作。生成的代码因此比SPEED要小得多，但操作会更慢。类仍将实现与SPEED模式中完全相同的公共API。此模式在包含大量.proto文件的应用程序中非常有用，并且不需要所有这些文件都快速地闪烁
  + `LITE_RUNTIME`: 编译器将生成仅取决于“lite”运行时库（libprotobuf-lite而不是libprotobuf）的类。 lite运行时比整个库小得多（大约小一个数量级），但省略了描述符和反射等特定功能。这对于在移动电话等受限平台上运行的应用程序特别有用。编译器仍然会像在SPEED模式下那样生成所有方法的快速实现。生成的类将仅实现每种语言的MessageLite接口，该接口仅提供完整的Message接口的一部分方法。
+ `deprecated` :如果设置为true，则表示该字段已弃用且不应被新代码使用。在大多数语言中，这没有      实际影响。  在Java中，这变成了@Deprecated注释。将来，其他语言特定的代码生成器可能会在字段的访问器上生成弃用注释，这会在编译试图使用该字段的代码时导致发出警告。如果该字段未被任何人使用，并且您想阻止新用户使用该字段，请考虑用保留语句替换该字段声明。

## 产生相应的代码

```sh
protoc --proto_path=IMPORT_PATH --cpp_out=DST_DIR --java_out=DST_DIR --python_out=DST_DIR --go_out=DST_DIR --ruby_out=DST_DIR --objc_out=DST_DIR --csharp_out=DST_DIR path/to/file.prototest
```

