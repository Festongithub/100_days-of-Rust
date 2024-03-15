#include <stdio.h>

void swap(int *x, int *y)
{
	int temp;
	temp = *x;
	*x = *y;
	*y = temp;
}
void selection_sort(int arr[], int n)
{
	/**
	 * create an outer loop which traverses through the
	 * array
	 * */

	int i;
	for (i = 0; i < n-1; i++)
	{
		int min = i;
		/**
		 * min = 0
		 **/

		int j;
		/**
		 * create an inner loop that compares the elements
		 * */

		for (j = i + 1; j < n; j++)
		{
			if (arr[j] < arr[min])
			{
				min = j;
			}
		}
		if (min != i)
		{
			swap(&arr[min], &arr[i]);
		}
	}
}

void printArray(int arr[], int size)
{
	for (int i = 0; i < size; i++)
	{
		printf("[%d] ", arr[i]);
	}
	printf("\n");
}
int main()
{
	int arr[] = {7, 4, 10, 8, 3, 1};
	int n = sizeof(arr) / sizeof(arr[0]);
	selection_sort(arr, n);
	printf("Sorted array: \n");
	printArray(arr, n);
	return (0);
}
