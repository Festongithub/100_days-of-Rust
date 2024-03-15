#include "malloc.h"
#include <stdio.h>
int main(void)
{
	int *intprt;
	float *floatprt;
	char *string;

	intprt = my_malloc(sizeof(int));
	if (intprt == NULL)
	{
		printf("could not allocate\n");
		return (1);
	}

	floatprt = my_malloc(sizeof(float));

	if (floatprt == NULL)
	{
		printf("Could not allocate\n");
		return(1);
	}
	string = my_malloc(sizeof(char) * 6);
	if (string == NULL)
	{
		printf("could not allocate\n");
		return (1);
	}
	
	*intprt = 1024;
	*floatprt = 98.7;
	string[0] = 'H';
	string[1] = 'E';
	string[2] = 'L';
	string[3] = 'L';
	string[4] = 'L';
	string[5] = '\0';
	

	printf("Intprt: %i\n", *intprt);
	printf("floatprt: %f\n", *floatprt);
	printf("String: %s\n", string);
	return (0);

}
