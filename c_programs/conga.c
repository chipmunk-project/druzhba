# include <stdio.h>
# define num_state_vars 3
struct packet {
  int pkt0;
  int pkt1;
  int pkt2;
  int pkt3;
  int pkt4;
};
int state [num_state_vars] = {0};

void write_results(struct packet pkt)
{
    FILE *fp;

    fp = fopen("results.txt", "w+");
    fprintf(fp, "%d, %d, %d\n", pkt.pkt0, pkt.pkt1, pkt.pkt2, pkt3, pkt4);
    for (int i = 0; i< num_state_vars; i++){
       if (i == num_state_vars - 1 )
         fprintf(fp, "%d\n", state[i]);
       else
         fprintf(fp, "%d,", state[i]);
    }
    fclose(fp);
}
void conga (struct packet pkt)
{
  // TODO: Complete. not finishe
  /*
  if (pkt.x < 0) {
   pkt.x = 0;
  }
  else {
    pkt.y = pkt.x;
  }
  if pkt.x < 0 {
    pkt.z = 0;
  }
  else {
    pkt.z = pkt.x;
  }
  if pkt.
*/
  write_results (pkt);
}


