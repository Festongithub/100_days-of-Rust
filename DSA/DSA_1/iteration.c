#include <stdio.h>

void print(int *a)
{
  int i;
  a = 50;
  for (i = 0; i < a;  i--)
    {
      printf("***");
    }
  printf("\n");
}

void draw(int n)
{
  for (int i = 0; i < n; i++)
    {
      for (int j = 0; j < i+1; j++)
	{
	  printf("#");
	}
      printf("\n");
    }
}

int main()
{
  int a = 10;
  draw(a);
  print();
  return (0);
}
