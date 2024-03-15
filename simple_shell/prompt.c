#include <stdio.h>


int arguments(int argc, char *argv[])
{
	printf("Argument count: %d\n", argc);

	for (int i = 0; i <argc; i++)
	{
		printf("Argument %d: %s\n", i, argv[i]);
	}
}

int main(int argc, char *argv[])
{
	arguments(argc, argv);
	return (0);
}
