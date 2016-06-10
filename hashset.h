#ifndef HASHSET_H
#define HASHSET_H

#include <string.h>
#include <stdlib.h>
#include <assert.h>

struct hash_entry {
  const char *key;
  struct hash_entry *next;
};

typedef struct hashset {
  int size, num_entries;
  struct hash_entry *first;
  struct hash_entry **table;
} hashset;

static inline unsigned long hash_function(const char *strinput) {
  unsigned long hash = 5381;
  const unsigned char *str = (const unsigned char *)strinput;
  int c;
  while ((c = *str++)) hash = hash * 33 ^ c;
  return hash;
}

/* Find the data stored under str in the hash */
static struct hash_entry * lookup_in_hash(hashset *hash, const char *str) {
  unsigned long h = hash_function(str) % hash->size;
  struct hash_entry *e = hash->table[h];
  while (e) {
    if (strcmp(e->key, str) == 0) return e;
    if (!e->next || hash_function(e->next->key) % hash->size != h) return 0;
    e = e->next;
  }
  return 0;
}

static inline void insert_hashset(hashset *hash, const char *key) {
  if (lookup_in_hash(hash, key)) return;
  struct hash_entry *e = malloc(sizeof(struct hash_entry)+strlen(key)+1);
  e->key = ((char *)e) + sizeof(struct hash_entry);
  strcpy((char *)e->key, key);
  e->next = 0;

  hash->num_entries++;
  unsigned long h = hash_function(e->key) % hash->size;
  struct hash_entry *enext = hash->table[h];
  if (enext) {
    e->next = enext->next;
    enext->next = e;
  } else {
    e->next = hash->first;
    hash->first = e;
    hash->table[h] = e;
  }
}

static inline void delete_from_hashset(hashset *hash, const char *key) {
  struct hash_entry *e = lookup_in_hash(hash, key);
  if (e) {
    struct hash_entry *x = hash->first;
    if (x == e) {
      hash->first = e->next;
    } else {
      while (x) {
        if (x->next == e) {
          x->next = e->next;
        }
        x = x->next;
      }
    }
    unsigned long h = hash_function(e->key) % hash->size;
    if (hash->table[h] == e) {
      if (e->next && hash_function(e->next->key) % hash->size == h) {
        hash->table[h] = e->next;
      } else {
        hash->table[h] = 0;
      }
    }
    hash->num_entries -= 1;

    free(e);
  }
}

static inline char **hashset_to_array(hashset *hs) {
  int numentries = 0;
  long total_size = 0;
  for (struct hash_entry *e = (struct hash_entry *)hs->first;
       e; e = (struct hash_entry *)e->next) {
    numentries++;
    total_size += strlen(e->key) + 1;
  }
  char **array = malloc((numentries + 1)*sizeof(char *) + total_size);
  char *strings = (char *)(array + numentries + 1);
  int i = 0;
  for (struct hash_entry *e = (struct hash_entry *)hs->first;
       e; e = (struct hash_entry *)e->next) {
    array[i] = strings;
    const char *from = e->key;
    while (*from) {
      *strings++ = *from++; // copy the key and advance the string;
    }
    *strings++ = 0; // add the null termination
    i++;
  }
  array[numentries] = 0; // terminate with a null pointer.
  return array;
}

static void free_hashset(hashset *h) {
  free(h->table);
  struct hash_entry *todelete = 0;
  for (struct hash_entry *e = h->first; e; e = e->next) {
    free(todelete);
    todelete = e;
  }
  free(todelete);
}

static void init_hashset(hashset *h, int size) {
  h->size = size;
  h->num_entries = 0;
  h->first = 0;
  h->table = calloc(sizeof(struct hash_entry *), size);
}

#endif
