#include <stdio.h>

int main(void) {
  FILE *f = fopen("tmp/root_symlink/tmp/yuck", "w");
  fprintf(f, "hello world\n");
  fclose(f);
  rename("tmp/root_symlink/tmp/yuck", "tmp/root_symlink");
  return 0;
}
