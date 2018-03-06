#include <stdint.h>

const char* rust_greeting(const char* to);
void rust_greeting_free(char *);

int rust_compute_hashes(const char*);
int rust_get_hashes_size();
char ** rust_get_hashes();
