
#include <stdlib.h>

int main() {

  unsigned char* c = malloc(100);

  c[100] = 0;

  return 0;
}
