#include <unistd.h>
#include <fcntl.h>
#include <stdio.h>

int main() {
  printf("about to chdir missing-directory\n");
  int output = chdir("missing-directory");
  printf("gave %d\n", output);
  printf("about to chdir missing/directory\n");
  output = chdir("missing/directory");
  printf("gave %d\n", output);
  printf("about to create tmp/still-running\n");
  open("tmp/still-running", O_WRONLY | O_CREAT, 0666);
  printf("have created tmp/still-running\n");
  return 0;
}
