with open('inputs/input1.txt', 'r') as file:
    example_data = file.read()
    

lines = example_data.splitlines()
banks = [[int(x) for x in list(line)] for line in lines]

def get_max_joltage(bank, n = 2):
    joltage = [0] * n

    for i, battery in enumerate(bank):
        for j, b_j in enumerate(joltage):
            if i - j >= len(bank) - n + 1:
                continue
            
            if battery > b_j:
                joltage[j] = battery
                for k in range(j + 1, len(joltage)):
                    joltage[k] = 0
                break
    str_joltage = "".join(map(str, joltage))
    return int(str_joltage)

def find_total_joltage(n = 2):
    total = 0
    for bank in banks:
        max_joltage = get_max_joltage(bank, n)
        total += max_joltage 
    return total

def part1():
    import time
    start = time.time()
    total = find_total_joltage(2)
    end = time.time()
    print(f"Part 1: {total}")
    print(f"Part 1 execution time: {end - start:.6f} seconds")

def part2():
    import time
    start = time.time()
    total = find_total_joltage(12)
    end = time.time()
    print(f"Part 2: {total}")
    print(f"Part 2 execution time: {end - start:.6f} seconds")
    
if __name__ == "__main__":
    part1()
    part2()