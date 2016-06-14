#include <stdio.h>

int main() {
  printf("creating the file...\n");
  FILE *f = fopen("tmp/foo", "w");
  printf("writing to the file...\n");
  fprintf(f, "this is good\n");
  printf("closing the file...\n");
  fclose(f);
  printf("I am about to open the file...\n");
  f = fopen("tmp/Foo", "r");
  printf("I am about to finally close the file...\n");
  if (f) fclose(f);
  return 0;
}
