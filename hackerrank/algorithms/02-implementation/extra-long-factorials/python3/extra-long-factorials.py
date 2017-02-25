def extra_long_factorials(n):
    if n == 1:
        return 1
    else:
        return n * extra_long_factorials(n-1)

if __name__ == "__main__":
    import sys
    n = int(input().strip())
    print(extra_long_factorials(n))
