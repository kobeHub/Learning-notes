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
I
```

## 5.添加更多views

可以在已经创建的app中加入更多的views `polls/views.py` 文件中添加相应的函数实现,然后在新创建的`poll/urls.py` 中添加

```python
from django.urls import path

from . import views

urlpatterns = [
    # ex: /polls/
    path('', views.index, name='index'),
    # ex: /polls/5/  通过该url将question_id 传递给views.py中的detail
    path('<int:question_id>/', views.detail, name='detail'),
    # ex: /polls/5/results/
    path('<int:question_id>/results/', views.results, name='results'),
    # ex: /polls/5/vote/
    path('<int:question_id>/vote/', views.vote, name='vote'),
]
```

当请求获取`pllos/22` django会加载根项目的urls.py 由于在里面的`urlpatterns=[path('polls/', include(polls.urls))]` 将app `polls`中的urls记载到里面,回进入`polls/url.py` 中查找,就可以找到`detail` view 

## 6.创建模板

为了避免对视图进行硬编码,可以使用模板进行定义,在polls文件夹下创建templates文件夹,django会自动在该文件夹寻找模板.Django项目关于模板的设置在默认`setting.py` 文件中,描述了项目如何记载呈送模板.定义了默认的`template backend ` 将`APP_DIRS=True` ,会在每个`INSTALLED_APP`中的子文件夹寻找templates文件夹作为模板文件夹

> 所以在polls app中的模板要放在 `poll/templates/polls/` 文件夹中

```html
{% if latest_question_list %}
    <ul>
    {% for question in latest_question_list %}
        <li><a href="/polls/{{ question.id }}/">{{ question.question_text }}</a></li>
    {% endfor %}
    </ul>
{% else %}
    <p>No polls are available.</p>
{% endif %}
```

> + 所有的判断和循环语句都需要使用{%%}进行嵌套
> + 用{% ***** %} 表示条件结束
> + 输出变量值时需要用{{ var }}
> + 其他语法根据html

### 添加到view

```python
from django.http import HttpResponse
from django.template import loader
from django.http import Http404

from .models import Question


def index(request):
    latest_question_list = Question.objects.order_by('-pub_date')[:5]
    template = loader.get_template('polls/index.html')
    context = {
        'latest_question_list': latest_question_list,
    }
    return HttpResponse(template.render(context, request))

def detail(request, question_id):
    try:
        question = Question.objects.get(pk=question_id)
    except Question.DoesNotExist:
        raise Http404("Question does not exist")
    return render(request, 'polls/detail.html', {'question': question})

def detail(request, question_id):
    question = get_object_or_404(Question, pk=question_id)
    return render(request, 'polls/detail.html', {'question': question})
```

> 获取模板的方式:
>
> + 使用loader加载模板,然后使用模板调用render函数,将所需要的参数以dict形式传递
> + 使用`shortcut.render ` 接受三个参数,httprequest  templates cointext,返回一个根据给定模板和内容的`HttpResponse` 对象
>
> 获取404:
>
> + 可以使用抛出异常的方式得到404
> + 使用shortcut.get_object_or_404(), 给定对象model 以及选择条件

### Templates urlspolls

+ 当使用一个连接以`<a href="/polls/{{question.id}/">...</a>` 的形式出现在模板中时,采用的是硬编码的形式,当项目中的url变化时,需要进行大量的更改,不符合复用原则,可以使用在polls/url.py中定义的 url     `{% url %}` ,如果需要更改url时只需要在urls.py中修改,不需要改动templates

+ **命名域的url:**

  当一个项目有很多app时,可能出现重名状况,所以需要在url前添加命名域,这时需要在`polls/urls.py`中添加 `app_name='polls' ` 既可以在url中指定命名域

  ```html
  <li><a href="{% url 'detail' question.id %}">{{ question.question_text }}</a></li>
  
  <li><a href="{% url 'polls:detail' question.id %}">{{ question.question_text }}</a></li>
  ```

  

  

## 7.使用生成视图,减少代码量

**detail() result()** 视图较为简单,直接提供一个数据,index也只提供一个列表.这些视图展示了web开发的一个基本样例:根据url参数从数据库获取数据并显示,加载一个模板,并且返回一个`rendered` template  ,django提供了一个便捷的方式去使用views ------`generic`  

+ 修改URL conf
+ 更改views

主要使用了`generic.ListView` `generic.DetailView` ,以这两个类作为基类,需要注意的是`DetaliView` 需要从url获取的主码是`pk` 所以需要将其进行更改

```python
from django.urls import path

