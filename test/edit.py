import keyword
import re

target = keyword.kwlist
print(target)
partern = re.compile(r'[a-z]', re.I)
s = ''
l = ''
lines = []

with open('hello.py', 'r') as f:
	lines = f.readlines()

with open('hello.py', 'w+') as file:
    print(lines)
    for one in lines:
        m = partern.findall(one)
        for i in one:
            if partern.match(i) is None:
                if l not in target:
                    l = l.upper()
                s += l
                l = ''
                s += i
            else:
                l += i
    file.write(s)
