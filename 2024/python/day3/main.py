import re
data = open('input.txt').read().strip()

# part 1
p = r'mul\(\d+,\d+\)'
m = re.findall(p, data)
ans = 0
for i in m:
    x, y = map(int, re.findall(r'\d+', i))
    ans += int(x) * int(y)
print(ans)

# part 2
p = r'(?:do|don\'t)\(\)|mul\(\d+,\d+\)'
m = re.findall(p, data)

ans = 0
do = True
for i in m:
    match(i):
        case "do()" : do = True
        case "don't()": do = False
        case _:
            x, y = map(int, re.findall(r'\d+', i))
            ans += do * (x * y)
print(ans)
