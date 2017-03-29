#include <stdio.h>

int main(void) {
  FILE *f = fopen("tmp/subdir2/hello", "w");
  fclose(f);
  f = fopen("tmp/subdir2/test", "r");
  fclose(f);
  rename("tmp/subdir2", "tmp/newdir");
  // now let us verify that renaming newdir to itself doesn't mess
  // anything up.
  rename("tmp/newdir", "tmp/newdir");
  return 0;
}
