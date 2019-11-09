#include <stdio.h>

struct Packet {
  int x;
};

void add_one_with_print (struct Packet pkt)
{
  pkt.x = pkt.x + 1; 
  printf("Result: %d\n", pkt.x);
}

