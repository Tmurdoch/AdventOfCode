

f = open("input.txt", "r+")


lines = [x.strip() for x in f.readlines()]

max_voltage = 0
for line in lines:
    splt = [int(x) for x in list(line)]
    first_max = max(splt[0:len(splt) - 1])
    max_idx = splt.index(first_max)
    second_max = max(splt[max_idx+1:])

    together = str(first_max) + str(second_max)
    max_voltage += int(together)
    print(together, line)

print(max_voltage)

    


