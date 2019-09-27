#include <cstdio>
#include <assert.h>
#include <iostream>
using namespace std;
#include "vops.h"
#include "snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_verify_iter_3.h"
namespace ANONYMOUS{

StateGroup* StateGroup::create(int  state_0_, int  state_1_){
  void* temp= malloc( sizeof(StateGroup)  ); 
  StateGroup* rv = new (temp)StateGroup();
  rv->state_0 =  state_0_;
  rv->state_1 =  state_1_;
  return rv;
}
StateAndPacket* StateAndPacket::create(int  pkt_0_, int  state_group_0_state_0_, int  state_group_0_state_1_){
  void* temp= malloc( sizeof(StateAndPacket)  ); 
  StateAndPacket* rv = new (temp)StateAndPacket();
  rv->pkt_0 =  pkt_0_;
  rv->state_group_0_state_0 =  state_group_0_state_0_;
  rv->state_group_0_state_1 =  state_group_0_state_1_;
  return rv;
}
void main__Wrapper(int pkt_0, int state_group_0_state_0, int state_group_0_state_1) {
  bool _tt0[7] = {0, 0, 0, 0, 0, 0, 0};
  int*  constant_vector__ANONYMOUS_s184= new int [7]; CopyArr<int >(constant_vector__ANONYMOUS_s184,_tt0, 7, 7);
  glblInit_constant_vector__ANONYMOUS_s188(constant_vector__ANONYMOUS_s184);
  _main(pkt_0, state_group_0_state_0, state_group_0_state_1, constant_vector__ANONYMOUS_s184);
  delete[] constant_vector__ANONYMOUS_s184;
}
void main__WrapperNospec(int pkt_0, int state_group_0_state_0, int state_group_0_state_1) {}
void glblInit_constant_vector__ANONYMOUS_s188(int* constant_vector__ANONYMOUS_s187/* len = 7 */) {
  int _tt1[7] = {0, 1, 2, 3, 998, 999, 1000};
  CopyArr<int >(constant_vector__ANONYMOUS_s187,_tt1, 7, 7);
}
void _main(int pkt_0, int state_group_0_state_0, int state_group_0_state_1, int* constant_vector__ANONYMOUS_s185/* len = 7 */) {
  pkt_0 = 27;
  state_group_0_state_0 = 999;
  state_group_0_state_1 = 0;
  std::cout<<"Pkt0: " <<pkt_0<<" state00: "<<state_group_0_state_0<<" state01: " <<state_group_0_state_1<<"\n";
  int  pipeline_result_s1_state_group_0_state_0_s196=0;
  int  pipeline_result_s1_state_group_0_state_1_s197=0;
  int  pipeline_result_s1_pkt_0_s195=0;
  pipeline(pkt_0, state_group_0_state_0, state_group_0_state_1, pipeline_result_s1_pkt_0_s195, pipeline_result_s1_state_group_0_state_0_s196, pipeline_result_s1_state_group_0_state_1_s197, constant_vector__ANONYMOUS_s185);
  std::cout<<"Result pkt_0: " <<pipeline_result_s1_pkt_0_s195 <<" state00: " << pipeline_result_s1_state_group_0_state_0_s196 << "state01: " << pipeline_result_s1_state_group_0_state_1_s197<<"\n\n\n";
  int  program_result_s3_state_group_0_state_0_s202=0;
  int  program_result_s3_state_group_0_state_1_s203=0;
  int  program_result_s3_pkt_0_s201=0;
  program(pkt_0, state_group_0_state_0, state_group_0_state_1, program_result_s3_pkt_0_s201, program_result_s3_state_group_0_state_0_s202, program_result_s3_state_group_0_state_1_s203);
  assert ((pipeline_result_s1_state_group_0_state_0_s196) == (program_result_s3_state_group_0_state_0_s202));;
  assert ((pipeline_result_s1_state_group_0_state_1_s197) == (program_result_s3_state_group_0_state_1_s203));;
  assert ((pipeline_result_s1_pkt_0_s195) == (program_result_s3_pkt_0_s201));;
}
void pipeline(int state_and_packet_pkt_0_s210, int state_and_packet_state_group_0_state_0_s211, int state_and_packet_state_group_0_state_1_s212, int& _out_pkt_0_s213, int& _out_state_group_0_state_0_s214, int& _out_state_group_0_state_1_s215, int* constant_vector__ANONYMOUS_s186/* len = 7 */) {
  int  destination_0_0_s5=0;
  snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateless_alu_0_0(state_and_packet_pkt_0_s210, 2, 0, 0, 0, 0, destination_0_0_s5, constant_vector__ANONYMOUS_s186);
  int  packet_operand_salu0_0_0_s7=0;
  snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_operand_mux_0_0_0(state_and_packet_pkt_0_s210, 0, packet_operand_salu0_0_0_s7);
  int  packet_operand_salu0_0_1_s9=0;
  snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_operand_mux_0_0_1(state_and_packet_pkt_0_s210, 0, packet_operand_salu0_0_1_s9);
  int  packet_operand_salu0_0_2_s11=0;
  snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_operand_mux_0_0_2(state_and_packet_pkt_0_s210, 0, packet_operand_salu0_0_2_s11);
  int  packet_operand_salu0_0_3_s13=0;
  snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_operand_mux_0_0_3(state_and_packet_pkt_0_s210, 0, packet_operand_salu0_0_3_s13);
  int  packet_operand_salu0_0_4_s15=0;
  snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_operand_mux_0_0_4(state_and_packet_pkt_0_s210, 0, packet_operand_salu0_0_4_s15);
  int  state_operand_salu_0_0_state_1_s217=state_and_packet_state_group_0_state_1_s212;
  int  state_operand_salu_0_0_state_0_s216=state_and_packet_state_group_0_state_0_s211;
  int  old_state_group_0_0_s17_state_0_s220=0;
  int  old_state_group_0_0_s17_state_1_s221=0;
  snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0(state_operand_salu_0_0_state_0_s216, state_operand_salu_0_0_state_1_s217, 1, packet_operand_salu0_0_0_s7, packet_operand_salu0_0_1_s9, packet_operand_salu0_0_2_s11, packet_operand_salu0_0_3_s13, packet_operand_salu0_0_4_s15, 1, 1, 2, 3, 2, 2, 2, 2, 3, 0, 2, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 5, 0, 1, 0, 0, 0, 0, 3, 1, 2, 4, 2, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 3, old_state_group_0_0_s17_state_0_s220, old_state_group_0_0_s17_state_1_s221, constant_vector__ANONYMOUS_s186);
  int  output_0_0_s19=0;
  snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_output_mux_phv_0_0(old_state_group_0_0_s17_state_0_s220, old_state_group_0_0_s17_state_1_s221, destination_0_0_s5, 2, output_0_0_s19);
  _out_pkt_0_s213 = output_0_0_s19;
  _out_state_group_0_state_0_s214 = state_operand_salu_0_0_state_0_s216;
  _out_state_group_0_state_1_s215 = state_operand_salu_0_0_state_1_s217;
  return;
}
void program(int state_and_packet_pkt_0_s204, int state_and_packet_state_group_0_state_0_s205, int state_and_packet_state_group_0_state_1_s206, int& _out_pkt_0_s207, int& _out_state_group_0_state_0_s208, int& _out_state_group_0_state_1_s209) {
  if ((state_and_packet_state_group_0_state_1_s206) == (0)) {
    state_and_packet_state_group_0_state_0_s205 = state_and_packet_state_group_0_state_0_s205 + 1;
    if ((state_and_packet_state_group_0_state_0_s205) == (1000)) {
      state_and_packet_state_group_0_state_1_s206 = 1;
    }
  }
  _out_pkt_0_s207 = state_and_packet_pkt_0_s204;
  _out_state_group_0_state_0_s208 = state_and_packet_state_group_0_state_0_s205;
  _out_state_group_0_state_1_s209 = state_and_packet_state_group_0_state_1_s206;
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateless_alu_0_0(int input0, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s166/* len = 7 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s166[immediate_operand_hole_local]);
  int  pkt_0_s157=0;
  snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateless_alu_0_0_mux1(input0, mux1_ctrl_hole_local, pkt_0_s157);
  int  pkt_1_s159=0;
  snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateless_alu_0_0_mux2(input0, mux2_ctrl_hole_local, pkt_1_s159);
  int  pkt_2_s161=0;
  snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateless_alu_0_0_mux3(input0, mux3_ctrl_hole_local, pkt_2_s161);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s157 + pkt_1_s159;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s157 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s157 - pkt_1_s159;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s157 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s157;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s157) != (pkt_1_s159);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s157) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s157) == (pkt_1_s159);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s157) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s157) >= (pkt_1_s159);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s157) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s157) < (pkt_1_s159);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s157) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s157) != (0)) {
                                  _out = pkt_1_s159;
                                  return;
                                } else {
                                  _out = pkt_2_s161;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s157) != (0)) {
                                    _out = pkt_1_s159;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s157) != (0)) || ((pkt_1_s159) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s157) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s157) != (0)) && ((pkt_1_s159) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s157) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s157) == (0);
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_operand_mux_0_0_0(int input0, int snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_operand_mux_0_0_0_ctrl_local, int& _out) {
  _out = input0;
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_operand_mux_0_0_1(int input0, int snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_operand_mux_0_0_1_ctrl_local, int& _out) {
  _out = input0;
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_operand_mux_0_0_2(int input0, int snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_operand_mux_0_0_2_ctrl_local, int& _out) {
  _out = input0;
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_operand_mux_0_0_3(int input0, int snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_operand_mux_0_0_3_ctrl_local, int& _out) {
  _out = input0;
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_operand_mux_0_0_4(int input0, int snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_operand_mux_0_0_4_ctrl_local, int& _out) {
  _out = input0;
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0(int& state_group_state_0_s222, int& state_group_state_1_s223, int output_mux_0, int pkt_1, int pkt_2, int pkt_3, int pkt_4, int pkt_5, int Mux3_0, int Mux3_1, int Mux3_10, int Mux3_11, int Mux3_12, int Mux3_13, int Mux3_14, int Mux3_15, int Mux3_16, int Mux3_17, int Mux3_18, int Mux3_19, int Mux3_2, int Mux3_20, int Mux3_21, int Mux3_22, int Mux3_23, int Mux3_24, int Mux3_25, int Mux3_26, int Mux3_27, int Mux3_3, int Mux3_4, int Mux3_5, int Mux3_6, int Mux3_7, int Mux3_8, int Mux3_9, int Opt_0, int Opt_1, int Opt_2, int Opt_3, int Opt_4, int Opt_5, int Opt_6, int Opt_7, int arith_op_0, int arith_op_1, int arith_op_2, int arith_op_3, int arith_op_4, int arith_op_5, int arith_op_6, int arith_op_7, int const_0, int const_1, int const_10, int const_11, int const_12, int const_13, int const_14, int const_15, int const_16, int const_17, int const_18, int const_19, int const_2, int const_3, int const_4, int const_5, int const_6, int const_7, int const_8, int const_9, int output_mux, int rel_op_0, int rel_op_1, int rel_op_2, int rel_op_3, int& _out_state_0_s224, int& _out_state_1_s225, int* constant_vector__ANONYMOUS_s183/* len = 7 */) {
  int  old_state_group_state_0_s226=state_group_state_0_s222;
  int  old_state_group_state_1_s227=state_group_state_1_s223;
  std::cout<<"STATEFUL ALU 0 0\n";
  int  state_0=0;
  state_0 = state_group_state_0_s222;
  int  state_1=0;
  state_1 = state_group_state_1_s223;
  int  _out_s21=0;
  snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_0(state_group_state_0_s222, state_group_state_1_s223, Mux3_0, _out_s21);
  int  _out_s23=0;
  snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_1(pkt_1, pkt_2, Mux3_1, _out_s23);
  int  _out_s25=0;
  snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_2(pkt_1, pkt_2, Mux3_2, _out_s25);
  int  _out_s27=0;
  snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_0(const_0, _out_s27, constant_vector__ANONYMOUS_s183);
  int  _out_s29=0;
  snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_rel_op_0((_out_s21 + _out_s23) - _out_s25, _out_s27, rel_op_0, _out_s29);
  if ((_out_s29) == (1)) {
    int  _out_s31=0;
    snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_3(state_group_state_0_s222, state_group_state_1_s223, Mux3_3, _out_s31);
    int  _out_s33=0;
    snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_4(pkt_1, pkt_2, Mux3_4, _out_s33);
    int  _out_s35=0;
    snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_5(pkt_1, pkt_2, Mux3_5, _out_s35);
    int  _out_s37=0;
    snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_1(const_1, _out_s37, constant_vector__ANONYMOUS_s183);
    int  _out_s39=0;
    snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_rel_op_1((_out_s31 + _out_s33) - _out_s35, _out_s37, rel_op_1, _out_s39);
    if ((_out_s39) == (1)) {
      int  state_0_s41=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Opt_0(state_group_state_0_s222, Opt_0, state_0_s41);
      int  state_0_s43=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_2(const_2, state_0_s43, constant_vector__ANONYMOUS_s183);
      int  state_0_s45=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_6(pkt_1, pkt_2, state_0_s43, Mux3_6, state_0_s45);
      int  state_0_s47=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_3(const_3, state_0_s47, constant_vector__ANONYMOUS_s183);
      int  state_0_s49=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_7(pkt_1, pkt_2, state_0_s47, Mux3_7, state_0_s49);
      int  state_0_s51=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_arith_op_0(state_0_s45, state_0_s49, arith_op_0, state_0_s51);
      state_0 = state_0_s41 + state_0_s51;
      int  state_1_s53=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Opt_1(state_group_state_1_s223, Opt_1, state_1_s53);
      int  state_1_s55=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_4(const_4, state_1_s55, constant_vector__ANONYMOUS_s183);
      int  state_1_s57=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_8(pkt_1, pkt_2, state_1_s55, Mux3_8, state_1_s57);
      int  state_1_s59=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_5(const_5, state_1_s59, constant_vector__ANONYMOUS_s183);
      int  state_1_s61=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_9(pkt_1, pkt_2, state_1_s59, Mux3_9, state_1_s61);
      int  state_1_s63=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_arith_op_1(state_1_s57, state_1_s61, arith_op_1, state_1_s63);
      state_1 = state_1_s53 + state_1_s63;
    } else {
      int  state_0_s65=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Opt_2(state_group_state_0_s222, Opt_2, state_0_s65);
      int  state_0_s67=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_6(const_6, state_0_s67, constant_vector__ANONYMOUS_s183);
      int  state_0_s69=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_10(pkt_1, pkt_2, state_0_s67, Mux3_10, state_0_s69);
      int  state_0_s71=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_7(const_7, state_0_s71, constant_vector__ANONYMOUS_s183);
      int  state_0_s73=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_11(pkt_1, pkt_2, state_0_s71, Mux3_11, state_0_s73);
      int  state_0_s75=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_arith_op_2(state_0_s69, state_0_s73, arith_op_2, state_0_s75);
      state_0 = state_0_s65 + state_0_s75;
      int  state_1_s77=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Opt_3(state_group_state_1_s223, Opt_3, state_1_s77);
      int  state_1_s79=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_8(const_8, state_1_s79, constant_vector__ANONYMOUS_s183);
      int  state_1_s81=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_12(pkt_1, pkt_2, state_1_s79, Mux3_12, state_1_s81);
      int  state_1_s83=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_9(const_9, state_1_s83, constant_vector__ANONYMOUS_s183);
      int  state_1_s85=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_13(pkt_1, pkt_2, state_1_s83, Mux3_13, state_1_s85);
      int  state_1_s87=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_arith_op_3(state_1_s81, state_1_s85, arith_op_3, state_1_s87);
      state_1 = state_1_s77 + state_1_s87;
    }
  } else {
    int  _out_s89=0;
    snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_14(state_group_state_0_s222, state_group_state_1_s223, Mux3_14, _out_s89);
    int  _out_s91=0;
    snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_15(pkt_1, pkt_2, Mux3_15, _out_s91);
    int  _out_s93=0;
    snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_16(pkt_1, pkt_2, Mux3_16, _out_s93);
    int  _out_s95=0;
    snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_10(const_10, _out_s95, constant_vector__ANONYMOUS_s183);
    int  _out_s97=0;
    snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_rel_op_2((_out_s89 + _out_s91) - _out_s93, _out_s95, rel_op_2, _out_s97);
    if ((_out_s97) == (1)) {
      int  _out_s99=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_17(state_group_state_0_s222, state_group_state_1_s223, Mux3_17, _out_s99);
      int  _out_s101=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_18(pkt_1, pkt_2, Mux3_18, _out_s101);
      int  _out_s103=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_19(pkt_1, pkt_2, Mux3_19, _out_s103);
      int  _out_s105=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_11(const_11, _out_s105, constant_vector__ANONYMOUS_s183);
      int  _out_s107=0;
      snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_rel_op_3((_out_s99 + _out_s101) - _out_s103, _out_s105, rel_op_3, _out_s107);
      if ((_out_s107) == (1)) {
        int  state_0_s109=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Opt_4(state_group_state_0_s222, Opt_4, state_0_s109);
        int  state_0_s111=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_12(const_12, state_0_s111, constant_vector__ANONYMOUS_s183);
        int  state_0_s113=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_20(pkt_1, pkt_2, state_0_s111, Mux3_20, state_0_s113);
        int  state_0_s115=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_13(const_13, state_0_s115, constant_vector__ANONYMOUS_s183);
        int  state_0_s117=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_21(pkt_1, pkt_2, state_0_s115, Mux3_21, state_0_s117);
        int  state_0_s119=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_arith_op_4(state_0_s113, state_0_s117, arith_op_4, state_0_s119);
        state_0 = state_0_s109 + state_0_s119;
        int  state_1_s121=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Opt_5(state_group_state_1_s223, Opt_5, state_1_s121);
        int  state_1_s123=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_14(const_14, state_1_s123, constant_vector__ANONYMOUS_s183);
        int  state_1_s125=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_22(pkt_1, pkt_2, state_1_s123, Mux3_22, state_1_s125);
        int  state_1_s127=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_15(const_15, state_1_s127, constant_vector__ANONYMOUS_s183);
        int  state_1_s129=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_23(pkt_1, pkt_2, state_1_s127, Mux3_23, state_1_s129);
        int  state_1_s131=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_arith_op_5(state_1_s125, state_1_s129, arith_op_5, state_1_s131);
        state_1 = state_1_s121 + state_1_s131;
      } else {
        int  state_0_s133=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Opt_6(state_group_state_0_s222, Opt_6, state_0_s133);
        int  state_0_s135=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_16(const_16, state_0_s135, constant_vector__ANONYMOUS_s183);
        int  state_0_s137=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_24(pkt_1, pkt_2, state_0_s135, Mux3_24, state_0_s137);
        int  state_0_s139=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_17(const_17, state_0_s139, constant_vector__ANONYMOUS_s183);
        int  state_0_s141=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_25(pkt_1, pkt_2, state_0_s139, Mux3_25, state_0_s141);
        int  state_0_s143=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_arith_op_6(state_0_s137, state_0_s141, arith_op_6, state_0_s143);
        state_0 = state_0_s133 + state_0_s143;
        int  state_1_s145=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Opt_7(state_group_state_1_s223, Opt_7, state_1_s145);
        int  state_1_s147=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_18(const_18, state_1_s147, constant_vector__ANONYMOUS_s183);
        int  state_1_s149=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_26(pkt_1, pkt_2, state_1_s147, Mux3_26, state_1_s149);
        int  state_1_s151=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_19(const_19, state_1_s151, constant_vector__ANONYMOUS_s183);
        int  state_1_s153=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_27(pkt_1, pkt_2, state_1_s151, Mux3_27, state_1_s153);
        int  state_1_s155=0;
        snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_arith_op_7(state_1_s149, state_1_s153, arith_op_7, state_1_s155);
        state_1 = state_1_s145 + state_1_s155;
      }
    }
  }
  std::cout<<"Old state00: " << old_state_group_state_0_s226<<"\n";

  std::cout<<"Old state01: " << old_state_group_state_1_s227<<"\n";
  std::cout<<"New state00: " <<state_0<<"\n";
  std::cout<<"New state01: "<<state_1<<"\n";
  state_group_state_0_s222 = state_0;
  state_group_state_1_s223 = state_1;
  if ((output_mux_0) == (1)) {
    _out_state_0_s224 = old_state_group_state_0_s226;
    _out_state_1_s225 = old_state_group_state_1_s227;
    return;
  } else {
    _out_state_0_s224 = state_0;
    _out_state_1_s225 = state_1;
    return;
  }
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_output_mux_phv_0_0(int input0, int input1, int input2, int snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_output_mux_phv_0_0_ctrl_local, int& _out) {
  if ((snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_output_mux_phv_0_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_output_mux_phv_0_0_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      _out = input2;
      return;
    }
  }
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateless_alu_0_0_mux1(int input0, int snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateless_alu_0_0_mux1_ctrl_local, int& _out) {
  _out = input0;
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateless_alu_0_0_mux2(int input0, int snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateless_alu_0_0_mux2_ctrl_local, int& _out) {
  _out = input0;
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateless_alu_0_0_mux3(int input0, int snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateless_alu_0_0_mux3_ctrl_local, int& _out) {
  _out = input0;
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_0(int op1, int op2, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_1(int op1, int op2, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_2(int op1, int op2, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_0(int const_var, int& _out, int* constant_vector__ANONYMOUS_s174/* len = 7 */) {
  _out = (constant_vector__ANONYMOUS_s174[const_var]);
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_rel_op_0(int operand1, int operand2, int opcode, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_3(int op1, int op2, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_4(int op1, int op2, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_5(int op1, int op2, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_1(int const_var, int& _out, int* constant_vector__ANONYMOUS_s163/* len = 7 */) {
  _out = (constant_vector__ANONYMOUS_s163[const_var]);
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_rel_op_1(int operand1, int operand2, int opcode, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Opt_0(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_2(int const_var, int& _out, int* constant_vector__ANONYMOUS_s168/* len = 7 */) {
  _out = (constant_vector__ANONYMOUS_s168[const_var]);
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_6(int op1, int op2, int op3, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_3(int const_var, int& _out, int* constant_vector__ANONYMOUS_s182/* len = 7 */) {
  _out = (constant_vector__ANONYMOUS_s182[const_var]);
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_7(int op1, int op2, int op3, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_arith_op_0(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Opt_1(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_4(int const_var, int& _out, int* constant_vector__ANONYMOUS_s175/* len = 7 */) {
  _out = (constant_vector__ANONYMOUS_s175[const_var]);
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_8(int op1, int op2, int op3, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_5(int const_var, int& _out, int* constant_vector__ANONYMOUS_s176/* len = 7 */) {
  _out = (constant_vector__ANONYMOUS_s176[const_var]);
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_9(int op1, int op2, int op3, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_arith_op_1(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Opt_2(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_6(int const_var, int& _out, int* constant_vector__ANONYMOUS_s172/* len = 7 */) {
  _out = (constant_vector__ANONYMOUS_s172[const_var]);
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_10(int op1, int op2, int op3, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_7(int const_var, int& _out, int* constant_vector__ANONYMOUS_s179/* len = 7 */) {
  _out = (constant_vector__ANONYMOUS_s179[const_var]);
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_11(int op1, int op2, int op3, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_arith_op_2(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Opt_3(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_8(int const_var, int& _out, int* constant_vector__ANONYMOUS_s165/* len = 7 */) {
  _out = (constant_vector__ANONYMOUS_s165[const_var]);
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_12(int op1, int op2, int op3, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_9(int const_var, int& _out, int* constant_vector__ANONYMOUS_s177/* len = 7 */) {
  _out = (constant_vector__ANONYMOUS_s177[const_var]);
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_13(int op1, int op2, int op3, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_arith_op_3(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_14(int op1, int op2, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_15(int op1, int op2, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_16(int op1, int op2, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_10(int const_var, int& _out, int* constant_vector__ANONYMOUS_s178/* len = 7 */) {
  _out = (constant_vector__ANONYMOUS_s178[const_var]);
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_rel_op_2(int operand1, int operand2, int opcode, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_17(int op1, int op2, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_18(int op1, int op2, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_19(int op1, int op2, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_11(int const_var, int& _out, int* constant_vector__ANONYMOUS_s162/* len = 7 */) {
  _out = (constant_vector__ANONYMOUS_s162[const_var]);
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_rel_op_3(int operand1, int operand2, int opcode, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Opt_4(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_12(int const_var, int& _out, int* constant_vector__ANONYMOUS_s164/* len = 7 */) {
  _out = (constant_vector__ANONYMOUS_s164[const_var]);
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_20(int op1, int op2, int op3, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_13(int const_var, int& _out, int* constant_vector__ANONYMOUS_s181/* len = 7 */) {
  _out = (constant_vector__ANONYMOUS_s181[const_var]);
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_21(int op1, int op2, int op3, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_arith_op_4(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Opt_5(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_14(int const_var, int& _out, int* constant_vector__ANONYMOUS_s180/* len = 7 */) {
  _out = (constant_vector__ANONYMOUS_s180[const_var]);
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_22(int op1, int op2, int op3, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_15(int const_var, int& _out, int* constant_vector__ANONYMOUS_s167/* len = 7 */) {
  _out = (constant_vector__ANONYMOUS_s167[const_var]);
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_23(int op1, int op2, int op3, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_arith_op_5(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Opt_6(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_16(int const_var, int& _out, int* constant_vector__ANONYMOUS_s171/* len = 7 */) {
  _out = (constant_vector__ANONYMOUS_s171[const_var]);
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_24(int op1, int op2, int op3, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_17(int const_var, int& _out, int* constant_vector__ANONYMOUS_s170/* len = 7 */) {
  _out = (constant_vector__ANONYMOUS_s170[const_var]);
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_25(int op1, int op2, int op3, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_arith_op_6(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Opt_7(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_18(int const_var, int& _out, int* constant_vector__ANONYMOUS_s173/* len = 7 */) {
  _out = (constant_vector__ANONYMOUS_s173[const_var]);
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_26(int op1, int op2, int op3, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_C_19(int const_var, int& _out, int* constant_vector__ANONYMOUS_s169/* len = 7 */) {
  _out = (constant_vector__ANONYMOUS_s169[const_var]);
  return;
}
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_Mux3_27(int op1, int op2, int op3, int choice, int& _out) {
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
void snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_stateful_alu_0_0_arith_op_7(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}

}
