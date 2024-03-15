#include <stdio.h>

int main()
{
	char a[] = {'H', 'e', 'l','l','o', '\0'};
	int size = sizeof(a) / sizeof(a[0]);

	for (int i = 0; i < size; i++)
	{
		printf("%s\n", i);
	}
	return (0);
}
