# Django 项目测试

[TOC]

##1. 概述

测试在项目开发中有着举足轻重的作用,可以提供有效便捷的代码检查.测试可以分为不同的level,有些测试关注于小的细节,比如一个model是否返回所需要的值,有的关注于项目的整体操作,比如输入用户的操作序列是否得到期待的结果.

Django提供了自动测试系统,可以通过编写测试代码提高代码可用性.有的程序猿遵守`test-driven devlopment` 即测试驱动开发,测试代码先于项目代码实现,,测试驱动开发简单地把所有问题放在了测试用例中,如果在开发过程中的代码实现不能满足测试的需求,系统就会报错,从而简化了寻找系统bug的过程,使用测试的好处如下

+ **save your time** 

  对于较为复杂的应用程序,在不同的组分之间可能有较多的耦合关联,一个元素的改变可能造成应用程序难以预料的行为,检查这些依赖需要大量的时间,但是通过自动测试,可以在瞬间玩策划那个对于各种依赖的检查

+ **Tests don't identify problems, they prevent them**

  测试并不是开发中的负面因素,很多认为无法意识到的错误可以被自动检测

+ **Tests make your code more attractive**

+ **Tests help teams work together**

## 2. 基本测试技巧

​    在Django项目中的每一个app下都有一个tests.py文件,该app中所有的行为测试都可以放在该文件中,通过对视图或者行为建立测试函数,运行`python3 manage.py test app` 即可得到测试结果

```python
import datetime

from django.test import TestCase
from django.utils import timezone

from .models import Question


class QuestionModelTests(TestCase):

    def test_was_published_recently_with_future_question(self):
        """
        was_published_recently() returns False for questions whose pub_date
        is in the future.
        """
        time = timezone.now() + datetime.timedelta(days=30)
        future_question = Question(pub_date=time)
        self.assertIs(future_question.was_published_recently(), False)
        
    def test_was_published_recently_with_old_question(self):
    	"""
    	was_published_recently() returns False for questions whose pub_date
    	is older than 1 day.
    	"""
    	time = timezone.now() - datetime.timedelta(days=1, seconds=1)
    	old_question = Question(pub_date=time)
    	self.assertIs(old_question.was_published_recently(), False)

    def test_was_published_recently_with_recent_question(self):
        """
    	was_published_recently() returns True for questions whose pub_date
    	is within the last day.
    	"""
    	time = timezone.now() - datetime.timedelta(hours=23, minutes=59, 				seconds=59)
    	recent_question = Question(pub_date=time)
    	self.assertIs(recent_question.was_published_recently(), True)
        
```

对于未来问题的测试,要求该函数返回False,可是在最初的实现中并没有得到预期的结果,所以需要进行更改

对于涉及时间点的测试,一般要包含,过去,现在,未来,以及临界点的测试

### Shell test

django提供了一个用户模拟端,进行用户行为的模拟,可以在tests.py或者shell中使用

```shell
>>> from django.test.utils import setup_test_environment
>>> setup_test_environment()




>>> from django.test import Client
>>> # create an instance of the client for our use
>>> client = Client()





>>> # get a response from '/'
>>> response = client.get('/')
Not Found: /
>>> # we should expect a 404 from that address; if you instead see an
>>> # "Invalid HTTP_HOST header" error and a 400 response, you probably
>>> # omitted the setup_test_environment() call described earlier.
>>> response.status_code
404
>>> # on the other hand we should expect to find something at '/polls/'
>>> # we'll use 'reverse()' rather than a hardcoded URL
>>> from django.urls import reverse
>>> response = client.get(reverse('polls:index'))
>>> response.status_code
200
>>> response.content
b'\n    <ul>\n    \n        <li><a href="/polls/1/">What&#39;s up?</a></li>\n    \n    </ul>\n\n'
>>> response.context['latest_question_list']
<QuerySet [<Question: What's up?>]>
```

**<u>setup\_test\_environment()</u>** 安装了一个template renderer ,提供了访问**response.context** 的方法,该方法并不会创建一个test database

## 3. views test

依照之前的实现,所有在未来发布的问题也都会在list展示,所以对views.py进行更改

```python
def get_queryset(self):
    """
    Return the last five published questions (not including those set to be
    published in the future).
    """
    return Question.objects.filter(
        pub_date__lte=timezone.now()
    ).order_by('-pub_date')[:5]
```

**pub_date__lte**表示小于等于该筛选值

## 4. 测试越多越好

可能测试代码会多余项目代码,但是是值得的,虽然会有多余的测试,但是不需要减少,测试代码只需要添加,因为进行自动测试的时间很短,效果却十分显著,一些基本的守则:

+ 对于每一个model view设置一个单独的测试类
+ 每一个想要测试的情况设置一个单独的测试方法\
+ 方法名描述测试行为