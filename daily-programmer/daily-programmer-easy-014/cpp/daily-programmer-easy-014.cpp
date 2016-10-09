// Ran with:
// g++ daily-programmer-easy-014.cpp && ./a.out

#include <iostream>

int* reserveBlockArray(int* origArr, int origArrSize, int blockSize) {

  if (blockSize > origArrSize) {
    blockSize = origArrSize;
  }

  int temp;
  for (int i = 0; i < blockSize/2; i++) {
    temp = origArr[i];
    origArr[i] = origArr[blockSize - i - 1];
    origArr[blockSize - i - 1] = temp;
  }
  return origArr;
}

int main() {
  // Initial values
  int origArr[7] = {1,2,3,4,5,6,7};
  int blockSize = 4;
  int origArrSize = sizeof(origArr)/sizeof(*origArr);

  int *result = reserveBlockArray(origArr, origArrSize, blockSize);
  for (int i = 0; i < origArrSize; i++) {
    std::cout << result[i] << std::endl;
  }
  return 0;
}



