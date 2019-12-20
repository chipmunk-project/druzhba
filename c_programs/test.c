struct Packet {
  int x;
  int y;
  int z;
  int none;
  int none1;
  int none2;
  int none3;
};

void simple_add (struct Packet pkt)
{
  pkt.none = pkt.none + 3;
  pkt.none1 = pkt.none1 + 3;
  pkt.none2 = pkt.none2 + 3;
  pkt.none3 = pkt.none3 + 3;
  pkt.x = pkt.x + 1; 
  pkt.y = pkt.y + 2; 
  pkt.z = pkt.z + 3; 
}

int main ()
{
  struct Packet p = {2, 5, 2, 12, 5, 8, 3};
  simple_add (p);
 
  return 0;
}

