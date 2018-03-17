# CGI初学踩坑

## 1.环境配置

​	采用经典的 **LAMP** 作为服务器，以apache2为例在ubuntu 16.04 上默认安装路径`/etc/apache2`

其配置文件存放在两种类型文件夹`*-available` . `*-enabled` 后者为正在使用的配置文件，被

include 到主配置文件中，前者为备用配置，可以复制到后者中重启ａｐａｃｈｅ生效 

​	在apache2中，ｃｇｉ默认文件夹为　:bug:`/usr/lib/cgi-bin`:bug: 具体配置方式：

```shell
cd /etc/apache2		进入目录查看
cd /mods-available 	ls *cgi*.*		找到可以使用的待配置的文件
其中 "proxy_fcgi.load ", " proxy_scgi.load " 我们暂不用，只需要前三个  " cgid.conf ", " cgid.load ", " cgi.load "  就可以了

```

![image](https://github.com/kobeHub/Hello-world/blob/master/pic/cgi.png)

```shell

接下来，使用 " ln -s *.* *.* " 将这三个文件软链接到 " mods-enabled " 目录下

ln -s /etc/apache2/mods-available/cgid.conf /etc/apache2/mods-enabled/cgid.conf

ln -s /etc/apache2/mods-available/cgid.load /etc/apache2/mods-enabled/cgid.load

ln -s /etc/apache2/mods-available/cgi.load /etc/apache2/mods-enabled/cgi.load
```

## 2.CGI编程大坑

### :battery: utf编码

　　对于python3 进行ＣＧＩ编程时，需要进行编码转换使用`#-*-coding:utf-8-*-`无用，需要显示定义输出格式`import codecs,sys `

 `sys.stdout = codecs.getwriter('utf-8')(sys.stdout.detach())`

 `sys.stdout = codecs.getwriter('utf-8')(sys.stdout.buffer`

### :battery:　改变ＣＧＩ程序目录

```shell
默认在/etc/apache2/conf-enabled/serve-cgi-bin.conf文件中可以设置默认目录：
<Directory "/usr/lib/cgi-bin">(可以是其他自定义文件夹)
   AllowOverride None
   Options +ExecCGI
   Order allow,deny
   Allow from all
</Directory>

```

### :battery: 权限问题

​	对所有ｃｇｉ程序而言如果需要用户调用，均需要更改其执行权限

`chmod 755 file `   对应权限为　`-rwxr-xr-x`　即所有者的读写执行，同组以及其他用户的读执行



​	对与使用wordpress时，若默认网站根目录为/var/www/wordpress 默认对用户可见的文件及文件夹的user group 均为`www-data` 具体可见/etc/apache2/envvars 文件中 

```shell
# Since there is no sane way to get the parsed apache2 config in scripts, some
# settings are defined via environment variables and then used in apache2ctl,
# /etc/init.d/apache2, /etc/logrotate.d/apache2, etc.
export APACHE_RUN_USER=www-data
export APACHE_RUN_GROUP=www-data
# temporary state file location. This might be changed to /run in Wheezy+1
export APACHE_PID_FILE=/var/run/apache2/apache2$SUFFIX.pid
export APACHE_RUN_DIR=/var/run/apache2$SUFFIX
export APACHE_LOCK_DIR=/var/lock/apache2$SUFFIX
# Only /var/log/apache2 is handled by /etc/logrotate.d/apache2.
export APACHE_LOG_DIR=/var/log/apache2$SUFFIX
```

对于上传文件时文件夹的 user .  group必须设置为`www-data` 不可以时ｒｏｏｔ

![cgi-permit](https://github.com/kobeHub/Hello-world/blob/master/pic/cgi-permit.png)

图中temp 文件夹用于存放用户上传的文件，需要更改所有者属性



## 3.常用操作

### :battery: python3　在浏览器输出ＨＴＭＬ

​	以print输出，以＇Content-type:text/html＇作为文件的头标志，输出一个空行后，即可输出ｈｔｍｌ代码，对于get /post方法获取的表单信息，需要引入cgi  cgitb 模块，通过建立FieldStorage 实例获取信息

**在html文件中使用cgi程序需要引入动作**

`action="/cgi-bin/script.py"  method="post"  target="_blank"`

## :battery:使用get Post方法传输数据

`get`方法发送编码后的用户信息到服务端，数据会包含在请求页的url上

`http://localhost/cgi-bin/hello_get.py?name=J.d&url=www.innohub.top&提交=提交`

**`get`方法的注意事项：**

- GET 请求可被缓存
- GET 请求保留在浏览器历史记录中
- GET 请求可被收藏为书签
- GET 请求不应在处理敏感数据时使用
- GET 请求有长度限制
- GET 请求只应当用于取回数据




post 方法传输数据是安全可靠的，一般用于用户密码信息的传送

**补充html常用元素：**

> <input type="_t" name="_n" value="_v"/>

`_t`:可以是　text, submit, checkbox, radio , file

`_n`:相当于键值用于在Ｐｙｔｈｏｎ中的FieldStorage对象获取某个指定选项

`_v`:相当于键值对应的值，可用于输出

> enctype上传文件

```html
<form enctype="multipart/form-data" 
                     action="/cgi-bin/save_file.py" method="post">
   <p>选中文件: <input type="file" name="filename" /></p>
   <p><input type="submit" value="上传" /></p>
</form>
```

> 文件上传

```python
通过ｈｔｔｐ头信息来设置
# HTTP 头部
print ("Content-Disposition: attachment; filename=\"foo.txt\"")
print ()
# 打开文件
fo = open("foo.txt", "rb")

str = fo.read();
print (str)

# 关闭文件
fo.close()
```

> cookies设置

http cookie的发送是通过http头部来实现的，他早于文件的传递，头部set-cookie的语法如下：

```
Set-cookie:name=name;expires=date;path=path;domain=domain;secure 
```

- **name=name:** 需要设置cookie的值(name不能使用"**;**"和"**,**"号),有多个name值时用 "**;**" 分隔，例如：**name1=name1;name2=name2;name3=name3**。

- **expires=date:** cookie的有效期限,格式： expires="Wdy,DD-Mon-YYYY HH:MM:SS"

- ​

- **path=path: **设置cookie支持的路径,如果path是一个路径，则cookie对这个目录下的所有文件及子目录生效，例如： path="/cgi-bin/"，如果path是一个文件，则cookie指对这个文件生效，例如：path="/cgi-bin/cookie.cgi"。

- **domain=domain:** 对cookie生效的域名，例如：domain="www.runoob.com"

- **secure:** 如果给出此标志，表示cookie只能通过SSL协议的https服务器来传递。

- cookie的接收是通过设置环境变量HTTP_COOKIE来实现的，CGI程序可以通过检索该变量获取cookie信息。


