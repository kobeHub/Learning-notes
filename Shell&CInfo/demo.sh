#!/bin/sh 

# shell脚本注释只可以用#,没有多行注释

################定义变量#######################
#定义变量时,变量名与等号间不可以有空格
#基本数据类型只有数字和字符串

#对于字符串类型,可以采用单引号或者双引号,或者不使用引号
#    单引号中的任何字符都会原样输出,里面的变量名是无效的,在单引号中不可以出现单引号
#    双引号中可以输出变量,可以进行转义输出
user="Kobe"
ano_user=leborn
third='baller'

#使用变量的值时需要用$,可以加{}用于标识变量的范围
out1="The first players name: '$user'"
echo "when you print a str, cast with \"\", use func echo"
echo It\'s gone be ${ano_user} is the second player.
echo print str with no \'\'  只是用echo函数,会输出一个空行

echo 
echo connect two string: '${user} ${ano_user}' ${user} ${ano_user}
echo "\n\n"Now test echo function to print multiple rows strings: \
    a \\ is necessary to print such a format string, \
    and it won\'t print 

con_str=${user}' and '${ano_user}
echo String connect test:${con_str}
echo The length of the String:${#con_str}

state="alibaba is a great company"
#echo ${state:1:7}

echo `expr index "$state" is`


#注意:shell流程控制不可为空,若无else则不写
# shell中的[是一个可执行程序,所以在左圆括号后面必须加空格
if `ls -l /usr/bin/[`; then echo "[ is a exec file!"; fi 


