# Export to wheel or egg-info

[TOC]

## 1. 文件打包

提高程序的可复用性是python的生命，[The Python Package Index](https://pypi.org/) (PyPI) 有大量的包可供安装,可以使用已有的包组合起来构建自己的web 程序,同时可以将已经写好的app进行打包,以wheel或者egg形式发布,通过pip进行安装

> :bulb: **Package? App?**
>
> python的包提供了一种组织相关文件以便于进行代码复用的方法,每个包里包含了多个文件,即通常的module
>
> 一个包里的文件可以通过`import foo.bar` `from foo import bar` 的形式引入到目标代码中去,每个包必须以`__init__.py` 文件进行标识
>
> 一个django app是一个django项目所用的实现特定功能的模块,可以讲其导出为二进制包 

### Wheel vs Egg

Wheel和Egg都是为了进行包安装而用的打包格式,不需要进行 build compilation 可以之间用于测试或者生产环境中,egg格式最早在2004年被引入,wheel在2012年引入,现在wheel已经被作为标准的`build` `binary` 包格式

- Wheel有一个官方的PEP427来定义，而Egg没有PEP定义。
- Wheel是一种分发格式，即打包格式。而Egg既是一种分发格式，也是一种运行时安装的格式，并且是可以被import的。
- Wheel文件不会包含.pyc文件
- Wheel使用和PEP376兼容的.dist-info目录，而Egg使用.egg-info目录。
- Wheel有着更丰富的命名规则。
- Wheel是有版本的。每个Wheel文件都包含wheel规格的版本和打包它的实现。
- Wheel在内部被sysconfig path type管理，因此转向其他格式也更容易。

## 2. 导出包

### 2.1 将app文件夹移到相应的打包位置

将polls文件夹移动到`django-polls` ,注意命名格式,不要与pypi中已经存在的包名重复,对于django项目使用django前缀,不要使用django中默认的已有包`contrib` `admin` `auth` `message` 

### 2.2 创建README.rst

```rst
=====
Polls
=====

Polls is a simple Django app to conduct Web-based polls. For each
question, visitors can choose between a fixed number of answers.

Detailed documentation is in the "docs" directory.

Quick start
-----------

1. Add "polls" to your INSTALLED_APPS setting like this::

    INSTALLED_APPS = [
        ...
        'polls',
    ]

2. Include the polls URLconf in your project urls.py like this::

    path('polls/', include('polls.urls')),

3. Run `python manage.py migrate` to create the polls models.

4. Start the development server and visit http://127.0.0.1:8000/admin/
   to create a poll (you'll need the Admin app enabled).

5. Visit http://127.0.0.1:8000/polls/ to participate in the poll.
```

### 2.3 创建LICENSE

```
可以是BSD MIT ...
```

### 2.4 setup.py

```python
import os
from setuptools import find_packages, setup

with open(os.path.join(os.path.dirname(__file__), 'README.rst')) as readme:
    README = readme.read()

# allow setup.py to be run from any path
os.chdir(os.path.normpath(os.path.join(os.path.abspath(__file__), os.pardir)))

setup(
    name='django-polls',
    version='0.1',
    packages=find_packages(),
    include_package_data=True,
    license='BSD License',  # example license
    description='A simple Django app to conduct Web-based polls.',
    long_description=README,
    url='https://www.example.com/',
    author='Your Name',
    author_email='yourname@example.com',
    classifiers=[
        'Environment :: Web Environment',
        'Framework :: Django',
        'Framework :: Django :: X.Y',  # replace "X.Y" as appropriate
        'Intended Audience :: Developers',
        'License :: OSI Approved :: BSD License',  # example license
        'Operating System :: OS Independent',
        'Programming Language :: Python',
        'Programming Language :: Python :: 3.5',
        'Programming Language :: Python :: 3.6',
        'Topic :: Internet :: WWW/HTTP',
        'Topic :: Internet :: WWW/HTTP :: Dynamic Content',
    ],
)
```

 ### 2.6 MANIFEST.in

```
include LICENSE
include README.rst
recursive-include polls/static *
recursive-include polls/templates *
recursive-include polls/docs *
```

## 3. 导出

```shell
python3 setup.py sdist   # 在dist文件夹得到 *.tar.gz 压缩包,是egg格式
python3 setup.py bdist_wheel    # build 得到可进行build文件, dist得到.whl二进制文件
```

