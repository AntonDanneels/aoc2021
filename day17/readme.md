No solution in Rust today, solved using a python REPL by just brute forcing
the solution:

def is_valid_trajectory(vx, vy, min_x, max_x, min_y, max_y):
    x, y = 0, 0

    while x <= max_x and y >= max_y:
        x += vx
        y += vy
        vx -= 1 if vx > 0 else 0
        vy -= 1

        if x >= min_x and x <= max_x and y <= min_y and y >= max_y:
            return True

    return False

def solve(x1, x2, y1, y2):
    result = []
    for vy in range(y2, -y2 + 1):
        for vx in range(1, x2 +1):
            if is_valid_trajectory(vx, vy, x1, x2, y1, y2):
                result.append((vx, vy))
    return result

print(solve(20, 30, -5, -10))
print(len(solve(20, 30, -5, -10)))
print(len(solve(144, 178, -76, -100)))

