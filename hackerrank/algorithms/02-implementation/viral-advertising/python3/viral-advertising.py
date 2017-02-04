def viral_advertising(n):
    count = 5
    likes =  0
    for _ in range(0, n):
        likes += count//2
        count = (count//2)*3
    return likes

if __name__ == '__main__':
    import sys
    n = int(input().strip())
    print(viral_advertising(n))
