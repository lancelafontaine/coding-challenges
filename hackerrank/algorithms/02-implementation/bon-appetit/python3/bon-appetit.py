def bon_appetit(n, k, cost, b):
    sum_without_k = 0
    for i in range(n):
        if i != k:
            sum_without_k += cost[i]

    actual_charge = sum_without_k/2;

    if b != actual_charge:
        return int(b - actual_charge)
    return 'Bon Appetit'

if __name__ == '__main__':
    import sys
    n, k = input().split(' ')
    n, k = int(n), int(k)
    cost = input().split(' ')
    cost = [int(i) for i in cost]
    b = int(input().strip())
    print(bon_appetit(n, k, cost, b))

