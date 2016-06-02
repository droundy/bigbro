#include <unistd.h>

int main(void) {
  truncate("tmp/foobar", 100); /* should fail because foobar does not exist */
  truncate("tmp/foo", 100);
  return 0;
}
