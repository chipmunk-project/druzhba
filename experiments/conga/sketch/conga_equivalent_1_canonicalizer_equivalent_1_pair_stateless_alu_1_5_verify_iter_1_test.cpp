#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <iostream>
#include "vops.h"
#include "conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_verify_iter_1.h"

using namespace std;

void main__Wrapper_ANONYMOUSTest(Parameters& _p_) {
  for(int _test_=0;_test_< _p_.niters ;_test_++) {
    int  pkt_0;
    pkt_0=abs(rand()) % 32;
    if(_p_.verbosity > 2){
      cout<<"pkt_0="<<pkt_0<<endl;
    }
    int  pkt_1;
    pkt_1=abs(rand()) % 32;
    if(_p_.verbosity > 2){
      cout<<"pkt_1="<<pkt_1<<endl;
    }
    int  pkt_2;
    pkt_2=abs(rand()) % 32;
    if(_p_.verbosity > 2){
      cout<<"pkt_2="<<pkt_2<<endl;
    }
    int  pkt_3;
    pkt_3=abs(rand()) % 32;
    if(_p_.verbosity > 2){
      cout<<"pkt_3="<<pkt_3<<endl;
    }
    int  pkt_4;
    pkt_4=abs(rand()) % 32;
    if(_p_.verbosity > 2){
      cout<<"pkt_4="<<pkt_4<<endl;
    }
    int  state_group_0_state_0;
    state_group_0_state_0=abs(rand()) % 32;
    if(_p_.verbosity > 2){
      cout<<"state_group_0_state_0="<<state_group_0_state_0<<endl;
    }
    int  state_group_0_state_1;
    state_group_0_state_1=abs(rand()) % 32;
    if(_p_.verbosity > 2){
      cout<<"state_group_0_state_1="<<state_group_0_state_1<<endl;
    }
    try{
      ANONYMOUS::main__WrapperNospec(pkt_0,pkt_1,pkt_2,pkt_3,pkt_4,state_group_0_state_0,state_group_0_state_1);
      ANONYMOUS::main__Wrapper(pkt_0,pkt_1,pkt_2,pkt_3,pkt_4,state_group_0_state_0,state_group_0_state_1);
    }catch(AssumptionFailedException& afe){  }
  }
}

int main(int argc, char** argv) {
  Parameters p(argc, argv);
  srand(time(0));
  main__Wrapper_ANONYMOUSTest(p);
  printf("Automated testing passed for conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_verify_iter_1\n");
  return 0;
}
