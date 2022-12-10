INPUT_FILE = "../input.txt"

def to_code(char):
    code = ord(char)
    if code > 96:
        return code - 96
    else:
        return code - 38

part1 = 0
part2 = 0

with open(INPUT_FILE, 'r') as file:
    buffers = [
        set(), set()
    ]
    for i, line in enumerate(file):
        x = i % 3
        line = line.strip()
        half = int(len(line)/2)
        a = set([to_code(x) for x in line[:half]])
        b = set([to_code(x) for x in line[half:]])
        part1 += sum(a.intersection(b))
        if x == 2:
            temp = buffers[0].intersection(buffers[1]).intersection(a | b)
            part2 += sum(temp)
        else:
            buffers[x] = a | b

print('Part #1:', part1)
print('Part #2:', part2)
