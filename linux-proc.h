#include "realpath.h"
#include "fcntl.h" // for AT_FDCWD

static inline char *flexible_realpath_at(pid_t pid, int dirfd, const char *path,
                                         rw_status *h,
                                         enum last_symlink_handling lasth) {
  if (!path) return NULL;
  char *rawpath = NULL;
  do {
    /* printf("path: %s\n", path); */
    const char *procself = "/proc/self/";
    const int procselflen = strlen(procself);
    if (strlen(path) > procselflen &&
        !memcmp(path, procself, procselflen)) {
      char *out = malloc(PATH_MAX);
      snprintf(out, PATH_MAX, "/proc/%d/%s", pid, path + procselflen);
      /* printf("rawpath: %s\n", out); */
      rawpath = out;
      break;
    }
    /* printf("path '%s' does not match '%s'\n", path, procself); */
    if (path[0] == '/') {
      /* printf("rawpath: %s\n", path); */
      rawpath = strdup(path);
      break;
    }

    char *proc_fd = malloc(PATH_MAX);
    if (dirfd >= 0 && dirfd != AT_FDCWD) snprintf(proc_fd, PATH_MAX, "/proc/%d/fd/%d", pid, dirfd);
    else snprintf(proc_fd, PATH_MAX, "/proc/%d/cwd", pid);
    char *cwd = malloc(PATH_MAX);
    int linklen = readlink(proc_fd, cwd, PATH_MAX);
    if (linklen < 0) {
      fprintf(stderr, "unable to determine cwd from %s i.e. fd %d!!! (%s)\n",
              proc_fd, dirfd, strerror(errno));
      return NULL;
    }
    cwd[linklen] = 0;
    /* printf("cwd(%d): %s (from %s)\n", dirfd, cwd, proc_fd); */
    int total_len = strlen(cwd) + strlen(path) + 4;
    if (total_len > PATH_MAX) cwd = realloc(cwd, total_len);
    strcat(cwd, "/");
    strcat(cwd, path);
    free(proc_fd);
    /* printf("rawpath: (%d) %s\n", dirfd, cwd); */
    rawpath = cwd;
    break;
  } while (0); // this loop is just to easily allow early jumping with break (hokey)

  // at this stage, rawpath holds `pwd`/path, but we have not yet
  // interpreted for symlinks, which is our next job:
  char *rp = flexible_realpath(rawpath, h, lasth);
  free(rawpath);
  return rp;
}

static inline void read_dir_fd(pid_t pid, int dirfd, rw_status *h) {
  char *abspath = flexible_realpath_at(pid, dirfd, ".", h, look_for_file_or_directory);
  if (!lookup_in_hash(&h->mkdir, abspath)) {
    insert_hashset(&h->readdir, abspath);
  }
  free(abspath);
}

static inline void read_something_at(pid_t pid, int dirfd, const char *path,
                                     rw_status *h, enum last_symlink_handling lh) {
  char *abspath = flexible_realpath_at(pid, dirfd, path, h, lh);
  if (!abspath) return;
  struct stat st;
  if (!lookup_in_hash(&h->written, abspath) && !stat(abspath, &st) && S_ISREG(st.st_mode)) {
    insert_hashset(&h->read, abspath);
  }
  free(abspath);
}

static inline void write_something_at(pid_t pid, int dirfd, const char *path,
                                      rw_status *h, enum last_symlink_handling lh) {
  char *abspath = flexible_realpath_at(pid, dirfd, path, h, lh);
  insert_hashset(&h->written, abspath);
  delete_from_hashset(&h->read, abspath);
  free(abspath);
}

static inline void read_file_at(pid_t pid, int dirfd, const char *path,
                                rw_status *h) {
  read_something_at(pid, dirfd, path, h, look_for_file_or_directory);
}

static inline void maybe_read_file_at(pid_t pid, int dirfd, const char *path,
                                rw_status *h) {
  read_something_at(pid, dirfd, path, h, look_for_file_or_directory);
}

static inline void write_file_at(pid_t pid, int dirfd, const char *path,
                                 rw_status *h) {
  write_something_at(pid, dirfd, path, h, look_for_file_or_directory);
}

static inline void read_link_at(pid_t pid, int dirfd, const char *path,
                                rw_status *h) {
  read_something_at(pid, dirfd, path, h, look_for_symlink);
}

static inline void write_link_at(pid_t pid, int dirfd, const char *path,
                                 rw_status *h) {
  write_something_at(pid, dirfd, path, h, look_for_symlink);
}