from . import views

app_name = 'polls'
urlpatterns = [
    path('', views.IndexView.as_view(), name='index'),
    path('<int:pk>/', views.DetailView.as_view(), name='detail'),
    path('<int:pk>/results/', views.ResultsView.as_view(), name='results'),
    path('<int:question_id>/vote/', views.vote, name='vote'),
]
```

对于views进行更改时,这两个类需要提供一个`model` 参数指明所使用的数据模型, `template_name` 参数指明模板

```python
from django.http import HttpResponseRedirect
from django.shortcuts import get_object_or_404, render
from django.urls import reverse
from django.views import generic

from .models import Choice, Question


class IndexView(generic.ListView):
    template_name = 'polls/index.html'
    context_object_name = 'latest_question_list'

    def get_queryset(self):
        """Return the last five published questions."""
        return Question.objects.order_by('-pub_date')[:5]


class DetailView(generic.DetailView):
    model = Question
    template_name = 'polls/detail.html'


class ResultsView(generic.DetailView):
    model = Question
    template_name = 'polls/results.html'


def vote(request, question_id):
    ... # same as above, no changes needed.
```

**Attention:**

之前的模板中使用`lasted_question_list` 作为参数列表,对于DetailView,再使用Question model 时会自动生成内容变量*question_list* ,所以必须对该参数进行改写

## 8. 使用静态文件

通常使用的静态文件包括 images, js, css `django.contrib.staticfiles` 收集每个app的静态文件到特定文件夹,该app包含在`INSTALLED_APPS`中.django的`STATICFILES_FINDERS` 设置了寻找静态文件的方式默认是

```python
[
    'django.contrib.staticfiles.finders.FileSystemFinder',
    'django.contrib.staticfiles.finders.AppDirectoriesFinder',
]
```

首选项是在`STATICFILES_DIR` ,可以在项目的setting.py中设置`STATICFILES_DIRS=[os.path.join(BASE_DIR, 'static/')]` ;如果没有该设置,使用第二个默认选项,会在每个app的一个
子文件夹static中寻找静态文件,而最常用的文件结构是在static文件夹下再建立`app/` 放置静态文件

可以使用样式表(polls/static/polls/style.css)

```css
ul {
    color green;
}
body {
    backgoround : black url('images/background.jpg') no-repeat;
}
```

然后在模板中引入css:

```html
{%load static%}
<link rel="stylesheet" type="text/css" href="{% static 'polls/style.css' %}">
<!--该标签产生静态文件的绝对路径-->
```



## 9.定制管理员站点 admin

:electric_plug: **关于模板文件组织:**

和static files类似,所有的模板可以放在同一个文件夹中,但是分属于不同app的模板和静态文件要放在每个子文件夹中,可以在醒目的根目录下创建`templates/admin` 来放置admin站点的模板文件,需要在setting文件中设置

```python
TEMPLATES = [
    {
        'BACKEND': 'django.template.backends.django.DjangoTemplates',
        'DIRS': [os.path.join(BASE_DIR, 'templates')],
        'APP_DIRS': True,
        'OPTIONS': {
            'context_processors': [
                'django.template.context_processors.debug',
                'django.template.context_processors.request',
                'django.contrib.auth.context_processors.auth',
                'django.contrib.messages.context_processors.messages',
            ],
        },
    },
]
```

可以修改base_site.html模板,更改站点header,对于admin中的详细内容需要在`polls/admin.py` 中进行修改

```python
from django.contrib import admin
from .models import Question, Choice


class ChoiceInline(admin.TabularInline):
    """
    作为Question的内连对象,可以继承自admin.StackedInline 但是相对与需要较大的页面空间
    使用这种方式所需的页面空间较小
    """
    model = Choice
    extra = 3



class QuestionAdmin(admin.ModelAdmin):
    # 指明父类参数, fields 列表形式, fieldsets 每个元素代表一个模块,模块内是标题,内容
    fieldsets = [
        (None, {'fields':['question_text']}),
        ('Date information', {'fields':['pub_date']}),
    ]
    inlines = [ChoiceInline]

    #在全部问题页面的列表设置,包含了三个纵列
    list_display = ('question_text', 'pub_date', 'published_recently')
    # 按日期筛选
    list_filter = ['pub_date']
    # 添加搜索框
    search_fields = ['question_text']



admin.site.register(Question, QuestionAdmin)
admin.site.register(Choice)

```

