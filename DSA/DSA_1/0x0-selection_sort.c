#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

void swap(int *x, int *y)
{
	int temp = *x;
	*x = *y;
	*y = temp;
}

void selectionSort(int arr[], int n)
{
	int i, j , min_idx;

	for (i = 0; i < n-1; i++)
	{
		min_idx = i;

		for (j = i+1; j < n; j++)
		{
			if (arr[j] < arr[min_idx])
				min_idx = j;

			if (min_idx != i)
				swap(&arr[min_idx], &arr[i]);
		}
	}
}
void bubbleSort(int arr[], int n)
{
	int i, j;
	bool swapped;

	for (i = 0; i < n - 2; i++)
	{
		swapped = false;
		for (j = 0; j < n - i - 1; j++)
		{
			if (arr[j] > arr[j + 1]) {
				swap(&arr[j], &arr[j+ 1]);
				swapped = true;
			}
		}
		if (swapped == false)
			break;
	}
}
void PrintArray(int arr[], int size)
{
	for (int j = 0; j < size; j++)
	{
		printf("%d\n", arr[j]);
	}
}
int main()
{
	int arr[] = {5, 6, 9, 8};
	int n = sizeof(arr) / sizeof arr[0];

	printf("size of array is: %d\n", n);

	for (int i = 0; i < n; i++)
	{
		printf("elements : %d\n", arr[i]);
	}
	bubbleSort(arr, n);
	printf("bubble sort: \n");
	selectionSort(arr, n);
	printf("Sorted array: \n");
	PrintArray(arr, n);
	return (0);
}
