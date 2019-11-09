#include <stdio.h>

struct Packet {
  int x;
  int y;
  int z;
};

void simple_add_with_print (struct Packet pkt)
{
  pkt.x = pkt.x + 1; 
  pkt.y = pkt.y + 2; 
  pkt.z = pkt.z + 3; 
  printf("Result: %d, %d, %d\n", pkt.x, pkt.y, pkt.z);
}

