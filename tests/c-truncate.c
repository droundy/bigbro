#define _XOPEN_SOURCE 500

#include <unistd.h>
#include <stdio.h>

int main(void) {
  if (!truncate("tmp/foobar", 100)) {
    printf("/tmp/foobar should fail because foobar does not exist\n");
    return 1;
  }
  if (truncate("tmp/foo", 100)) {
    printf("/tmp/foo should not fail because foobar does not exist\n");
    return 1;
  }
  return 0;
}
