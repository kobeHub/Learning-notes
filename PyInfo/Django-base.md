# Django base

[TOC]

## 1. 创建项目

```shell
django-admin startproject server_test
```

```powershell
# 基本文件树如下
server_test/
    manage.py
    server_test/
        __init__.py
        settings.py
        urls.py
        wsgi.py
        
####################################################
# manage.py 基本的命令行工具
# inner server_test文件夹是真正的django项目
# __init__.py 标识python package
# setting.py 项目配置信息
# urls.py 项目url的声明,是一个内容表
# wsgi.py uwsgi服务的入口
```

**收集静态文件**

在setting.py文件中添加

```python
STATIC_ROOT = os.join(BASE_DIR, 'static/')
```

然后执行

```shell
python3 manage.py collectstatic
```

根据nginx的配置,在主目录下创建script/文件夹,创建uwsgi.ini uwsgi.log uwsgi.sock 文件,创建media文件夹,存放静态媒体信息

## 2.创建第一个app

Django是基于MTV Design Partten *Model Template and View* 

+ Model : 数据模型,构造了一个数据接口,包含了数据的所有特征,可以获取数据,验证数据,控制数据的行为可以在忽略底层数据库的构造情况下进行数据操作
+ Template: 显示层,包含了显示相关的决策:网页上的元素该如何显示
+ View: 模型和模板之间的桥梁,处理逻辑层面 

在一个django项目中可以包含多个app,同时一个app可以包含在多个项目中,创建的项目位于根目录下

```shell
python manage.py startapp polls
```

### 创建第一个view

在polls/view.py中添加内容:

```python
rom django.http import HttpResponse


def index(request):
    return HttpResponse("Hello, world. You're at the polls index.")

```

为了调用该view,需要将其映射到一个url上,在app目录下创建urls.py,引入该view,同时将app的urls引入项目的urls中

```python
from django.urls import path

from . import views

urlpatterns = [
    path('', views.index, name='index'),
]
```

```python
from django.contrib import admin
from django.urls import include, path

urlpatterns = [
    path('polls/', include('polls.urls')),
    path('admin/', admin.site.urls),
]
```

```shell
# 运行
python3 manage.py runserver
# 现在可以在localhost:8000/polls/ 得到该view
```

## 3.database config

django默认使用的数据库是sqllite,在配置文件中指定其位置在项目主目录下,还支持oracle mysql postgresql

在`./server_test/setting.py`文件中还可以设置时区:

```python
TIME_ZONE = 'Asia/Shanhai'
USE_TZ = True
```

配置文件中的`installed_apps` 是该项目需要使用的app,django app是类似于插件的方式安装的,需要显式添加到配置文中去

```shell
polls.app.PollsConfig, # 位于polls/app.py中
```

一些app至少需要一个table,所以需要进行数据库迁移

```shell
python3 manage.py migrate
# 执行该命令会检查`INSTALLED_APP`中所需的数据库的表是否已经创建,对于每一个app创建其model所需的数据模式的表
```

### 创建models

在polls/models.py文件中添加

```python
from django.db import models


class Question(models.Model):
    question_text = models.CharField(max_length=200)
    pub_date = models.DateTimeField('date published')


class Choice(models.Model):
    question = models.ForeignKey(Question, on_delete=models.CASCADE)
    choice_text = models.CharField(max_length=200)
    votes = models.IntegerField(default=0)
```

每一个数据模型都有相应的变量,作为数据模式,会自动添加id作为主码,注意不可以用self来标注模式,否则模式下只有id一项属性,变量名即为属性名,后面是数据类型,同时可以添加外码,表示每一个chioce都与一个question对应

在配置文件中添加app后即可进行数据库的迁移

```
python3 manage.py makemigrations polls
```

 通过执行该命令告诉django对model进行了更改,数据库的模式发生了变化,所以产生了一个控制迁移的文件polls/migrations/0001_initial.py 里面记录了迁移的步骤,可以通过命令得到相应的sql

```Python
python manage.py sqlmigrate polls 0001
```

