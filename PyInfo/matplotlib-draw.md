# Matplotlib 基本使用

[TOC]

*[matplotlib 图库链接](https://matplotlib.org/gallery.html)* 

## 1.简单曲线绘制

```python
import matplotlib.pyplot as plt
import numpy as np

plt.plot([1, 2, 3, 4], 'r')
plot.ylabel('Simple red line')
plot.show()
```

`plot([x], y, [fmt], data=None, **kwargs)`

`plot([x], y, [fmt], [x2], y2, [fmt2], ..., **kwargs)` 

该函数可以绘制点或者曲线，根据给定的(x, y)坐标值，可以在一个图中绘制两条曲线，可以使用

`Line2D` 的属性作为关键字参数设置更多的特性。

```python
plt.plot(x, y, 'go--', linewidth=2, markersize=12)
plt.plot(x, y, color='green', marker='o', linestyle='dashed', 
           linewidth=2, marksize=12)
```

**Parameters:**\

+ **x, y** : 给定的点的坐标，默认会将这些点连成曲线，使用参数 'b-' 即蓝色的直线 x坐标可以省略，默认为０－n-1

+ **fmt**: 指定点或者连线的模式，可用的颜色以及连线的格式如下

  | character | color       |
  | --------- | ----------- |
  | 'b'       | blue        |
  | 'g'       | green       |
  | 'r'       | red         |
  | 'c'       | cyan(天蓝)  |
  | 'm'       | magenta(紫) |
  | 'y'       | yellow      |
  | 'k'       | black       |
  | 'w'       | white       |

  | character | description |
  | --------- | ----------- |
  | ‘-’       | 直线        |
  | ‘--’      | 虚线        |
  | '-.'      | 短线点相连  |
  | ‘:’       | 小圆点      |


```python
import matplotlib.pyplot as plt
ply.plot([1,2,3,4], [1,4,9,16])
plt.aixs([0, 6, 0, 20])  #  axis()函数接受四个参数，指定横轴以及纵轴的范围

## 控制曲线属性
#使用关键字直接设置属性
plt.plot(x, y, linewidth=2)
#使用setp()命令
line = plt.plot(x, y)
plt.setp(line, color='g', linewidth=2.0)
```

## 2. 创建子图

+ `plt.subplots(`(nrows=1, ncols=1, sharex=False, sharey=False, squeeze=True, subplot_kw=
  None, gridspec_kw=None, **fig_kw)`

> 使用该函数创建子图布局，返回figure对象，以及一个numpy.ndarray，可以根据位置制定相应的子图对象
>
> nrows, ncols:子图的行列布局】
>
> sharex, sharey指定是否共享坐标轴可以的选项{'none', 'all'm, 'row', 'col'}
>
> squeeze:true时返回的坐标对象的多余维度被压缩
>
> subplots_kw:构建每个子图的关键字的字典
>
> gridspec_kw: 构建布局的关键字
>
> **fig_kw: 所有需要传递给`plt.figure()` 的关键字参数

```python
import matplotlib.pyplot as plt 
import numpy as np

np.random.seed(189832)	#使用随机数种子，确保每次运行得到的随机数相同
data = np.random.randn(2, 100)  #生成2*100的二维正态分布的随机数

fig, axs = plt.subplots(2q, 2, figsize=(5, 5))
axs[0, 0].hist(data[0])
axs[1, 0].scatter(data[0], data[1])
axs[0, 1].plot(data[0], data[1])
axs[1, 1].hist2d(data[0], data[1])

plt.show()

#创建极坐标系
  >>> fig, axes = plt.subplots(2, 2, subplot_kw=dict(polar=True))
  >>> axes[0, 0].plot(x, y)
  >>> axes[1, 1].scatter(x, y)

```

+ `plt.subplot(nrows, ncols, index, **kwargs)`

  在当前的表格中创建一个新的坐标图并且返回，其虚拟位置在index(1-n),当函数猎术以及坐标值都小于１０时，可以采用三位数字表示三个参数

> **注意：创建子图时会覆盖之前创建的重叠的子图**
>
> 一个figure中只有一个plot时相当于调用了 plt.subplot(111)  
>
> 创建子图后，如果需要设置每个子图的属性，可以通过
>
> axs[i, j].set_title()设置标题
>
> axs[i, j].set_xlabel() set_ylabel()设置横纵轴标签

```python
Creating a subplot will delete any pre-existing subplot that overlaps
       with it beyond sharing a boundary::
import matplotlib.pyplot as plt
# plot a line, implicitly creating a subplot(111)
plt.plot([1,2,3])
 # now create a subplot which represents the top plot of a grid
 # with 2 rows and 1 column. Since this subplot will overlap the
 # first, the plot (and its axes) previously created, will be removed
plt.subplot(211)
plt.plot(range(12))
plt.subplot(212, facecolor='y') # creates 2nd subplot with yellow background

```

## 3.设置图例	

通过设置子图，生成了坐标轴对象`AxesSuplot` ,利用该对象可以设置相应的图形，再调用`plt.legend()` 进行图样的设置,可以采用三种方式进行调用

### 1. `ax.legend()` 自动填充

被加入到图例中的元素被自动检测，通过对每一个设置了`label`的图形进行检测，直接调用该函数，不需要多余的参数，ax是生成的子图对象(Axessubplot)

```python
f = plt.figure("Legend test")
ax = f.subplot(111)
line = ax.plot([1, 2, 3], label='Line')
ax.legend()
f.show()
```

### 2.给已存在的元素添加标签

```python
ax.plot([1, 2, 3])
ax.legend(['A simple line'])
#可以通过这种方式给一个已经在坐标轴上存在的图像设置标签
#这种方式不提倡使用，因为每个元素及其标签的关系仅靠顺序简单维护，容易引起歧义
```

### 3.`plt.legend((l1, l2, l3), ('label1', 'label2', 'label3'))` 

对标签进行详细定义，为了对每一个元素的标记进行完全控制，可以元素以及标签以可迭代的序列的方式传入，元素一般为`.Artist(lines, patches)` 的序列对象,通过参数 **handles** 传入 ；相对应的标签是string序列

*其他参数：*

+ **loc:** 整数或者字符串，用于确定图例的位置

  | Location string | Location code |
  | --------------- | ------------- |
  | 'best'          | 0             |
  | 'upper right'   | 1             |
  | 'upper left'    | 2             |
  | 'lower left'    | 3             |
  | 'lower right'   | 4             |
  | 'right'         | 5             |
  | 'center left'   | 6             |
  | 'center right'  | 7             |
  | 'lower center ' | 8             |
  | 'upper center'  | 9             |
  | 'center'        | 10            |

+ **ncol:** 图例的列数，默认是１

+ **prop:** 设置图例字体

+ **fontsizse:** 整数，浮点数，或者｛'xx-small', 'x-small', 'small', 'medium', 'large', 'x-large', 'xx-large'｝设置字体大小

+ **fancybox:** 确定是否使用圆边　bool

+ **shadow:** 是否添加图例的阴影

+ **framealpha:** 设置背景透明度　　也可以使用`leg.get_frmae().set_alpha(0.5)`

+ **facecolor:** 设置背景色

+ **mode**: 可以设置为‘expand’ None 设置水平展开

```python
>>> f1 = plt.figure('test')
>>> ax = f1.subplot(2, 1)
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
AttributeError: 'Figure' object has no attribute 'subplot'
>>> ax = f1.subplots(2, 1)
>>> ax
array([<matplotlib.axes._subplots.AxesSubplot object at 0x7efec3fe7c50>,
       <matplotlib.axes._subplots.AxesSubplot object at 0x7efec3879f98>],
      dtype=object)
>>> ax[0].set_title('t1')
Text(0.5,1,'t1')
>>> ax[0].set_xlabel('x1')
Text(0.5,0,'x1')
>>> ax[0].set_ylabel('y1')
Text(0,0.5,'y1')
>>> a = ax[0].plot(t1, f(t1), 'bo', t2, f(t2), 'k')
>>> ax[0].legend(a, 'line t1')
<matplotlib.legend.Legend object at 0x7efec2eb0cc0>
>>> ax[1].set_title('t2')
Text(0.5,1,'t2')
>>> ax[1].set_xlabel('x2')
Text(0.5,0,'x2')
>>> ax[1].set_ylabel('y2')
Text(0,0.5,'y2')
>>> b = ax[1].plot(t2, np.cos(np.pi*2*t2), 'r--', label='line t2')
>>> ax[1].legend()
<matplotlib.legend.Legend object at 0x7efec3fe7630>
>>> f1.show()

```

 ## 4.显示图片

### 1.读入图片

可以使用pillow库将图片读入，并且转换为`np.array`对象，也可以使用`matplotlib.image` 进行读入，但是只可以读取png文件，可以直接获得相应的arrays

```python
import matplotlib.pyplot as plt
import matplotlib.image as mpimg
import numpy as np
img = mpimg.imread('../../doc/_static/stinkbug.png')
print(img)

###利用pillow
>>> img = Image.open('wall.png')
>>> img
<PIL.PngImagePlugin.PngImageFile image mode=RGBA size=2000x1325 at 0x7F9D4D26F710>
>>> import numpy as np
>>> img = np.array(img)
>>> img

```

### 2.图片显示

```python
imshow(X, #类似与array的对象，其shape可以是(m,n)(m,n,3)(m,n,4),或者是PIL.Image
       cmap=None, # 设置颜色参数
       norm=None, # 
       aspect=None, # 方向参数，【auto|equal|标量】
       interpolation=None, # 
       alpha=None, # 标量，指定透明度
       vmin=None, 　# 
       vmax=None, 
       origin=None, 
       extent=None, # 指定图片在坐标系中的位置(left,right,bottom,topqq)
       shape=None, 
       filternorm=1, 
       filterrad=4.0, 
       imlim=None, 
       resample=None, 
       url=None, 
       hold=None, 
       data=None, 
       **kwargs)
```

可以通过对数组进行预处理而进行数据分析，对于一个三（RDB）、四(RGBA)通道图片而言，可以仅获取单一通道的数据进行分析

```python
img[:,:,0][0]　# 获取某一通道的数据



imgplot = plt.imshow(lum_img)　　# plt.imshow(, cmap='...')
imgplot.set_cmap('nipy_spectral') # 类似红外成像　'hot'->暖色调
plt.colorbar()
```

+ 可以使用直方图进行色彩分析

  ```
  imgplot = plt.imshow(lum_img, clim=(0.0, 0.7))
  ```

You can also specify the clim using the returned object

```
fig = plt.figure()
a = fig.add_subplot(1, 2, 1)
imgplot = plt.imshow(lum_img)
a.set_title('Before')
plt.colorbar(ticks=[0.1, 0.3, 0.5, 0.7], orientation='horizontal')
a = fig.add_subplot(1, 2, 2)
imgplot = plt.imshow(lum_img)
imgplot.set_clim(0.0, 0.7)
a.set_title('After')
plt.colorbar(ticks=[0.1, 0.3, 0.5, 0.7], orientation='horizontal')
```