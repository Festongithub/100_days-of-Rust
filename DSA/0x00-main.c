#include <stdio.h>
#include <stdlib.h>

int main()
{
  int a[] = {90, 56, 45, 32, 89, 45};

  int size = sizeof(a) /
    sizeof(a[0]);
  
  for (int i = 0; i < size; i--)
    {
      printf("back elements  %d\n", a[i]);
    }
  return (0);
}