可以通过`check`命令检查是否存在问题.此时并没有进行数据库的实际改动

```shell
python3 manage.py migrate
```

### 命令行查看数据库

```shell
python3 manage.py shell
>>> from polls.models import Choice, Question  # Import the model classes we just wrote.

# No questions are in the system yet.
>>> Question.objects.all()
<QuerySet []>

# Create a new Question.
# Support for time zones is enabled in the default settings file, so
# Django expects a datetime with tzinfo for pub_date. Use timezone.now()
# instead of datetime.datetime.now() and it will do the right thing.
>>> from django.utils import timezone
>>> q = Question(question_text="What's new?", pub_date=timezone.now())

# Save the object into the database. You have to call save() explicitly.
>>> q.save()

# Now it has an ID.
>>> q.id
1

# Access model field values via Python attributes.
>>> q.question_text
"What's new?"
>>> q.pub_date
datetime.datetime(2012, 2, 26, 13, 0, 0, 775217, tzinfo=<UTC>)

# Change values by changing the attributes, then calling save().
>>> q.question_text = "What's up?"
>>> q.save()

# objects.all() displays all the questions in the database.
>>> Question.objects.all()
<QuerySet [<Question: Question object (1)>]>

```

```shell
>>> from polls.models import Choice, Question

# Make sure our __str__() addition worked.
>>> Question.objects.all()
<QuerySet [<Question: What's up?>]>

# Django provides a rich database lookup API that's entirely driven by
# keyword arguments.
>>> Question.objects.filter(id=1)
<QuerySet [<Question: What's up?>]>
>>> Question.objects.filter(question_text__startswith='What')
<QuerySet [<Question: What's up?>]>

# Get the question that was published this year.
>>> from django.utils import timezone
>>> current_year = timezone.now().year
>>> Question.objects.get(pub_date__year=current_year)
<Question: What's up?>

# Request an ID that doesn't exist, this will raise an exception.
>>> Question.objects.get(id=2)
Traceback (most recent call last):
    ...
DoesNotExist: Question matching query does not exist.

# Lookup by a primary key is the most common case, so Django provides a
# shortcut for primary-key exact lookups.
# The following is identical to Question.objects.get(id=1).
>>> Question.objects.get(pk=1)
<Question: What's up?>

# Make sure our custom method worked.
>>> q = Question.objects.get(pk=1)
>>> q.was_published_recently()
True

# Give the Question a couple of Choices. The create call constructs a new
# Choice object, does the INSERT statement, adds the choice to the set
# of available choices and returns the new Choice object. Django creates
# a set to hold the "other side" of a ForeignKey relation
# (e.g. a question's choice) which can be accessed via the API.
>>> q = Question.objects.get(pk=1)

# Display any choices from the related object set -- none so far.
>>> q.choice_set.all()
<QuerySet []>

# Create three choices.
>>> q.choice_set.create(choice_text='Not much', votes=0)
<Choice: Not much>
>>> q.choice_set.create(choice_text='The sky', votes=0)
<Choice: The sky>
>>> c = q.choice_set.create(choice_text='Just hacking again', votes=0)

# Choice objects have API access to their related Question objects.
>>> c.question
<Question: What's up?>

# And vice versa: Question objects get access to Choice objects.
>>> q.choice_set.all()
<QuerySet [<Choice: Not much>, <Choice: The sky>, <Choice: Just hacking again>]>
>>> q.choice_set.count()
3

# The API automatically follows relationships as far as you need.
# Use double underscores to separate relationships.
# This works as many levels deep as you want; there's no limit.
# Find all Choices for any question whose pub_date is in this year
# (reusing the 'current_year' variable we created above).
>>> Choice.objects.filter(question__pub_date__year=current_year)
<QuerySet [<Choice: Not much>, <Choice: The sky>, <Choice: Just hacking again>]>

# Let's delete one of the choices. Use delete() for that.
>>> c = q.choice_set.filter(choice_text__startswith='Just hacking')
>>> c.delete()
```

## 4.创建管理员

```
python3 manage.py createsuperuser
Username:xx

```



