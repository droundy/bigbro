#include <stdio.h>

int main() {
  FILE *f = fopen("tmp/foo", "w");
  fprintf(f, "this is good\n");
  fclose(f);
  return 0;
}
