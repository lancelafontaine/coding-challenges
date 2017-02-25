def find_digits(n):
    counter = 0
    digits = [ int(elem) for elem in list(str(n)) ]
    for digit in digits:
        if digit != 0 and n % digit == 0:
            counter += 1
    return counter

if __name__ == "__main__":
    import sys
    t = int(input().strip())
    for a0 in range(t):
        n = int(input().strip())
        print(find_digits(n))

