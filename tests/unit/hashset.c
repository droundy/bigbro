#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#include "hashset.h"

const int bufsize = 4*4096;

int main() {
  hashset set_one, set_two, set_union;
  char *buf = (char *)malloc(bufsize);
  int setsize = atoi(fgets(buf, bufsize, stdin));
  if (setsize < 1) setsize = 1;
  if (setsize > 1024*1024) setsize = 1024;
  init_hashset(&set_one, setsize);
  init_hashset(&set_two, setsize);
  init_hashset(&set_union, setsize);

  // first read things into set_one
  while (fgets(buf, bufsize, stdin)) {
    if (!strcmp(buf, "=\n")) break; // switch to set_two
    if (*buf != 0) {
      if (*buf == '-') {
	delete_from_hashset(&set_one, buf+1);
      } else {
	insert_hashset(&set_one, buf);
      }
    }
  }

  // second read things into set_two
  while (fgets(buf, bufsize, stdin)) {
    if (!strcmp(buf, "=\n")) break; // switch to set_two
    if (*buf != 0) {
      if (*buf == '-') {
	delete_from_hashset(&set_two, buf+1);
      } else {
	insert_hashset(&set_two, buf);
      }
    }
  }

  char **one = hashset_to_array(&set_one);
  char **two = hashset_to_array(&set_two);

  // third, create the union
  for (char **p = one; *p; p++) {
    insert_hashset(&set_union, *p);
  }
  for (char **p = two; *p; p++) {
    insert_hashset(&set_union, *p);
  }

  for (char **p = one; *p; p++) {
    assert(lookup_in_hash(&set_union, *p));
  }
  for (char **p = two; *p; p++) {
    assert(lookup_in_hash(&set_union, *p));
  }
  free_hashset(&set_one);
  free_hashset(&set_two);
  free_hashset(&set_union);

  return 0;
}
