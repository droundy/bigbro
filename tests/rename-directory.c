#include <stdio.h>

int main(void) {
  FILE *f = fopen("tmp/subdir2/hello", "w");
  fclose(f);
  f = fopen("tmp/subdir2/test", "r");
  fclose(f);
  rename("tmp/subdir2", "tmp/newdir");
  return 0;
}
