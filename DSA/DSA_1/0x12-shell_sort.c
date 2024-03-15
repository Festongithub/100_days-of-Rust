#include <stdio.h>

void shell_sort(int *array, size_t size)
{
  for (size_t gap = size / 2; gap > 0; gap /= 2)
    {
      for (size_t i = gap; i < size; i++)
	{
	  size_t tmp = array[i];
	  size_t j 
