#!/use/bin/python3

class Person(object):

    def __init__(self):
        
        self.__x = None
        
    @property    
    def x(self):
        return self.__x
    
    @x.setter
    def x(self, value):
        self.__x = value
    
    @x.deleter
    def x(self):
        del self.__x
        
p = Person()

p.x = 123  # 自动调用 setx 方法
print(p.x)  # 自动调用 getx 方法

del p.x    # 自动调用 delx 方法
print(p.x)