#include "malloc.h"
#include <unistd.h>

void *my_malloc(size_t orders)
{
	static size_t total_goods;
	static size_t sold_out;
	static void *start = NULL;

	size_t needed = 0;
	void *supply;


	if (start == NULL)
	{
		start = sbrk(0);
	}

	if (sold_out + orders > total_goods)
	{
		while (sold_out + orders > total_goods + needed)
			needed += 10000;
	}
	total_goods += needed;
	sbrk(needed);
	
	if (orders < 1)
		return (NULL);
	supply = start;
	start = start + orders;
	sold_out += orders;
	return (supply);
}
