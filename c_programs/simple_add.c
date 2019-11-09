struct Packet {
  int x;
  int y;
  int z;
};

void simple_add (struct Packet pkt)
{
  pkt.x = pkt.x + 1; 
  pkt.y = pkt.y + 2; 
  pkt.z = pkt.z + 3; 
}

