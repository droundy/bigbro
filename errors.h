#ifndef ERRORS_H
#define ERRORS_H

#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>

static const int debug_output = 0;

static inline void debugprintf(const char *format, ...) {
  va_list args;
  va_start(args, format);
  if (debug_output) vfprintf(stdout, format, args);
  va_end(args);
}

static inline void verbosedebugprintf(const char *format, ...) {
  va_list args;
  va_start(args, format);
  if (debug_output > 1) vfprintf(stdout, format, args);
  va_end(args);
}

#endif
