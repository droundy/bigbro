#define _GNU_SOURCE // needed for mempcpy

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>

#include "hashset.h"
#include "realpath.h"

const int bufsize = 4*4096;

int main() {
  printf("am in realpath test\n");
  rw_status st;
  char *buf = (char *)malloc(bufsize);
  char *parent = (char *)malloc(bufsize);
  /* char *resolved = (char *)malloc(bufsize); */
  int setsize = 50;
  init_hashset(&st.read, setsize);
  init_hashset(&st.readdir, setsize);
  init_hashset(&st.written, setsize);
  init_hashset(&st.mkdir, setsize);

  if (fgets(buf, bufsize, stdin)) {
    int len = strlen(buf);
    if (len > 1) {
      if (buf[len-1] == '\n') {
	buf[len-1] = 0;
      }
    }
    strcpy(parent, buf);
    for (int i=len-1; i>=0; i--) {
      if (parent[i] == '/') {
        parent[i] = 0;
        break;
      }
    }
    bool is_symlink = false;
    bool exists = false;
    bool parent_exists = false;
    bool parent_isdir = false;
    struct stat thestat;
    if (!lstat(buf, &thestat)) {
      if (S_ISLNK(thestat.st_mode)) is_symlink = true;
    }
    if (!stat(buf, &thestat)) {
      exists = true;
    }
    if (!stat(parent, &thestat)) {
      parent_isdir = S_ISDIR(thestat.st_mode);
      parent_exists = true;
    } else if (*parent == 0) {
      parent_exists = true;
      parent_isdir = true;
    }
    if (is_symlink) printf("this is a symlink\n");
    else printf("NOT a symlink\n");

    if (exists) printf("this thing exists\n");
    else printf("thing does NOT exist\n");

    if (parent_exists) printf("this parent exists\n");
    else printf("parent does NOT exist\n");

    if (parent_isdir) printf("this parent is a directory\n");
    else printf("parent '%s' is NOT a directory\n", parent);

    char *actual_rp = realpath(buf, 0);
    char *rp = flexible_realpath(buf, 0, &st, look_for_file_or_directory, true);
    printf("input %s\n", buf);
    printf("actual_rp %s\n", actual_rp);
    printf("flexible_realpath look_for_file_or_directory true returns %s\n", rp);
    if (*buf != '/') {
      printf("relative paths count as failure...\n");
      assert(!rp);
    } else if (!parent_isdir) {
      printf("got %s from %s\n", rp, buf);
      assert(!rp); // flexible_realpath returns null if there is no parent
    } else if (!exists) {
      printf("got %s from %s\n", rp, buf);
      assert(rp);
    } else {
      printf("exists so %s from %s\n", rp, buf);
      assert(rp);
      assert(actual_rp);
      assert(!strcmp(rp, actual_rp));
    }
    char *symrp = flexible_realpath(buf, 0, &st, look_for_symlink, true);
    if (*buf != '/') {
      printf("relative paths count as failure...\n");
      assert(!rp);
    } else if (is_symlink) {
      assert(symrp);
    } else {
      if (!rp) {
	assert(!symrp);
      } else {
	assert(!strcmp(rp, symrp));
      }
    }
    /* rp = flexible_realpath(buf, resolved, &st, look_for_file_or_directory, false); */
    /* if (!exists) { */
    /*   assert(!rp); */
    /* } else { */
    /*   assert(!strcmp(rp, actual_rp)); */
    /* } */
    /* symrp = flexible_realpath(buf, resolved, &st, look_for_symlink, false); */
    /* if (is_symlink) { */
    /*   assert(rp); */
    /* } else { */
    /*   assert(!strcmp(rp, symrp)); */
    /* } */
  }
  free_hashset(&st.read);

  return 0;
}
