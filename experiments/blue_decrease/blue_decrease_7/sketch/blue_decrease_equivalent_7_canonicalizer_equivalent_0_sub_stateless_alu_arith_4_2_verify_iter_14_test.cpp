#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <iostream>
#include "vops.h"
#include "blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_verify_iter_14.h"

using namespace std;

void main__Wrapper_ANONYMOUSTest(Parameters& _p_) {
  for(int _test_=0;_test_< _p_.niters ;_test_++) {
    int  pkt_0;
    pkt_0=abs(rand()) % 100;
    if(_p_.verbosity > 2){
      cout<<"pkt_0="<<pkt_0<<endl;
    }
    int  pkt_1;
    pkt_1=abs(rand()) % 100;
    if(_p_.verbosity > 2){
      cout<<"pkt_1="<<pkt_1<<endl;
    }
    int  state_group_0_state_0;
    state_group_0_state_0=abs(rand()) % 100;
    if(_p_.verbosity > 2){
      cout<<"state_group_0_state_0="<<state_group_0_state_0<<endl;
    }
    int  state_group_1_state_0;
    state_group_1_state_0=abs(rand()) % 100;
    if(_p_.verbosity > 2){
      cout<<"state_group_1_state_0="<<state_group_1_state_0<<endl;
    }
    try{
      ANONYMOUS::main__WrapperNospec(pkt_0,pkt_1,state_group_0_state_0,state_group_1_state_0);
      ANONYMOUS::main__Wrapper(pkt_0,pkt_1,state_group_0_state_0,state_group_1_state_0);
    }catch(AssumptionFailedException& afe){  }
  }
}

int main(int argc, char** argv) {
  Parameters p(argc, argv);
  srand(time(0));
  main__Wrapper_ANONYMOUSTest(p);
  printf("Automated testing passed for blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_verify_iter_14\n");
  return 0;
}
