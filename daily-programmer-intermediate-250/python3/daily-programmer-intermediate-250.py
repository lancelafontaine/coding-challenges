#!/usr/bin/env python3

#### PERFORMANCE:
# daily-programmer-intermediate-250/python3 → time ./daily-programmer-intermediate-250.py 8
#[42101000]
#./daily-programmer-intermediate-250.py 8  195.00s user 0.99s system 99% cpu 3:16.12 total

def selfDescriptiveNumber(numDigits):
    if numDigits <= 0:
        return "Number must be greater than 0"

    resultList = []
    startRange = "1" + (numDigits-1)*"0"
    endRange = startRange + "0"
    numList = list(range(int(startRange), int(endRange)))

    for num in numList:
        digitSum = 0
        shouldContinue = False
        for i in range(len(str(num))):
            digitSum += int(str(num)[i])
            if digitSum > numDigits or str(num).count(str(i)) != int(str(num)[i]):
                shouldContinue = True
                break
        if shouldContinue:
            continue
        resultList.append(num)

    return resultList if len(resultList) > 0  else "No self-descriptive numbers in this range."

if __name__ == '__main__':
    import sys
    if len(sys.argv) <= 1:
        print("Must provide number of digits as an argument.")
        sys.exit()
    try:
        numDigits = int(sys.argv[1])
    except ValueError:
        print("Must provide an integer as an argument for number of digits")
        sys.exit()
    print(selfDescriptiveNumber(numDigits))

