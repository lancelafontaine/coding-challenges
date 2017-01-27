import sys

def mini_max_sum(int_list):
    max = 0
    min = float('inf')

    for i in range(0,len(int_list)):
        total = 0
        for j in range(0,len(int_list)):
            if j != i:
                total += int_list[j]

        if total > max:
            max = total
        if total < min:
            min = total

    return str(min) + ' ' + str(max)



if __name__ == '__main__':
    a,b,c,d,e = input().strip().split(' ')
    a,b,c,d,e = [int(a),int(b),int(c),int(d),int(e)]
    int_list = [a,b,c,d,e]

    print(mini_max_sum(int_list))

