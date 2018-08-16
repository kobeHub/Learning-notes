# [使用ngnix和uWSGI配置Django](http://uwsgi-docs.readthedocs.io/en/latest/tutorials/Django_and_nginx.html?highlight=django)

> `Django` 是一个python web框架,可以进行高效快速的开发;`nginx` 是一个高性能HTTP服务器及反向代理,又是一个IMAP/POP3代理服务器;`WSGI` (web server gateway interface),基于python标准的网络服务网关接口,`uwsgi` 是其一种实现,通过uwsgi产生一个unix socket,用于向web server反馈请求
>
> 通过nginx接受所有的web请求,并且处理静态文件,然后把非静态请求通过uwsgi传递给django,从而完成一次web请求.

```
the web client <-> web server <-> socket <-> uwsgi <-> django
```

## 1.安装

nginx可以通过apt安装,uwsgi django通过pip安装

## 2.django project and uwsgi

### 创建django项目

```shell
django-admin startproject project_name 
# 其结构树如下
 tree
.
|-- HelloWorld
|   |-- __init__.py
|   |-- settings.py
|   |-- urls.py
|   `-- wsgi.py
`-- manage.py
```

+ 最外层目录是项目的容器,内层`manage.py` 是用来与项目交互的命令行工具
+ __init\__.py:空文件,用于标识该目录是一个python包
+ setting.py该项目的配置
+ urls.py该项目的声明,一份由django驱动的网站目录
+ wsgi.py:WSGI兼容web服务器的入口

### 运行django

```shell
python3 manage.py runserver 0.0.0.0:8000
```

测试uwsgi,通过编写以下测试文件

```python
# test.py
def application(env, start_response):
    start_response('200 OK', [('Content-Type','text/html')])
    return [b"Hello World"] # python3
    #return ["Hello World"] # python2
```

调用命令:

```shell
uwsgi --http :8000 --wsgi-file test.py
```

即可以在指定ip port中通过浏览器得到数据返回

**使用wsgi运行django项目:**

```shell
cd /../hello
uwsgi --http :8000 --module hello.wsgi
```

+ --module参数指定特定的django项目中的wsgi模块

现在通过uwsgi接受所有客户端请求,并且转交给django处理,uwsgi扮演一个中介的角色

```shell
the web client <-> uuWSGI <-> Django
```

## 3.Basic nginx

```shell
apt-get install nginx
/etc/init.d/nginx start
```

nginx配置文件与apache2类似,位于/etc/nginx/文件夹中,根配置文件是 `nginx.conf` 里面include 了/etc/nginx/conf.d/*.conf    /etc/nginx/sites-enabled/\*  默认配置下,sites-enabled文件夹下的default文件注明了监听 80 以及[::]:80端口,设置新的配置文件时如果使用80端口要把该配置文件全部注释.

添加新的配置文件时,可以在sites-avaiable文件夹中添加, my-sites.conf 然后硬链接到sites-enabled中

```shell
ln -s /etc/nginx/sites-avaiable/my-sites.conf /etc/nginx/sites-enabled/
```

重启即可生效

## 配置文件

在django项目的目录下touch uwsgi_params文件,用于nginx和uwsgi的通信

[https://github.com/nginx/nginx/blob/master/conf/uwsgi_params](https://github.com/nginx/nginx/blob/master/conf/uwsgi_params) 

```shell
# mysite_nginx.conf

# the upstream component nginx needs to connect to
upstream django {
    # server unix:///path/to/your/mysite/mysite.sock; # for a file socket
    server ip:8001; # for a web port socket (we'll use this first)
}

# configuration of the server
server {
    # the port your site will be served on
    listen      8000;
    # the domain name it will serve for
    server_name example.com; # substitute your machine's IP address or FQDN
    charset     utf-8;

    # max upload size
    client_max_body_size 75M;   # adjust to taste

    # Django media
    location /media  {
        alias /path/to/your/mysite/media;  # your Django project's media files - amend as required
    }

    location /static {
        alias /path/to/your/mysite/static; # your Django project's static files - amend as required
    }

    # Finally, send all non-media requests to the Django server.
    location / {
        uwsgi_pass  django;
        include     /path/to/your/mysite/uwsgi_params; # the uwsgi_params file you installed
    }
}

```

###port简单尝试

同时使用uwsgi服务,执行test.py文件

```shell
uwsgi --socket :8001 --wsgi-file test.py
```

由nginx接收到的客户端的非静态请求通过socket,传递给uwsgi,uwsgi调用python interperator执行该脚本

```shell
wen client <-> nginx <-> socket <-> uwsgi <-> python
```

此时直接访问`http:ip:8000` 既可以得到test.py的执行结果

### 使用unix socket代替port

更改`my-sites.conf` 

```shell
server unix:/server/../script/uwsgi.sock
```

```shell
uwsgi --socket uwsgi.sock --wsgi-file test.py
```

这时候通过8000端口的请求会通过unix socket传递给uwsgi

可以执行django项目

```shell
uwsgi --socket uwsgi.sock --module hello.wsgi --chmod-socket 664
```

## 最终配置

通过在django项目的./script文件中的uwsgi.ini文件配置uwsgi服务

```shell
# mysite_uwsgi.ini file
[uwsgi]

# Django-related settings
# the base directory (full path)
chdir           = /path/to/your/project
# Django's wsgi file
module          = project.wsgi
# the virtualenv (full path)
home            = /path/to/virtualenv

# process-related settings
# master
master          = true
# maximum number of worker processes
processes       = 10
# the socket (use the full path to be safe
socket          = /path/to/your/project/mysite.sock
# ... with appropriate permissions - may be needed
# chmod-socket    = 664
# clear environment on exit
vacuum          = true
```

```shell
uwsgi --ini wusgi.ini
```

这时候会发现uwsgi服务已经启动,并且根据设置的process数量fork出了相应数量的子进程

```shell
root@vultr:/etc/nginx# ps -ef | grep uwsgi
root      4400     1  0 Aug06 ?        00:00:21 uwsgi --ini uwsgi.ini
root      4402  4400  0 Aug06 ?        00:00:00 uwsgi --ini uwsgi.ini
root      4403  4400  0 Aug06 ?        00:00:00 uwsgi --ini uwsgi.ini
root      4404  4400  0 Aug06 ?        00:00:00 uwsgi --ini uwsgi.ini
root      4405  4400  0 Aug06 ?        00:00:00 uwsgi --ini uwsgi.ini
root      4406  4400  0 Aug06 ?        00:00:00 uwsgi --ini uwsgi.ini
root      4407  4400  0 Aug06 ?        00:00:00 uwsgi --ini uwsgi.ini
root      4408  4400  0 Aug06 ?        00:00:00 uwsgi --ini uwsgi.ini
root      4409  4400  0 Aug06 ?        00:00:00 uwsgi --ini uwsgi.ini
root      4410  4400  0 Aug06 ?        00:00:00 uwsgi --ini uwsgi.ini
root      4411  4400  0 Aug06 ?        00:00:00 uwsgi --ini uwsgi.ini
root     29654 29339  0 09:23 pts/0    00:00:00 grep --color=auto uwsgi
```

**log位置:**

```shell
nginx /var/log/nginx
uwsgi /project/script/uwsgi.log
uwsgi-emporer /var/log/uwsgi-emperor.log
```

### uwsgi 停止服务

```shell
uwsgi --stop uwsgi.pid
uwsgi --ini uswgi.ini 
```

