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

#include <assert.h>
#include <windows.h>
#include <limits.h>
#include <stdio.h>

#include "../errors.h"
#include "queue.h"

int queueInit(struct queue *q, const char *key) {
  static size_t sz = 1024*10;
  int i;
  char *qname = malloc(4096);
  for (i = 0; key[i]; i++) {
    qname[i] = key[i] == '\\' || key[i] == ':'? '/' : key[i];
  }
  qname[i] = 0;
  q->mf = CreateFileMappingA(INVALID_HANDLE_VALUE, 0, PAGE_READWRITE,
                             0, sz, qname);
  if (q->mf == 0) return 1;
  q->buf = MapViewOfFile(q->mf, FILE_MAP_ALL_ACCESS, 0, 0,
                         sizeof(struct queue_internals) + sz);
  if (q->buf == 0) return 1;
  q->buf->written_to_here = 0;
  q->buf->size = sz;
  return 0;
}

int queueTerm(struct queue *q) {
  int err = 0;
  HANDLE mf = q->mf;
  err += !UnmapViewOfFile(q->buf) << 0;
  err += !CloseHandle(mf) << 1;
  return err;
}

static struct queue q;

int globalQueueInit(const char *name) {
  assert(!q.buf);
  return queueInit(&q, name);
}

int globalQueueTerm() {
  assert(q.buf);
  return queueTerm(&q);
}

void queueOp(char op, const char *filename) {
  if (!q.buf) return;
  if (!filename) return;
  uint32_t sz = 2 + strlen(filename);
  uint32_t bufsize = q.buf->size;
  uint32_t write_start = __sync_fetch_and_add(&q.buf->written_to_here, sz) % bufsize;
  q.buf->data[write_start] = op;
  for (uint32_t i=0; i < sz-1; i++) {
    q.buf->data[(write_start+i+1) % bufsize] = filename[i];
  }
  debugprintf("DEBUG: %s\n", &q.buf->data[write_start]);
}

void queueOp2(char op, const char *filename1, const char *filename2) {
  if (!q.buf) return;
  if (!filename1) return;
  if (!filename2) return;
  uint32_t sz1 = strlen(filename1)+1;
  uint32_t sz2 = strlen(filename2)+1;
  uint32_t sz = 1 + sz1 + sz2;
  uint32_t bufsize = q.buf->size;
  uint32_t write_start = __sync_fetch_and_add(&q.buf->written_to_here, sz) % bufsize;
  q.buf->data[write_start] = op;
  for (uint32_t i=0; i < sz1; i++) {
    q.buf->data[(write_start+i+1) % bufsize] = filename1[i];
  }
  for (uint32_t i=0; i < sz2; i++) {
    q.buf->data[(write_start+i+sz1+1) % bufsize] = filename2[i];
  }
  debugprintf("DEBUG: %s\n", &q.buf->data[write_start]);
}
