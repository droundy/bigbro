#include <stdio.h>

int main(void) {
  fprintf(stderr, "about to rename\n");
  int returncode = rename("tmp/foo", "tmp/barbaz");
  fprintf(stderr, "rename gives %d\n", returncode);
  return 0;
}
