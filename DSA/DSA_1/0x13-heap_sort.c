#include <stdio.h>

void swap(int *x, int *y)
{
  int temp;
  *x = temp;
  *x = *y;
  *y = temp;
}

void heapify(int *arr,  int N, int i)
{
  int largest = i;
  int left = 2 * i + 1;
  int right = 2 * i + 2;

  if (left < N && arr[left] > arr[largest])
    largest = left;

  if (right < N && arr[right] > arr[largest])
    largest = right;

  if (largest != i)
    {
      swap(&arr[i], &arr[largest]);
      heapify(arr, N, largest);
    }
}
void heapSort(int *arr, int N)
{
  for (int i = N/ 2 - 1; i >= 0; i--)
    heapify(arr, N, i);
  for (int i = N - 1; i >= 0; i--)
    {
      swap(&arr[0], &arr[i]);
      heapify(arr, i, 0);
    }
}

void printArray(int arr[], int N)
{
  for (int i = 0; i < N; i++)
    {
      printf("%d ", arr[i]);
    }
  printf("\n");
}

int main()
{
  int array[] = {19, 48, 99, 71, 13, 52, 96, 73, 86, 7};
  int N = sizeof(array) / sizeof(array[0]);

  heapSort(array, N);
  printf("Sorted array\n");
  printArray(array, N);
  return (0);
}
