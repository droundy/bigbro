#ifndef HASHSET_H
#define HASHSET_H

#include "iterablehash.h"
#include <stdbool.h>

typedef struct hash_table hashset;

static void initialize_hashset(hashset *array);
static void free_hashset(hashset *h);

static void insert_to_hashset(hashset *array, const char *path);

static char **hashset_to_array(hashset *hs); // returns a single malloced array

#endif
