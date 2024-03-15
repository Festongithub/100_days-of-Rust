#include <stdio.h>
#include <unistd.h>
#include <errorn.h>
#include <sys/types.h>
#include <fcntl.h>

int find( int argc, const char *argv[])
{
	int fd = access("add.c", F_OK);
	if (fd == -1)
	{
		printf("Error number: %d\n", errno);
		perror("Error description");
	}
	else {
		printf("No error\n");
	}
}

int main()
{

