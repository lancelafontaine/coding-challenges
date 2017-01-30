#!/bin/python3

def sock_merchant(c):
    counter = 0
    socks = set()

    for sock in c:
        if sock not in socks:
            socks.add(sock)
        else:
            counter += 1
            socks.remove(sock)

    return counter


if __name__ == '__main__':
    import sys
    n = int(input().strip())
    c = input().strip().split(' ')
    print(sock_merchant(c))

