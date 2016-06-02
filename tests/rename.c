#include <stdio.h>

int main(void) {
  rename("tmp/foo", "tmp/barbaz");
  return 0;
}
