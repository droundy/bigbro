/* Copyright (c) 2015, Jorge Acereda Maciá <jacereda@gmail.com>
   Copyright (C) 2016 David Roundy <daveroundy@gmail.com>

   Permission to use, copy, modify, and/or distribute this software
   for any purpose with or without fee is hereby granted, provided
   that the above copyright notice and this permission notice appear
   in all copies.

   THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL
   WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED
   WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE
   AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR
   CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
   OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT,
   NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
   CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE. */

#include <stdint.h>

struct queue_internals {
  uint32_t written_to_here;
  uint32_t size;
  char data[];
};

struct queue {
  HANDLE mf;
  struct queue_internals *buf;
};
int queueInit(struct queue *q, const char *key);
int queueTerm(struct queue *q);

int globalQueueInit(const char *name);
int globalQueueTerm();

enum OPERATIONS {
  WRITE_OP = 'w',
  READ_OP = 'r',
  RENAME_OP = 'n',
  READDIR_OP = 'd',
  GETINFO_OP = 'q',
  SETINFO_OP = 's'
};

void queueOp(char op, const char *filename);
void queueOp2(char op, const char *filename1, const char *filename2);
