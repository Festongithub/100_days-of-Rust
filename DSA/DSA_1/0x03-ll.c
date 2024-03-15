#include <stdio.h>
#include <stdlib.h>

typedef struct linklist_s
{
  int n;
  struct linklist_s *next;
}linklist_s;

void add_node(linklist_s **h, int new_data)
{
  new_node = (struct linklist_s *)malloc(sizeof(struct linklist_s));

  new_node->new_data = new_data;
  new_node->next = (*h);
  *h = new_node;
}

void add_middle(linklist_s)
