from collections import defaultdict
import re

p = r"\d+"

data = open('input.txt').read().strip()
rules, updates_ = data.split("\n\n")

dict_rules = defaultdict(set)
for r in rules.split("\n"):
    x, y = map(int, re.findall(p, r))
    dict_rules[x].add(y)

# part 1
ans1 = 0
ans2 = 0
invalid_updates = []
for u in updates_.split("\n"):
    upd = list(map(int, re.findall(p, u)))

    # part 1
    if not any(upd[i] in dict_rules[upd[j]] for i in range(len(upd)) for j in range(i + 1, len(upd))):
        ans1 += upd[len(upd) // 2]
    # part 2
    else:
        helper = {up: sum(1 for x in dict_rules if up in dict_rules[x] and x in upd) for up in upd}
        make_it_valid = []
        while helper:
            x = next(up for up in helper if helper[up] == 0)
            make_it_valid.append(x)
            del helper[x]
            for y in dict_rules[x]:
                if y in helper:
                    helper[y] -= 1
        ans2 += make_it_valid[len(make_it_valid)//2]
print(ans1)
print(ans2)



