#include <stdio.h>
#include <stdlib.h>

int main()
{
  int n;
  char *arr;
  n = 5;

  arr = malloc(n * sizeof(char));
  arr[0] = 'H';
  arr[1] = 'e';
  arr[2] = 'l';
  arr[3] = 'l';
  arr[4] = '\0';
  printf("%s\n", arr);

  int i;
  i = 3;

  int *a;
  a = malloc(i * sizeof(int));
  a[0] = 68;
  a[1] = 69;
  a[2] = 70;
  a[3] = 71;
  a[4] = 72;
  a[5] = 73;
  a[6] = 74;
  a[7] = 75;
  a[8] = 76;
  a[9] = 77;
  a[10] = 78;
  a[11] = 79;
  printf("numbers: %ls\n", a);
  return (0);
}
