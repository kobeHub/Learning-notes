# python os操作

[TOC]

## 1.os打开及关闭文件标识符

os.open() 方法用于打开一个文件，并且设置需要的打开选项，模式参数mode参数是可选的，默认为 0777。返回值是文件描述符（一个整数）

![os.open](https://github.com/kobeHub/Hello-world/blob/master/pic/osopen.png)

```
os.open(file, flags[, mode]);
```

**参数**

- **file** -- 要打开的文件

- **flags** -- 该参数可以是以下选项，多个使用 "|" 隔开：

  - **os.O_RDONLY:** 以只读的方式打开
  - **os.O_WRONLY:** 以只写的方式打开
  - **os.O_RDWR :** 以读写的方式打开
  - **os.O_NONBLOCK:** 打开时不阻塞
  - **os.O_APPEND:** 以追加的方式打开
  - **os.O_CREAT:** 创建并打开一个新文件
  - **os.O_TRUNC:** 打开一个文件并截断它的长度为零（必须有写权限）
  - **os.O_EXCL:** 如果指定的文件存在，返回错误
  - **os.O_SHLOCK:** 自动获取共享锁
  - **os.O_EXLOCK:** 自动获取独立锁
  - **os.O_DIRECT:** 消除或减少缓存效果
  - **os.O_FSYNC :** 同步写入
  - **os.O_NOFOLLOW:** 不追踪软链接

- **mode** -- 类似 [chmod()](http://www.runoob.com/python3/python3-os-chmod.html)。

  **关闭文件标识符：**

  `os.close(fd)`

  `os.closerange(fd_low, fd_high)` 关闭所有文件描述符，从fd_low(包含) 到fd_high（不包含） 

  `os.dup(fd)`:复制文件描述符给另一个变量

  `os.dup2(fd, fd2)`:把fd复制给fd2
### :star:通过文件标识符创建文件对象

`os.fdopen(fd, [, mode[, buffsize]])`

通过文件描述符创建文件对象并返回是函数`open()`的别名，可接受一样参数，但第一个参数必须是整形

- **fd** -- 打开的文件的描述符，在Unix下，描述符是一个小整数。
- **mode** -- 可选，和 Python 内建的 open 函数一样，mode参数可以指定『r,w,a,r+,w+,a+,b』等，表示文件的是只读的还是可以读写的，以及打开文件是以二进制还是文本形式打开。这些参数和C语言中的<stdio.h>中fopen函数中指定的mode参数类似。
- **bufsize** -- 可选，指定返回的文件对象是否带缓冲：bufsize=0，表示没有带缓冲；bufsize=1，表示该文件对象是行缓冲的；bufsize=正数，表示使用一个指定大小的缓冲冲，单位为byte，但是这个大小不是精确的；bufsize=负数，表示使用一个系统默认大小的缓冲，对于tty字元设备一般是行缓冲，而对于其他文件则一般是全缓冲。如果这个参数没有制定，则使用系统默认的缓冲设定。

## 2.查看及改变os工作目录

### :star: 改变当前进程根目录

`os.chroot(path)`:需要得到管理员权限

### :star:查看及改变当前工作目录

`os.getcwd()`:得到当前目录

`os.fchdir(fd)`: 通过文件描述符改变当前工作目录

`os.chdir(path)`:改变当前目录

## 3.权限查看及改变

### :star:  权限查看

`os.access(path, mode);` os.access() 方法使用当前的uid/gid尝试访问路径。大部分操作使用有效的 uid/gid, 因此运行环境可以在 suid/sgid 环境尝试

**参数：**

- **path** -- 要用来检测是否有访问权限的路径。
- **mode** -- mode为F_OK，测试存在的路径，或者它可以是包含R_OK, W_OK和X_OK或者R_OK, W_OK和X_OK其中之一或者更多。
  - **os.F_OK:** 作为access()的mode参数，测试path是否存在。
  - **os.R_OK:** 包含在access()的mode参数中 ， 测试path是否可读。
  - **os.W_OK** 包含在access()的mode参数中 ， 测试path是否可写。
  - **os.X_OK** 包含在access()的mode参数中 ，测试path是否可执行。

### :star: 改变权限

os.chmod() 方法用于更改文件或目录的权限。

**chmod()**方法语法格式如下：

```
os.chmod(path, mode)
```

- **path** -- 文件名路径或目录路径。
- **flags** -- 可用以下选项按位或操作生成， 目录的读权限表示可以获取目录里文件名列表， ，执行权限表示可以把工作目录切换到此目录 ，删除添加目录里的文件必须同时有写和执行权限 ，文件权限以用户id->组id->其它顺序检验,最先匹配的允许或禁止权限被应用。
  - **stat.S_IXOTH:** 其他用户有执行权0o001
  - **stat.S_IWOTH:** 其他用户有写权限0o002
  - **stat.S_IROTH:** 其他用户有读权限0o004
  - **stat.S_IRWXO:** 其他用户有全部权限(权限掩码)0o007
  - **stat.S_IXGRP:** 组用户有执行权限0o010
  - **stat.S_IWGRP:** 组用户有写权限0o020
  - **stat.S_IRGRP:** 组用户有读权限0o040
  - **stat.S_IRWXG:** 组用户有全部权限(权限掩码)0o070
  - **stat.S_IXUSR:** 拥有者具有执行权限0o100
  - **stat.S_IWUSR:** 拥有者具有写权限0o200
  - **stat.S_IRUSR:** 拥有者具有读权限0o400
  - **stat.S_IRWXU:** 拥有者有全部权限(权限掩码)0o700
  - **stat.S_ISVTX:** 目录里文件目录只有拥有者才可删除更改0o1000
  - **stat.S_ISGID:** 执行此文件其进程有效组为文件所在组0o2000
  - **stat.S_ISUID:** 执行此文件其进程有效用户为文件所有者0o4000
  - **stat.S_IREAD:** windows下设为只读
  - **stat.S_IWRITE:** windows下取消只读



`os.fchmod(fd, mode)` 通过文件标识符改变权限

**改变文件所有者：**

`os.chown(path, uid, gid)`   `os.fchmod(fd, uid, gid)`

os.fstat(fd).st_uid		os.stat(path).st_uid  		查看文件拥有者用户id，st_gid 查看所有者组id

## 4.文件信息查看

### 系统调用查看

os.stat() 方法用于在给定的路径上执行一个系统 stat 的调用。

`stat() ` 方法语法格式如下：

```python
os.stat(path)
os.lstat(path) 与stat类似，没有软连接
os.fstat(fd)  返回文件标识符的stat
```

- **path** -- 指定路径

**stat 结构:**

- **st_mode: **inode 保护模式

- **st_ino: **inode 节点号。

- **st_dev: **inode 驻留的设备。

- **st_nlink: **inode 的链接数。

- **st_uid: **所有者的用户ID。

- **st_gid: **所有者的组ID。

- **st_size: **普通文件以字节为单位的大小；包含等待某些特殊文件的数据。

- **st_atime: **上次访问的时间。

- **st_mtime: **最后一次修改的时间。

- **st_ctime: **由操作系统报告的"ctime"。在某些系统上（如Unix）是最新的元数据更改的时间，在其它系统上（如Windows）是创建时间（详细信息参见平台的文档）。

  ​

### 文件信息查看

使用`os.statvfs(path)`  `os.fstatvfs(fd)`  返回相应文件或者文件夹的文件系统信息返回的结构

`<class os.statvfs_result>` :

- **f_bsize:** 文件系统块大小
- **f_frsize:** 分栈大小
- **f_blocks:** 文件系统数据块总数
- **f_bfree:** 可用块数
- **f_bavail:**非超级用户可获取的块数
- **f_files:** 文件结点总数
- **f_ffree:** 可用文件结点数
- **f_favail:** 非超级用户的可用文件结点数
- **f_fsid:** 文件系统标识 ID
- **f_flag:** 挂载标记
- **f_namemax:** 最大文件长度

## 5.os读写

os.write() 方法用于写入字符串到文件描述符 fd 中. 返回实际写入的字符串长度。

### 语法

**write()**方法语法格式如下：

```
os.write(fd, str)
```

### 参数

- **fd** -- 文件描述符。
- **str** -- 写入的字符串。

### 返回值

该方法返回写入的实际位数。

### 实例

以下实例演示了 write() 方法的使用：

```python
#!/usr/bin/python3

import os, sys

# 打开文件
fd = os.open("f1.txt",os.O_RDWR|os.O_CREAT)

# 写入字符串
str = "This is runoob.com site"
ret = os.write(fd,bytes(str, 'UTF-8'))

# 输入返回值
print ("写入的位数为: ")
print (ret)

print ("写入成功")

# 关闭文件
os.close(fd)
print ("关闭文件成功!!")
```



os.read() 方法用于从文件描述符 fd 中读取最多 n 个字节，返回包含读取字节的字符串，文件描述符 fd对应文件已达到结尾, 返回一个空字符串。

### 语法

**read()**方法语法格式如下：

```
os.read(fd,n)
```

### 参数

- **fd** -- 文件描述符。
- **n** -- 读取的字节。

### 返回值

返回包含读取字节的字符串

### 实例

以下实例演示了 read() 方法的使用：

```python
#!/usr/bin/python3

import os, sys
# 打开文件
fd = os.open("f1.txt",os.O_RDWR)
   
# 读取文本
ret = os.read(fd,12)
print (ret)

# 关闭文件
os.close(fd)
print ("关闭文件成功!!")
```

执行以上程序输出结果为：

```
This is test
关闭文件成功!!
```



> `os.fsync(fd)`: 强制将文件标识符写入硬盘
>
> `os.fdatasync(fd)`:强制将文件写入硬盘但不更新文件状态信息，可用于刷新缓存区



## 6.链接进行文件备份

### 硬链接
> `os.link(src， dst)`:创建硬链接src为原地址，dst为目标地址

```python
#!/usr/bin/python3

import os, sys

# 打开文件
path = "/var/www/html/foo.txt"
fd = os.open( path, os.O_RDWR|os.O_CREAT )

# 关闭文件
os.close( fd )

# 创建以上文件的拷贝
dst = "/tmp/foo.txt"
os.link( path, dst)

print ("创建硬链接成功!!")
```

### 软链接

> `os.symlink(src, dst)`:创建软链接
>
> os.readlink(dst): 返回软连接指向的文件

```
#!/usr/bin/python3

import os

src = '/usr/bin/python'
dst = '/tmp/python'

# 创建软链接
os.symlink(src, dst)

# 使用软链接显示源链接
path = os.readlink( dst )
print (path)
```
## 7.重命名

os.rename()方法用于重命名一个文件或者目录，从src 到dst 若dst存在则抛出OSError

```python
!/usr/bin/python3

import os, sys

# 列出目录
print ("目录为: %s" % os.listdir(os.getcwd()))

# 重命名
os.rename("test","test2")

print ("重命名成功。")

# 列出重命名后的目录
print ("目录为: %s" %os.listdir(os.getcwd()))
```

## 8.创建管道

`os.pipe()`用于创建一个管道返回一对文件描述符（r，w）分别进行读写

```python
#!/usr/bin/python3

import os, sys

print ("The child will write text to a pipe and ")
print ("the parent will read the text written by child...")

# 文件描述符 r, w 用于读、写
r, w = os.pipe() 

processid = os.fork()  # 产生两个进程，子进程的pid为0  父进程pid大于0
if processid:
    # 父进程
    # 关闭文件描述符 w
    os.close(w)
    r = os.fdopen(r)
    print ("Parent reading")
    str = r.read()
    print ("text =", str)
    sys.exit(0)
else:
    # 子进程
    os.close(r)
    w = os.fdopen(w, 'w')
    print ("Child writing")
    w.write("Text written by child...")
    w.close()
    print ("Child closing")
    sys.exit(0)
```

执行以上程序输出结果为：

```python
The child will write text to a pipe and
the parent will read the text written by child...
Parent reading
Child writing
Child closing
text = Text written by child...
```

`os.popen(command[, mode=‘r’[, bufsize=-1]])`:

用于从一个命令打开一个管道

- **command** -- 使用的命令。
- **mode** -- 模式权限可以是 'r'(默认) 或 'w'。
- **bufsize** -- 指明了文件需要的缓冲大小：0意味着无缓冲；1意味着行缓冲；其它正值表示使用参数大小的缓冲（大概值，以字节为单位）。负的bufsize意味着使用系统的默认值，一般来说，对于tty设备，它是行缓冲；对于其它文件，它是全缓冲。如果没有改参数，使用系统的默认值。

# python3 os.path常用方法总结

## 1.路径及目录

`abspath(path)`:返回一个目录的绝对路径

`basename(path)`:返回一个目录的基名即最后的名称

`dirname(path)`:返回一个目录的目录名

`realpath(path)`:返回指定文件的标准路径而不是软连接所指路径

`samefile(f1, f2)`:测试两个路径是否指向同一个文件或目录

`sameopenfile(fp1,fp2)`:测试两个打开的文件是否指向同一个文件

```python
>>> os.path.abspath('Python')
'/home/kobe/Python'
>>> os.path.abspath('test')
'/home/kobe/test'
>>> os.path.abspath('a.out')
'/home/kobe/a.out'

>>> os.path.basename('/home/kobe/Python/code/test.py')
'test.py'
>>> os.path.dirname('/home/kobe/Python/code/test.py')
'/home/kobe/Python/code'
>>> os.path.realpath('test')
'/home/kobe/Python/code/test.txt'

>>> os.path.samefile('test', '/Python/code/test.txt')  # 当前目录下的目录不可以加斜杠
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
  File "/usr/lib/python3.6/genericpath.py", line 97, in samefile
    s2 = os.stat(f2)
FileNotFoundError: [Errno 2] No such file or directory: '/Python/code/test.txt'
>>> os.path.samefile('test', '/home/kobe/Python/code/test.txt')
True

>>> os.path.sameopenfile('test', '/home/kobe/Python/code/test.txt')
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
  File "/usr/lib/python3.6/genericpath.py", line 105, in sameopenfile
    s1 = os.fstat(fp1)
TypeError: an integer is required (got type str)
## 比较的必须是文件标识符，而不是文件对象       
>>> d = open('test')
>>> f = open('Python/code/test.txt')
>>> os.path.sameopenfile(d, f)
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
  File "/usr/lib/python3.6/genericpath.py", line 105, in sameopenfile
    s1 = os.fstat(fp1)
TypeError: an integer is required (got type _io.TextIOWrapper)
    
>>> d = os.open('test', os.O_RDONLY);  f = os.open('Python/code/test.txt', os.O_RDONLY)
>>> os.path.sameopenfile(d, f)
True

```

## 2.文件测试

`exists()`:Test whether a path exists.	Return False for broken symbolic links

`os.getatime(file)`:Return the last access time of a file, reported by os.stat()

`os.getmtime(file)`:Return the last modification time of a file, reported by  os.stat()

`os.path.isabs(s)`:Test whether a path is absolute

`os.path.isdir(s)`:Return true if the pathname refers to an existing directory

`os.path.isfile(s)` `os.path.islink(s)`  (symbolic link)

`os.path.ismount(path)`:是否为挂载点

## 3.路径连接与分割

`os.path.join(a, *p)`

```shell
 Join two or more pathname components, inserting '/' as needed.
If any component is an absolute path, all previous path components
will be discarded.  An empty last part will result in a path that
ends with a separator.
>>> os.path.join(os.getcwd(), 'test')
'/home/kobe/test'
>>> os.path.join(os.getcwd(), )
'/home/kobe'
>>> os.path.join(os.getcwd() )
'/home/kobe'
>>> os.path.join(os.getcwd(),'test', 'gui' )
'/home/kobe/test/gui'
```
`os.path.split(p)`

将路径以最后一个斜线为依据，分割为一个二元元组，最后一个元素可以为空

     Split a pathname.  Returns tuple "(head, tail)" where "tail" is
    everything after the final slash.  Either part may be empty.

`os.splitext(p)`

```
分割文件名，返回由文件名和扩展名组成的元组
Split the extension from a pathname.

Extension is everything from the last dot to the end, ignoring

leading dots.  Returns "(root, ext)"; ext may be empty.

```

## 4.获取当前路径以及上级路径

```python
# 获取当前路径
>>>os.listdir(os.getcwd())
>>>os.listdir(os.curdir)
>>>os.path.abspath(os.curdir)

# 获取上级路径
>>>os.path.abspath(os.path.join('.', '..'))
>>>os.path.abspath(os.path.join(os.getcwd(), '..'))
>>>os.path.dirname(os.getcwd())

# 获取上上级
>>>os.path.abspath(os.path.join(os.getcwd(), "../.."))
```



# python3 命令行操作

## 1.  `os.system(command)`

最为直接的调用可以直接进行命令行操作

## 2.  `subprocess()`模块

```python
import subprocess
subprocess.call("ll", shell=True)
subprocess.call(['ls', '-l'], shell=True)
subprocess.call(['ls', '-l'], shell=0)
```

直接调用命令，返回值即为系统返回，`shell=True`  表示直接在系统ｓｈｅｌｌ中运行Python文档中出于安全考虑，不建议使用shell=True。建议使用Python库来代替shell命令，或使用pipe的一些功能做一些转义。官方的出发点是好的，不过真心麻烦了很多， so....

如果你更关注命令的终端输出，可以这样

> \>>>subprocess.check_output(["echo", "Hello World!"])

> 'Hello World!\n'
>
> \>>> subprocess.check_output("exit 1", shell=True)
> Traceback (most recent call last):
>   ...
> subprocess.CalledProcessError: Command 'exit 1' returned non-zero exit status 1

**可用方法：**

```python
 Older API
    =========
    call(...): Runs a command, waits for it to complete, then returns
        the return code.
    check_call(...): Same as call() but raises CalledProcessError()
        if return code is not 0
    check_output(...): Same as check_call() but returns the contents of
        stdout instead of a return code
    getoutput(...): Runs a command in the shell, waits for it to complete,
        then returns the output
    getstatusoutput(...): Runs a command in the shell, waits for it to complete,
        then returns a (exitcode, output) tuple
```

