# part 1
file = open('input.txt', 'r')
data = file.read()

helper_fn = lambda line: [line_ll for line_ll in map(int, line.split())]

second_helper_fn = lambda t: (
    all(1 <= line[i+1] - line[i] <= 3 for i in range(len(line) - 1)) or
    all(-3 <= line[i+1] - line[i] <= -1 for i in range(len(line) - 1))
    for line in [helper_fn(t)]
)

true_positive = sum(next(second_helper_fn(line)) for line in data.strip().splitlines())
print(true_positive)

# part 2
def can_fix_by_removing(line) -> bool:
    line_ll = helper_fn(line)
    if next(second_helper_fn(line)):
        return True
    for i in range(len(line_ll)):
        new_line_ll = line_ll.copy()
        new_line_ll.pop(i)

        if all(1 <= new_line_ll[ii+1] - new_line_ll[ii] <= 3 for ii in range(len(new_line_ll) - 1)) or \
           all(-3 <= new_line_ll[ii+1] - new_line_ll[ii] <= -1 for ii in range(len(new_line_ll) - 1)):
            return True
    return False

with_false_negative = sum(can_fix_by_removing(line) for line in data.strip().splitlines())
print(with_false_negative)