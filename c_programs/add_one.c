#include <stdio.h>

struct Packet {
  int x;
};

void add_one (struct Packet pkt)
{
  pkt.x = pkt.x + 1; 
  printf("Result: %d\n", pkt.x);
}

int main ()
{
  struct Packet p = { 0 };
  add_one(p);

}
