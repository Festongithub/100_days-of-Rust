#include <fcntl.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>
#include "malloc.h"
#include <stdio.h>

int main(void)
{
	struct stat statbuf;
	size_t size;
	char *contents;
	int fd;

	fd = open("Person.py", O_RDONLY);
	fstat(fd, &statbuf);
	size = statbuf.st_size;

	contents = my_malloc(size + 1);
	read(fd, contents, size);
	contents[size] = '\0';

	printf("%s\n", contents);
	return (0);
}
