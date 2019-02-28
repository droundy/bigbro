#include <sys/stat.h>

int main(void) {
  struct stat st;
  stat("tmp/foo", &st);
  return 0;
}
