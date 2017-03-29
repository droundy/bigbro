#include <stdio.h>

int main(void) {
  FILE *f = fopen("tmp/foodle", "a");
  fprintf(f, "hi there!\n");
  fclose(f);
  fprintf(stderr, "about to rename\n");
  int returncode = rename("tmp/foodle", "tmp/barbaz");
  fprintf(stderr, "rename gives %d\n", returncode);

  f = fopen("tmp/silly-file", "a");
  fprintf(f, "hi there!\n");
  fclose(f);
  rename("tmp/silly-file", "tmp/silly-file");
return 0;
}
