#include <stdlib.h>

void func1() {
  int a = 10;
  int *b = func2(a);
  free(b); // ヒープメモリ解放
}

int* func2(int a) {
  // ヒープメモリに確保
  int *tmp = (int*)malloc(sizeof(int));
  *tmp = a * 20;
  return tmp;
}