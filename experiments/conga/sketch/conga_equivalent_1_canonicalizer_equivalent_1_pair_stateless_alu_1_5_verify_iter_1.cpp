#include <cstdio>
#include <assert.h>
#include <iostream>
using namespace std;
#include "vops.h"
#include "conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_verify_iter_1.h"
namespace ANONYMOUS{

StateGroup* StateGroup::create(int  state_0_, int  state_1_){
  void* temp= malloc( sizeof(StateGroup)  ); 
  StateGroup* rv = new (temp)StateGroup();
  rv->state_0 =  state_0_;
  rv->state_1 =  state_1_;
  return rv;
}
StateAndPacket* StateAndPacket::create(int  pkt_0_, int  pkt_1_, int  pkt_2_, int  pkt_3_, int  pkt_4_, int  state_group_0_state_0_, int  state_group_0_state_1_){
  void* temp= malloc( sizeof(StateAndPacket)  ); 
  StateAndPacket* rv = new (temp)StateAndPacket();
  rv->pkt_0 =  pkt_0_;
  rv->pkt_1 =  pkt_1_;
  rv->pkt_2 =  pkt_2_;
  rv->pkt_3 =  pkt_3_;
  rv->pkt_4 =  pkt_4_;
  rv->state_group_0_state_0 =  state_group_0_state_0_;
  rv->state_group_0_state_1 =  state_group_0_state_1_;
  return rv;
}
void main__Wrapper(int pkt_0, int pkt_1, int pkt_2, int pkt_3, int pkt_4, int state_group_0_state_0, int state_group_0_state_1) {
  bool _tt0[4] = {0, 0, 0, 0};
  int*  constant_vector__ANONYMOUS_s230= new int [4]; CopyArr<int >(constant_vector__ANONYMOUS_s230,_tt0, 4, 4);
  glblInit_constant_vector__ANONYMOUS_s232(constant_vector__ANONYMOUS_s230);
  _main(pkt_0, pkt_1, pkt_2, pkt_3, pkt_4, state_group_0_state_0, state_group_0_state_1, constant_vector__ANONYMOUS_s230);
  delete[] constant_vector__ANONYMOUS_s230;
}
void main__WrapperNospec(int pkt_0, int pkt_1, int pkt_2, int pkt_3, int pkt_4, int state_group_0_state_0, int state_group_0_state_1) {}
void glblInit_constant_vector__ANONYMOUS_s232(int* constant_vector__ANONYMOUS_s231/* len = 4 */) {
  int _tt1[4] = {0, 1, 2, 3};
  CopyArr<int >(constant_vector__ANONYMOUS_s231,_tt1, 4, 4);
}
void _main(int pkt_0, int pkt_1, int pkt_2, int pkt_3, int pkt_4, int state_group_0_state_0, int state_group_0_state_1, int* constant_vector__ANONYMOUS_s229/* len = 4 */) {
  pkt_0 = 9;
  pkt_1 = 8;
  pkt_2 = 20;
  pkt_3 = 13;
  pkt_4 = 6;
  state_group_0_state_0 = 67;
  state_group_0_state_1 = 12;
  std::cout<<"pkt0 = " <<pkt_0 << " pkt1 = " << pkt_1<< " pkt2 = " << pkt_2 <<" pkt_3 = "<<pkt_3 <<" pkt4 = " <<pkt_4 <<" state00 = " << state_group_0_state_0 <<" state01 = " << state_group_0_state_1<<"\n";
  int  pipeline_result_s1_state_group_0_state_0_s252=0;
  int  pipeline_result_s1_state_group_0_state_1_s253=0;
  int  pipeline_result_s1_pkt_0_s247=0;
  int  pipeline_result_s1_pkt_1_s248=0;
  int  pipeline_result_s1_pkt_2_s249=0;
  int  pipeline_result_s1_pkt_3_s250=0;
  int  pipeline_result_s1_pkt_4_s251=0;
  pipeline(pkt_0, pkt_1, pkt_2, pkt_3, pkt_4, state_group_0_state_0, state_group_0_state_1, pipeline_result_s1_pkt_0_s247, pipeline_result_s1_pkt_1_s248, pipeline_result_s1_pkt_2_s249, pipeline_result_s1_pkt_3_s250, pipeline_result_s1_pkt_4_s251, pipeline_result_s1_state_group_0_state_0_s252, pipeline_result_s1_state_group_0_state_1_s253, constant_vector__ANONYMOUS_s229);
  std::cout<<"pkt 0 = " <<pipeline_result_s1_pkt_0_s247 <<" pkt 1 = " << pipeline_result_s1_pkt_1_s248 << " pkt 2 = " << pipeline_result_s1_pkt_2_s249 << " pkt 3 = " << pipeline_result_s1_pkt_3_s250 << " pkt 4 = " << pipeline_result_s1_pkt_4_s251 << " state00 = " << pipeline_result_s1_state_group_0_state_0_s252 << " state01 = " << pipeline_result_s1_state_group_0_state_1_s253<<"\n\n";
  int  program_result_s3_state_group_0_state_0_s266=0;
  int  program_result_s3_state_group_0_state_1_s267=0;
  int  program_result_s3_pkt_0_s261=0;
  int  program_result_s3_pkt_1_s262=0;
  int  program_result_s3_pkt_2_s263=0;
  int  program_result_s3_pkt_3_s264=0;
  int  program_result_s3_pkt_4_s265=0;
  program(pkt_0, pkt_1, pkt_2, pkt_3, pkt_4, state_group_0_state_0, state_group_0_state_1, program_result_s3_pkt_0_s261, program_result_s3_pkt_1_s262, program_result_s3_pkt_2_s263, program_result_s3_pkt_3_s264, program_result_s3_pkt_4_s265, program_result_s3_state_group_0_state_0_s266, program_result_s3_state_group_0_state_1_s267);
  assert ((pipeline_result_s1_state_group_0_state_0_s252) == (program_result_s3_state_group_0_state_0_s266));;
  assert ((pipeline_result_s1_state_group_0_state_1_s253) == (program_result_s3_state_group_0_state_1_s267));;
  assert ((pipeline_result_s1_pkt_0_s247) == (program_result_s3_pkt_0_s261));;
  assert ((pipeline_result_s1_pkt_1_s248) == (program_result_s3_pkt_1_s262));;
  assert ((pipeline_result_s1_pkt_2_s249) == (program_result_s3_pkt_2_s263));;
  assert ((pipeline_result_s1_pkt_3_s250) == (program_result_s3_pkt_3_s264));;
  assert ((pipeline_result_s1_pkt_4_s251) == (program_result_s3_pkt_4_s265));;
}
void pipeline(int state_and_packet_pkt_0_s282, int state_and_packet_pkt_1_s283, int state_and_packet_pkt_2_s284, int state_and_packet_pkt_3_s285, int state_and_packet_pkt_4_s286, int state_and_packet_state_group_0_state_0_s287, int state_and_packet_state_group_0_state_1_s288, int& _out_pkt_0_s289, int& _out_pkt_1_s290, int& _out_pkt_2_s291, int& _out_pkt_3_s292, int& _out_pkt_4_s293, int& _out_state_group_0_state_0_s294, int& _out_state_group_0_state_1_s295, int* constant_vector__ANONYMOUS_s227/* len = 4 */) {
  int  destination_0_0_s5=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0(state_and_packet_pkt_0_s282, state_and_packet_pkt_1_s283, state_and_packet_pkt_2_s284, state_and_packet_pkt_3_s285, state_and_packet_pkt_4_s286, 14, 2, 0, 0, 0, destination_0_0_s5, constant_vector__ANONYMOUS_s227);
  int  destination_0_1_s7=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1(state_and_packet_pkt_0_s282, state_and_packet_pkt_1_s283, state_and_packet_pkt_2_s284, state_and_packet_pkt_3_s285, state_and_packet_pkt_4_s286, 4, 0, 0, 3, 0, destination_0_1_s7, constant_vector__ANONYMOUS_s227);
  int  destination_0_2_s9=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2(state_and_packet_pkt_0_s282, state_and_packet_pkt_1_s283, state_and_packet_pkt_2_s284, state_and_packet_pkt_3_s285, state_and_packet_pkt_4_s286, 2, 0, 0, 6, 3, destination_0_2_s9, constant_vector__ANONYMOUS_s227);
  int  destination_0_3_s11=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3(state_and_packet_pkt_0_s282, state_and_packet_pkt_1_s283, state_and_packet_pkt_2_s284, state_and_packet_pkt_3_s285, state_and_packet_pkt_4_s286, 14, 3, 2, 3, 3, destination_0_3_s11, constant_vector__ANONYMOUS_s227);
  int  destination_0_4_s13=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4(state_and_packet_pkt_0_s282, state_and_packet_pkt_1_s283, state_and_packet_pkt_2_s284, state_and_packet_pkt_3_s285, state_and_packet_pkt_4_s286, 14, 1, 2, 6, 4, destination_0_4_s13, constant_vector__ANONYMOUS_s227);
  int  packet_operand_salu0_0_0_s15=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_0(state_and_packet_pkt_0_s282, state_and_packet_pkt_1_s283, state_and_packet_pkt_2_s284, state_and_packet_pkt_3_s285, state_and_packet_pkt_4_s286, 6, packet_operand_salu0_0_0_s15);
  int  packet_operand_salu0_0_1_s17=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_1(state_and_packet_pkt_0_s282, state_and_packet_pkt_1_s283, state_and_packet_pkt_2_s284, state_and_packet_pkt_3_s285, state_and_packet_pkt_4_s286, 3, packet_operand_salu0_0_1_s17);
  int  packet_operand_salu0_0_2_s19=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_2(state_and_packet_pkt_0_s282, state_and_packet_pkt_1_s283, state_and_packet_pkt_2_s284, state_and_packet_pkt_3_s285, state_and_packet_pkt_4_s286, 0, packet_operand_salu0_0_2_s19);
  int  packet_operand_salu0_0_3_s21=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_3(state_and_packet_pkt_0_s282, state_and_packet_pkt_1_s283, state_and_packet_pkt_2_s284, state_and_packet_pkt_3_s285, state_and_packet_pkt_4_s286, 0, packet_operand_salu0_0_3_s21);
  int  packet_operand_salu0_0_4_s23=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_4(state_and_packet_pkt_0_s282, state_and_packet_pkt_1_s283, state_and_packet_pkt_2_s284, state_and_packet_pkt_3_s285, state_and_packet_pkt_4_s286, 0, packet_operand_salu0_0_4_s23);
  int  state_operand_salu_0_0_state_1_s297=state_and_packet_state_group_0_state_1_s288;
  int  state_operand_salu_0_0_state_0_s296=state_and_packet_state_group_0_state_0_s287;
  int  old_state_group_0_0_s25_state_0_s300=0;
  int  old_state_group_0_0_s25_state_1_s301=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0(state_operand_salu_0_0_state_0_s296, state_operand_salu_0_0_state_1_s297, 0, packet_operand_salu0_0_0_s15, packet_operand_salu0_0_1_s17, packet_operand_salu0_0_2_s19, packet_operand_salu0_0_3_s21, packet_operand_salu0_0_4_s23, 1, 2, 2, 0, 2, 0, 0, 0, 1, 0, 2, 1, 0, 1, 2, 3, 0, 3, 3, 2, 2, 2, 0, 0, 2, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1, 1, 0, 0, 2, 3, 0, 3, 0, 2, 2, old_state_group_0_0_s25_state_0_s300, old_state_group_0_0_s25_state_1_s301, constant_vector__ANONYMOUS_s227);
  int  output_0_0_s27=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_0(old_state_group_0_0_s25_state_0_s300, old_state_group_0_0_s25_state_1_s301, destination_0_0_s5, 3, output_0_0_s27);
  int  output_0_1_s29=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_1(old_state_group_0_0_s25_state_0_s300, old_state_group_0_0_s25_state_1_s301, destination_0_1_s7, 3, output_0_1_s29);
  int  output_0_2_s31=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_2(old_state_group_0_0_s25_state_0_s300, old_state_group_0_0_s25_state_1_s301, destination_0_2_s9, 2, output_0_2_s31);
  int  output_0_3_s33=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_3(old_state_group_0_0_s25_state_0_s300, old_state_group_0_0_s25_state_1_s301, destination_0_3_s11, 2, output_0_3_s33);
  int  output_0_4_s35=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_4(old_state_group_0_0_s25_state_0_s300, old_state_group_0_0_s25_state_1_s301, destination_0_4_s13, 3, output_0_4_s35);
  _out_pkt_0_s289 = output_0_0_s27;
  _out_pkt_1_s290 = output_0_1_s29;
  _out_pkt_2_s291 = output_0_2_s31;
  _out_pkt_3_s292 = output_0_3_s33;
  _out_pkt_4_s293 = output_0_4_s35;
  _out_state_group_0_state_0_s294 = state_operand_salu_0_0_state_0_s296;
  _out_state_group_0_state_1_s295 = state_operand_salu_0_0_state_1_s297;
  return;
}
void program(int state_and_packet_pkt_0_s268, int state_and_packet_pkt_1_s269, int state_and_packet_pkt_2_s270, int state_and_packet_pkt_3_s271, int state_and_packet_pkt_4_s272, int state_and_packet_state_group_0_state_0_s273, int state_and_packet_state_group_0_state_1_s274, int& _out_pkt_0_s275, int& _out_pkt_1_s276, int& _out_pkt_2_s277, int& _out_pkt_3_s278, int& _out_pkt_4_s279, int& _out_state_group_0_state_0_s280, int& _out_state_group_0_state_1_s281) {
  state_and_packet_pkt_1_s269 = ((state_and_packet_pkt_0_s268) < (0) ? ((int) (0)) : state_and_packet_pkt_0_s268);
  state_and_packet_pkt_2_s270 = ((state_and_packet_pkt_0_s268) < (0) ? ((int) (0)) : state_and_packet_pkt_0_s268);
  if (!(!(!((state_and_packet_pkt_3_s271) < (state_and_packet_state_group_0_state_0_s273))))) {
    if (!((state_and_packet_pkt_3_s271) < (state_and_packet_state_group_0_state_0_s273))) {
      if ((state_and_packet_pkt_4_s272) == (state_and_packet_state_group_0_state_1_s274)) {
        state_and_packet_state_group_0_state_0_s273 = state_and_packet_pkt_3_s271;
      }
    }
  } else {
    if (!(!((state_and_packet_pkt_3_s271) < (state_and_packet_state_group_0_state_0_s273)))) {
      if ((state_and_packet_pkt_3_s271) < (state_and_packet_state_group_0_state_0_s273)) {
        state_and_packet_state_group_0_state_0_s273 = state_and_packet_pkt_3_s271;
        state_and_packet_state_group_0_state_1_s274 = state_and_packet_pkt_4_s272;
      }
    }
  }
  _out_pkt_0_s275 = state_and_packet_pkt_0_s268;
  _out_pkt_1_s276 = state_and_packet_pkt_1_s269;
  _out_pkt_2_s277 = state_and_packet_pkt_2_s270;
  _out_pkt_3_s278 = state_and_packet_pkt_3_s271;
  _out_pkt_4_s279 = state_and_packet_pkt_4_s272;
  _out_state_group_0_state_0_s280 = state_and_packet_state_group_0_state_0_s273;
  _out_state_group_0_state_1_s281 = state_and_packet_state_group_0_state_1_s274;
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s208/* len = 4 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s208[immediate_operand_hole_local]);
  int  pkt_0_s197=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s197);
  int  pkt_1_s199=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s199);
  int  pkt_2_s201=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s201);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s197 + pkt_1_s199;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s197 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s197 - pkt_1_s199;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s197 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s197;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s197) != (pkt_1_s199);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s197) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s197) == (pkt_1_s199);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s197) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s197) >= (pkt_1_s199);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s197) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s197) < (pkt_1_s199);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s197) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s197) != (0)) {
                                  _out = pkt_1_s199;
                                  return;
                                } else {
                                  _out = pkt_2_s201;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s197) != (0)) {
                                    _out = pkt_1_s199;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s197) != (0)) || ((pkt_1_s199) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s197) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s197) != (0)) && ((pkt_1_s199) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s197) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s197) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s204/* len = 4 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s204[immediate_operand_hole_local]);
  int  pkt_0_s191=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s191);
  int  pkt_1_s193=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s193);
  int  pkt_2_s195=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s195);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s191 + pkt_1_s193;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s191 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s191 - pkt_1_s193;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s191 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s191;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s191) != (pkt_1_s193);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s191) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s191) == (pkt_1_s193);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s191) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s191) >= (pkt_1_s193);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s191) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s191) < (pkt_1_s193);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s191) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s191) != (0)) {
                                  _out = pkt_1_s193;
                                  return;
                                } else {
                                  _out = pkt_2_s195;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s191) != (0)) {
                                    _out = pkt_1_s193;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s191) != (0)) || ((pkt_1_s193) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s191) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s191) != (0)) && ((pkt_1_s193) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s191) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s191) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s213/* len = 4 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s213[immediate_operand_hole_local]);
  int  pkt_0_s185=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s185);
  int  pkt_1_s187=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s187);
  int  pkt_2_s189=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s189);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s185 + pkt_1_s187;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s185 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s185 - pkt_1_s187;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s185 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s185;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s185) != (pkt_1_s187);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s185) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s185) == (pkt_1_s187);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s185) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s185) >= (pkt_1_s187);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s185) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s185) < (pkt_1_s187);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s185) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s185) != (0)) {
                                  _out = pkt_1_s187;
                                  return;
                                } else {
                                  _out = pkt_2_s189;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s185) != (0)) {
                                    _out = pkt_1_s187;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s185) != (0)) || ((pkt_1_s187) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s185) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s185) != (0)) && ((pkt_1_s187) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s185) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s185) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s222/* len = 4 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s222[immediate_operand_hole_local]);
  int  pkt_0_s179=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s179);
  int  pkt_1_s181=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s181);
  int  pkt_2_s183=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s183);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s179 + pkt_1_s181;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s179 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s179 - pkt_1_s181;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s179 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s179;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s179) != (pkt_1_s181);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s179) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s179) == (pkt_1_s181);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s179) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s179) >= (pkt_1_s181);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s179) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s179) < (pkt_1_s181);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s179) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s179) != (0)) {
                                  _out = pkt_1_s181;
                                  return;
                                } else {
                                  _out = pkt_2_s183;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s179) != (0)) {
                                    _out = pkt_1_s181;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s179) != (0)) || ((pkt_1_s181) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s179) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s179) != (0)) && ((pkt_1_s181) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s179) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s179) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s210/* len = 4 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s210[immediate_operand_hole_local]);
  int  pkt_0_s173=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s173);
  int  pkt_1_s175=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s175);
  int  pkt_2_s177=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s177);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s173 + pkt_1_s175;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s173 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s173 - pkt_1_s175;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s173 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s173;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s173) != (pkt_1_s175);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s173) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s173) == (pkt_1_s175);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s173) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s173) >= (pkt_1_s175);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s173) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s173) < (pkt_1_s175);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s173) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s173) != (0)) {
                                  _out = pkt_1_s175;
                                  return;
                                } else {
                                  _out = pkt_2_s177;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s173) != (0)) {
                                    _out = pkt_1_s175;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s173) != (0)) || ((pkt_1_s175) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s173) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s173) != (0)) && ((pkt_1_s175) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s173) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s173) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_0(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_0_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_0_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_0_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_0_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_1(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_1_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_2(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_2_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_3(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_3_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_4(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_4_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_4_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_4_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_4_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_4_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0(int& state_group_state_0_s302, int& state_group_state_1_s303, int output_mux_0, int pkt_1, int pkt_2, int pkt_3, int pkt_4, int pkt_5, int Mux3_0, int Mux3_1, int Mux3_10, int Mux3_11, int Mux3_12, int Mux3_13, int Mux3_14, int Mux3_15, int Mux3_16, int Mux3_17, int Mux3_18, int Mux3_19, int Mux3_2, int Mux3_20, int Mux3_21, int Mux3_22, int Mux3_23, int Mux3_24, int Mux3_25, int Mux3_26, int Mux3_27, int Mux3_3, int Mux3_4, int Mux3_5, int Mux3_6, int Mux3_7, int Mux3_8, int Mux3_9, int Opt_0, int Opt_1, int Opt_2, int Opt_3, int Opt_4, int Opt_5, int Opt_6, int Opt_7, int arith_op_0, int arith_op_1, int arith_op_2, int arith_op_3, int arith_op_4, int arith_op_5, int arith_op_6, int arith_op_7, int const_0, int const_1, int const_10, int const_11, int const_12, int const_13, int const_14, int const_15, int const_16, int const_17, int const_18, int const_19, int const_2, int const_3, int const_4, int const_5, int const_6, int const_7, int const_8, int const_9, int output_mux, int rel_op_0, int rel_op_1, int rel_op_2, int rel_op_3, int& _out_state_0_s304, int& _out_state_1_s305, int* constant_vector__ANONYMOUS_s228/* len = 4 */) {
  int  old_state_group_state_0_s306=state_group_state_0_s302;
  int  old_state_group_state_1_s307=state_group_state_1_s303;
  int  state_0=0;
  state_0 = state_group_state_0_s302;
  int  state_1=0;
  state_1 = state_group_state_1_s303;
  int  _out_s37=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_0(state_group_state_0_s302, state_group_state_1_s303, Mux3_0, _out_s37);
  int  _out_s39=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_1(pkt_1, pkt_2, Mux3_1, _out_s39);
  int  _out_s41=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_2(pkt_1, pkt_2, Mux3_2, _out_s41);
  int  _out_s43=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_0(const_0, _out_s43, constant_vector__ANONYMOUS_s228);
  int  _out_s45=0;
  conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_rel_op_0((_out_s37 + _out_s39) - _out_s41, _out_s43, rel_op_0, _out_s45);
  if ((_out_s45) == (1)) {
    int  _out_s47=0;
    conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_3(state_group_state_0_s302, state_group_state_1_s303, Mux3_3, _out_s47);
    int  _out_s49=0;
    conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_4(pkt_1, pkt_2, Mux3_4, _out_s49);
    int  _out_s51=0;
    conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_5(pkt_1, pkt_2, Mux3_5, _out_s51);
    int  _out_s53=0;
    conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_1(const_1, _out_s53, constant_vector__ANONYMOUS_s228);
    int  _out_s55=0;
    conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_rel_op_1((_out_s47 + _out_s49) - _out_s51, _out_s53, rel_op_1, _out_s55);
    if ((_out_s55) == (1)) {
      int  state_0_s57=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_0(state_group_state_0_s302, Opt_0, state_0_s57);
      int  state_0_s59=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_2(const_2, state_0_s59, constant_vector__ANONYMOUS_s228);
      int  state_0_s61=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_6(pkt_1, pkt_2, state_0_s59, Mux3_6, state_0_s61);
      int  state_0_s63=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_3(const_3, state_0_s63, constant_vector__ANONYMOUS_s228);
      int  state_0_s65=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_7(pkt_1, pkt_2, state_0_s63, Mux3_7, state_0_s65);
      int  state_0_s67=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_0(state_0_s61, state_0_s65, arith_op_0, state_0_s67);
      state_0 = state_0_s57 + state_0_s67;
      int  state_1_s69=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_1(state_group_state_1_s303, Opt_1, state_1_s69);
      int  state_1_s71=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_4(const_4, state_1_s71, constant_vector__ANONYMOUS_s228);
      int  state_1_s73=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_8(pkt_1, pkt_2, state_1_s71, Mux3_8, state_1_s73);
      int  state_1_s75=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_5(const_5, state_1_s75, constant_vector__ANONYMOUS_s228);
      int  state_1_s77=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_9(pkt_1, pkt_2, state_1_s75, Mux3_9, state_1_s77);
      int  state_1_s79=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_1(state_1_s73, state_1_s77, arith_op_1, state_1_s79);
      state_1 = state_1_s69 + state_1_s79;
    } else {
      int  state_0_s81=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_2(state_group_state_0_s302, Opt_2, state_0_s81);
      int  state_0_s83=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_6(const_6, state_0_s83, constant_vector__ANONYMOUS_s228);
      int  state_0_s85=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_10(pkt_1, pkt_2, state_0_s83, Mux3_10, state_0_s85);
      int  state_0_s87=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_7(const_7, state_0_s87, constant_vector__ANONYMOUS_s228);
      int  state_0_s89=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_11(pkt_1, pkt_2, state_0_s87, Mux3_11, state_0_s89);
      int  state_0_s91=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_2(state_0_s85, state_0_s89, arith_op_2, state_0_s91);
      state_0 = state_0_s81 + state_0_s91;
      int  state_1_s93=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_3(state_group_state_1_s303, Opt_3, state_1_s93);
      int  state_1_s95=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_8(const_8, state_1_s95, constant_vector__ANONYMOUS_s228);
      int  state_1_s97=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_12(pkt_1, pkt_2, state_1_s95, Mux3_12, state_1_s97);
      int  state_1_s99=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_9(const_9, state_1_s99, constant_vector__ANONYMOUS_s228);
      int  state_1_s101=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_13(pkt_1, pkt_2, state_1_s99, Mux3_13, state_1_s101);
      int  state_1_s103=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_3(state_1_s97, state_1_s101, arith_op_3, state_1_s103);
      state_1 = state_1_s93 + state_1_s103;
    }
  } else {
    int  _out_s105=0;
    conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_14(state_group_state_0_s302, state_group_state_1_s303, Mux3_14, _out_s105);
    int  _out_s107=0;
    conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_15(pkt_1, pkt_2, Mux3_15, _out_s107);
    int  _out_s109=0;
    conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_16(pkt_1, pkt_2, Mux3_16, _out_s109);
    int  _out_s111=0;
    conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_10(const_10, _out_s111, constant_vector__ANONYMOUS_s228);
    int  _out_s113=0;
    conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_rel_op_2((_out_s105 + _out_s107) - _out_s109, _out_s111, rel_op_2, _out_s113);
    if ((_out_s113) == (1)) {
      int  _out_s115=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_17(state_group_state_0_s302, state_group_state_1_s303, Mux3_17, _out_s115);
      int  _out_s117=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_18(pkt_1, pkt_2, Mux3_18, _out_s117);
      int  _out_s119=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_19(pkt_1, pkt_2, Mux3_19, _out_s119);
      int  _out_s121=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_11(const_11, _out_s121, constant_vector__ANONYMOUS_s228);
      int  _out_s123=0;
      conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_rel_op_3((_out_s115 + _out_s117) - _out_s119, _out_s121, rel_op_3, _out_s123);
      if ((_out_s123) == (1)) {
        int  state_0_s125=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_4(state_group_state_0_s302, Opt_4, state_0_s125);
        int  state_0_s127=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_12(const_12, state_0_s127, constant_vector__ANONYMOUS_s228);
        int  state_0_s129=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_20(pkt_1, pkt_2, state_0_s127, Mux3_20, state_0_s129);
        int  state_0_s131=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_13(const_13, state_0_s131, constant_vector__ANONYMOUS_s228);
        int  state_0_s133=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_21(pkt_1, pkt_2, state_0_s131, Mux3_21, state_0_s133);
        int  state_0_s135=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_4(state_0_s129, state_0_s133, arith_op_4, state_0_s135);
        state_0 = state_0_s125 + state_0_s135;
        int  state_1_s137=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_5(state_group_state_1_s303, Opt_5, state_1_s137);
        int  state_1_s139=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_14(const_14, state_1_s139, constant_vector__ANONYMOUS_s228);
        int  state_1_s141=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_22(pkt_1, pkt_2, state_1_s139, Mux3_22, state_1_s141);
        int  state_1_s143=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_15(const_15, state_1_s143, constant_vector__ANONYMOUS_s228);
        int  state_1_s145=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_23(pkt_1, pkt_2, state_1_s143, Mux3_23, state_1_s145);
        int  state_1_s147=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_5(state_1_s141, state_1_s145, arith_op_5, state_1_s147);
        state_1 = state_1_s137 + state_1_s147;
      } else {
        int  state_0_s149=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_6(state_group_state_0_s302, Opt_6, state_0_s149);
        int  state_0_s151=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_16(const_16, state_0_s151, constant_vector__ANONYMOUS_s228);
        int  state_0_s153=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_24(pkt_1, pkt_2, state_0_s151, Mux3_24, state_0_s153);
        int  state_0_s155=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_17(const_17, state_0_s155, constant_vector__ANONYMOUS_s228);
        int  state_0_s157=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_25(pkt_1, pkt_2, state_0_s155, Mux3_25, state_0_s157);
        int  state_0_s159=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_6(state_0_s153, state_0_s157, arith_op_6, state_0_s159);
        state_0 = state_0_s149 + state_0_s159;
        int  state_1_s161=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_7(state_group_state_1_s303, Opt_7, state_1_s161);
        int  state_1_s163=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_18(const_18, state_1_s163, constant_vector__ANONYMOUS_s228);
        int  state_1_s165=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_26(pkt_1, pkt_2, state_1_s163, Mux3_26, state_1_s165);
        int  state_1_s167=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_19(const_19, state_1_s167, constant_vector__ANONYMOUS_s228);
        int  state_1_s169=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_27(pkt_1, pkt_2, state_1_s167, Mux3_27, state_1_s169);
        int  state_1_s171=0;
        conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_7(state_1_s165, state_1_s169, arith_op_7, state_1_s171);
        state_1 = state_1_s161 + state_1_s171;
      }
    }
  }
  state_group_state_0_s302 = state_0;
  state_group_state_1_s303 = state_1;
  if ((output_mux_0) == (1)) {
    _out_state_0_s304 = old_state_group_state_0_s306;
    _out_state_1_s305 = old_state_group_state_1_s307;
    return;
  } else {
    _out_state_0_s304 = state_0;
    _out_state_1_s305 = state_1;
    return;
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_0(int input0, int input1, int input2, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_0_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_0_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      _out = input2;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_1(int input0, int input1, int input2, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_1_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      _out = input2;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_2(int input0, int input1, int input2, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_2_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      _out = input2;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_3(int input0, int input1, int input2, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_3_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      _out = input2;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_4(int input0, int input1, int input2, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_4_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_4_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_4_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      _out = input2;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux1(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux1_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux2(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux2_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux3(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux3_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux1(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux1_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux2(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux2_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux3(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux3_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux1(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux1_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux2(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux2_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux3(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux3_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux1(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux1_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux2(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux2_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux3(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux3_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux1(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux1_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux2(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux2_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux3(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux3_ctrl_local, int& _out) {
  if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_0(int op1, int op2, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = 0;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_1(int op1, int op2, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = 0;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_2(int op1, int op2, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = 0;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_0(int const_var, int& _out, int* constant_vector__ANONYMOUS_s225/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s225[const_var]);
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_rel_op_0(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = ((operand1) != (operand2) ? 1 : 0);
    return;
  } else {
    if ((opcode) == (1)) {
      _out = ((operand1) < (operand2) ? 1 : 0);
      return;
    } else {
      if ((opcode) == (2)) {
        _out = ((operand1) > (operand2) ? 1 : 0);
        return;
      } else {
        _out = ((operand1) == (operand2) ? 1 : 0);
        return;
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_3(int op1, int op2, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = 0;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_4(int op1, int op2, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = 0;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_5(int op1, int op2, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = 0;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_1(int const_var, int& _out, int* constant_vector__ANONYMOUS_s207/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s207[const_var]);
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_rel_op_1(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = ((operand1) != (operand2) ? 1 : 0);
    return;
  } else {
    if ((opcode) == (1)) {
      _out = ((operand1) < (operand2) ? 1 : 0);
      return;
    } else {
      if ((opcode) == (2)) {
        _out = ((operand1) > (operand2) ? 1 : 0);
        return;
      } else {
        _out = ((operand1) == (operand2) ? 1 : 0);
        return;
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_0(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_2(int const_var, int& _out, int* constant_vector__ANONYMOUS_s214/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s214[const_var]);
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_6(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_3(int const_var, int& _out, int* constant_vector__ANONYMOUS_s215/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s215[const_var]);
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_7(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_0(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_1(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_4(int const_var, int& _out, int* constant_vector__ANONYMOUS_s205/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s205[const_var]);
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_8(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_5(int const_var, int& _out, int* constant_vector__ANONYMOUS_s212/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s212[const_var]);
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_9(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_1(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_2(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_6(int const_var, int& _out, int* constant_vector__ANONYMOUS_s203/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s203[const_var]);
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_10(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_7(int const_var, int& _out, int* constant_vector__ANONYMOUS_s216/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s216[const_var]);
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_11(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_2(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_3(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_8(int const_var, int& _out, int* constant_vector__ANONYMOUS_s226/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s226[const_var]);
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_12(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_9(int const_var, int& _out, int* constant_vector__ANONYMOUS_s206/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s206[const_var]);
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_13(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_3(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_14(int op1, int op2, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = 0;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_15(int op1, int op2, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = 0;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_16(int op1, int op2, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = 0;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_10(int const_var, int& _out, int* constant_vector__ANONYMOUS_s209/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s209[const_var]);
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_rel_op_2(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = ((operand1) != (operand2) ? 1 : 0);
    return;
  } else {
    if ((opcode) == (1)) {
      _out = ((operand1) < (operand2) ? 1 : 0);
      return;
    } else {
      if ((opcode) == (2)) {
        _out = ((operand1) > (operand2) ? 1 : 0);
        return;
      } else {
        _out = ((operand1) == (operand2) ? 1 : 0);
        return;
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_17(int op1, int op2, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = 0;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_18(int op1, int op2, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = 0;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_19(int op1, int op2, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = 0;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_11(int const_var, int& _out, int* constant_vector__ANONYMOUS_s219/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s219[const_var]);
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_rel_op_3(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = ((operand1) != (operand2) ? 1 : 0);
    return;
  } else {
    if ((opcode) == (1)) {
      _out = ((operand1) < (operand2) ? 1 : 0);
      return;
    } else {
      if ((opcode) == (2)) {
        _out = ((operand1) > (operand2) ? 1 : 0);
        return;
      } else {
        _out = ((operand1) == (operand2) ? 1 : 0);
        return;
      }
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_4(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_12(int const_var, int& _out, int* constant_vector__ANONYMOUS_s211/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s211[const_var]);
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_20(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_13(int const_var, int& _out, int* constant_vector__ANONYMOUS_s224/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s224[const_var]);
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_21(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_4(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_5(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_14(int const_var, int& _out, int* constant_vector__ANONYMOUS_s221/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s221[const_var]);
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_22(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_15(int const_var, int& _out, int* constant_vector__ANONYMOUS_s220/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s220[const_var]);
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_23(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_5(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_6(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_16(int const_var, int& _out, int* constant_vector__ANONYMOUS_s217/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s217[const_var]);
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_24(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_17(int const_var, int& _out, int* constant_vector__ANONYMOUS_s218/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s218[const_var]);
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_25(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_6(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_7(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_18(int const_var, int& _out, int* constant_vector__ANONYMOUS_s202/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s202[const_var]);
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_26(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_19(int const_var, int& _out, int* constant_vector__ANONYMOUS_s223/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s223[const_var]);
  return;
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_27(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_7(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}

}
