#include <stdio.h>

struct Packet {
  int x;
  int y;
  int z;
};

void add (struct Packet pkt)
{
  pkt.x = pkt.x + 1; 
  pkt.y = pkt.y + 2; 
  pkt.z = pkt.z + 3; 
  printf("Packet: %d %d %d\n", pkt.x, pkt.y, pkt.z);
}

void main ()
{
  struct Packet pkt = { 0, 0, 0 };
  add (pkt);
}
