INPUT_FILE = "../input.txt"

calories = set()
with open(INPUT_FILE, 'r') as file:
    buffer = 0
    for line in file:
        if line.isspace():
            calories.add(buffer)
            buffer = 0
        else:
            buffer += int(line)

calories = sorted(calories, reverse=True)
part1 = calories[0]
part2 = sum(calories[0:3])

print('Part #1:', part1)
print('Part #2:', part2)
