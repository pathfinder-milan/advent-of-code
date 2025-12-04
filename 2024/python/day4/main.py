from collections import defaultdict, Counter
import re

data = open('input.txt').read().strip()
l = data.split("\n")

# part 1
ans = 0
p = r'X'
d = dict()

def how_many_XMAS(row, col):
    counter = 0
    dir = [
        (-1, 0), (1, 0), (0, 1), (0, -1),
        (-1, -1), (-1, 1), (1, 1), (1, -1),
    ]
    for d_row, d_col in dir:
        s_row, s_col = row + (d_row * 3), col + (d_col * 3)
        if (0 <= s_row < len(l) and 0 <= s_col < len(l[0])):
            m = l[row + d_row][col + d_col] == "M"
            a = l[row + (d_row * 2)][col + (d_col) * 2] == "A"
            s = l[row + (d_row * 3)][col + (d_col) * 3] == "S"
            if m and a and s: counter += 1
    return counter

for ll_index, ll in enumerate(l):
    xes = []
    X = re.finditer(p, ll)
    for x in X:
        xes.append(x.start())
    for x in xes:
        n = how_many_XMAS(ll_index, x)
        ans += n
print(ans)

# part 2

ans = 0
p = r'A'

def valid_xmas(row, col):
    if col-1<0 or row-1<0 or row+1>=len(l) or col+1>=len(l[0]): return False
    if l[row-1][col-1] == "M" and l[row-1][col+1] == "M" and l[row+1][col-1] == "S" and l[row+1][col+1] == "S": return True
    if l[row+1][col-1] == "M" and l[row+1][col+1] == "M" and l[row-1][col-1] == "S" and l[row-1][col+1] == "S": return True
    if l[row-1][col-1] == "M" and l[row+1][col-1] == "M" and l[row-1][col+1] == "S" and l[row+1][col+1] == "S": return True
    if l[row-1][col+1] == "M" and l[row+1][col+1] == "M" and l[row-1][col-1] == "S" and l[row+1][col-1] == "S": return True
    return False


for ll_index, ll in enumerate(l):
    ases = []
    A = re.finditer(p, ll)
    
    for a in A:
        ases.append(a.start())
    for a in ases:
        if valid_xmas(ll_index, a): ans +=1
print(ans)