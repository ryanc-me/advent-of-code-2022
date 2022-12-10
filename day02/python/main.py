INPUT_FILE = "../input.txt"

part1 = 0
part2 = 0
def get_result(our_move, their_move):
    res = our_move - their_move - 1
    while res < 1:
        res += 3
    return res

def get_move(their_move, desired_result):
    res = desired_result + their_move + 1
    while res > 3:
        res -= 3
    return res

with open(INPUT_FILE, 'r') as file:
    buffer = 0
    for line in file:
        op = ord(line[0]) - 64
        me = ord(line[2]) - 87
        
        part1 += me
        result1 = get_result(me, op)
        if result1 == 2:
            part1 += 3
        elif result1 == 3:
            part1 += 6

        result2 = get_move(op, me)
        part2 += result2
        if me == 2:
            part2 += 3
        elif me == 3:
            part2 += 6


print('Part #1:', part1)
print('Part #2:', part2)
