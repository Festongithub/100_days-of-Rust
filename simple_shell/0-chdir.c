#include <stdio.h>
#include <unistd.h>

int chdir(const char *path)
{ 
int main()
{

  if (chdir("/usr") != 0)
    perror("chdir() to /usr failed");

  if (chdir("/tmp") != 0)
    perror("chdir() to /tmp failed");

  if (chdir("/error") != 0)
    perror("chdir() to /error failed");
  return (0);
}
