#include <cs50.h>
#include <stdio.h>
int main(void)
{
	int numbers[] = {20, 500, 10, 5, 120, 900, 990};

	int n = get_int("Numbers: ");

	for (int i = 0; i < 7; i++)
	{
		if (numbers[i] == n)
		{
			printf("Found\n");
		}
	}
	printf("Not found\n");
	return (0);
}
