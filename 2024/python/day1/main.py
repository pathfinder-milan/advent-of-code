# part 1
file = open('input.txt', 'r')
data = file.read()

helper_fn = lambda line: map(int, line.split())
left, right = map(sorted, zip(*map(helper_fn, data.strip().splitlines())))

result = sum(abs(l - r) for l, r in zip(left, right))
print(result)

# part 2
similarity_score = sum(map(lambda l: right.count(l) * l, left))
print(similarity_score)
