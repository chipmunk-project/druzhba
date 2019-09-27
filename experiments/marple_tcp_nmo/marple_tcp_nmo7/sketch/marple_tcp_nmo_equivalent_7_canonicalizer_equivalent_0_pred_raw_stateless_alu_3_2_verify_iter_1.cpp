#include <cstdio>
#include <assert.h>
#include <iostream>
using namespace std;
#include "vops.h"
#include "marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_verify_iter_1.h"
namespace ANONYMOUS{

StateGroup* StateGroup::create(int  state_0_){
  void* temp= malloc( sizeof(StateGroup)  ); 
  StateGroup* rv = new (temp)StateGroup();
  rv->state_0 =  state_0_;
  return rv;
}
StateAndPacket* StateAndPacket::create(int  pkt_0_, int  state_group_0_state_0_, int  state_group_1_state_0_){
  void* temp= malloc( sizeof(StateAndPacket)  ); 
  StateAndPacket* rv = new (temp)StateAndPacket();
  rv->pkt_0 =  pkt_0_;
  rv->state_group_0_state_0 =  state_group_0_state_0_;
  rv->state_group_1_state_0 =  state_group_1_state_0_;
  return rv;
}
void main__Wrapper(int pkt_0, int state_group_0_state_0, int state_group_1_state_0) {
  bool _tt0[4] = {0, 0, 0, 0};
  int*  constant_vector__ANONYMOUS_s202= new int [4]; CopyArr<int >(constant_vector__ANONYMOUS_s202,_tt0, 4, 4);
  glblInit_constant_vector__ANONYMOUS_s212(constant_vector__ANONYMOUS_s202);
  _main(pkt_0, state_group_0_state_0, state_group_1_state_0, constant_vector__ANONYMOUS_s202);
  delete[] constant_vector__ANONYMOUS_s202;
}
void main__WrapperNospec(int pkt_0, int state_group_0_state_0, int state_group_1_state_0) {}
void glblInit_constant_vector__ANONYMOUS_s212(int* constant_vector__ANONYMOUS_s211/* len = 4 */) {
  int _tt1[4] = {0, 1, 2, 3};
  CopyArr<int >(constant_vector__ANONYMOUS_s211,_tt1, 4, 4);
}
void _main(int pkt_0, int state_group_0_state_0, int state_group_1_state_0, int* constant_vector__ANONYMOUS_s204/* len = 4 */) {
  pkt_0 = 0;
  state_group_0_state_0 = 1;
  state_group_1_state_0 = 1;
  std::cout<<"pkt_0 = " <<pkt_0 << " state00 = " << state_group_0_state_0 <<" state10 = " <<state_group_1_state_0 <<"\n";
  int  pipeline_result_s1_state_group_0_state_0_s220=0;
  int  pipeline_result_s1_state_group_1_state_0_s221=0;
  int  pipeline_result_s1_pkt_0_s219=0;
  pipeline(pkt_0, state_group_0_state_0, state_group_1_state_0, pipeline_result_s1_pkt_0_s219, pipeline_result_s1_state_group_0_state_0_s220, pipeline_result_s1_state_group_1_state_0_s221, constant_vector__ANONYMOUS_s204);
  std::cout<<"RESULT pkt_0 = " << pipeline_result_s1_pkt_0_s219<<" state00 = " << pipeline_result_s1_state_group_0_state_0_s220 << " state_10 = " << pipeline_result_s1_state_group_1_state_0_s221<<"\n\n";
  int  program_result_s3_state_group_0_state_0_s226=0;
  int  program_result_s3_state_group_1_state_0_s227=0;
  int  program_result_s3_pkt_0_s225=0;
  program(pkt_0, state_group_0_state_0, state_group_1_state_0, program_result_s3_pkt_0_s225, program_result_s3_state_group_0_state_0_s226, program_result_s3_state_group_1_state_0_s227);
  assert ((pipeline_result_s1_state_group_0_state_0_s220) == (program_result_s3_state_group_0_state_0_s226));;
  assert ((pipeline_result_s1_state_group_1_state_0_s221) == (program_result_s3_state_group_1_state_0_s227));;
  assert ((pipeline_result_s1_pkt_0_s219) == (program_result_s3_pkt_0_s225));;
}
void pipeline(int state_and_packet_pkt_0_s234, int state_and_packet_state_group_0_state_0_s235, int state_and_packet_state_group_1_state_0_s236, int& _out_pkt_0_s237, int& _out_state_group_0_state_0_s238, int& _out_state_group_1_state_0_s239, int* constant_vector__ANONYMOUS_s203/* len = 4 */) {
  int  destination_0_0_s5=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_0(state_and_packet_pkt_0_s234, 0, 15, 1, 0, 0, 0, destination_0_0_s5, constant_vector__ANONYMOUS_s203);
  int  destination_0_1_s7=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_1(state_and_packet_pkt_0_s234, 0, 15, 2, 1, 1, 0, destination_0_1_s7, constant_vector__ANONYMOUS_s203)
      ;
   std::cout<<"Stateful muxes 0\n";
  int  packet_operand_salu0_0_0_s9=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_0_0(state_and_packet_pkt_0_s234, 0, 0, packet_operand_salu0_0_0_s9);
  int  packet_operand_salu0_0_1_s11=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_0_1(state_and_packet_pkt_0_s234, 0, 0, packet_operand_salu0_0_1_s11);
  int  packet_operand_salu0_1_0_s13=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_1_0(state_and_packet_pkt_0_s234, 0, 0, packet_operand_salu0_1_0_s13);
  int  packet_operand_salu0_1_1_s15=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_1_1(state_and_packet_pkt_0_s234, 0, 1, packet_operand_salu0_1_1_s15);
  int  state_operand_salu_0_1_state_0_s241=state_and_packet_state_group_1_state_0_s236;
  int  old_state_group_0_0_s17_state_0_s247=0;
  int  state_operand_salu_0_0_state_0_s240=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0(state_operand_salu_0_0_state_0_s240, 0, packet_operand_salu0_0_0_s9, packet_operand_salu0_0_1_s11, 3, 1, 0, 0, 0, 3, 0, 3, old_state_group_0_0_s17_state_0_s247, constant_vector__ANONYMOUS_s203);
  int  old_state_group_0_1_s19_state_0_s249=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1(state_operand_salu_0_1_state_0_s241, 0, packet_operand_salu0_1_0_s13, packet_operand_salu0_1_1_s15, 0, 0, 0, 1, 1, 2, 0, 1, old_state_group_0_1_s19_state_0_s249, constant_vector__ANONYMOUS_s203);
  int  output_0_0_s21=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_0_0(old_state_group_0_0_s17_state_0_s247, old_state_group_0_1_s19_state_0_s249, destination_0_0_s5, 0, output_0_0_s21);
  int  output_0_1_s23=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_0_1(old_state_group_0_0_s17_state_0_s247, old_state_group_0_1_s19_state_0_s249, destination_0_1_s7, 1, output_0_1_s23);

  std::cout<<"Output mux results 0: " << output_0_0_s21 << " , " << output_0_1_s23<<"\n";

  int  destination_1_0_s25=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_0(output_0_0_s21, output_0_1_s23, 1, 0, 1, 0, 0, destination_1_0_s25, constant_vector__ANONYMOUS_s203);
  int  destination_1_1_s27=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_1(output_0_0_s21, output_0_1_s23, 12, 0, 0, 1, 0, destination_1_1_s27, constant_vector__ANONYMOUS_s203);
  std::cout<<"Stateful operand muxes 1\n";
  int  packet_operand_salu1_0_0_s29=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_0_0(output_0_0_s21, output_0_1_s23, 0, packet_operand_salu1_0_0_s29);
  int  packet_operand_salu1_0_1_s31=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_0_1(output_0_0_s21, output_0_1_s23, 0, packet_operand_salu1_0_1_s31);
  int  packet_operand_salu1_1_0_s33=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_1_0(output_0_0_s21, output_0_1_s23, 0, packet_operand_salu1_1_0_s33);
  int  packet_operand_salu1_1_1_s35=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_1_1(output_0_0_s21, output_0_1_s23, 0, packet_operand_salu1_1_1_s35);
  int  old_state_group_1_0_s37_state_0_s251=0;
  int  state_operand_salu_1_0_state_0_s242=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0(state_operand_salu_1_0_state_0_s242, 0, packet_operand_salu1_0_0_s29, packet_operand_salu1_0_1_s31, 1, 3, 0, 0, 1, 1, 0, 3, old_state_group_1_0_s37_state_0_s251, constant_vector__ANONYMOUS_s203);
  int  old_state_group_1_1_s39_state_0_s253=0;
  int  state_operand_salu_1_1_state_0_s243=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1(state_operand_salu_1_1_state_0_s243, 0, packet_operand_salu1_1_0_s33, packet_operand_salu1_1_1_s35, 0, 1, 0, 0, 2, 3, 0, 1, old_state_group_1_1_s39_state_0_s253, constant_vector__ANONYMOUS_s203);
  int  output_1_0_s41=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_1_0(old_state_group_1_0_s37_state_0_s251, old_state_group_1_1_s39_state_0_s253, destination_1_0_s25, 1, output_1_0_s41);
  int  output_1_1_s43=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_1_1(old_state_group_1_0_s37_state_0_s251, old_state_group_1_1_s39_state_0_s253, destination_1_1_s27, 3, output_1_1_s43);

  std::cout<<"Output mux results 1: " << output_1_0_s41 << " , " << output_1_1_s43<<"\n";
  int  destination_2_0_s45=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_0(output_1_0_s41, output_1_1_s43, 14, 1, 0, 0, 0, destination_2_0_s45, constant_vector__ANONYMOUS_s203);
  int  destination_2_1_s47=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_1(output_1_0_s41, output_1_1_s43, 0, 2, 0, 0, 0, destination_2_1_s47, constant_vector__ANONYMOUS_s203);
  int  packet_operand_salu2_0_0_s49=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_0_0(output_1_0_s41, output_1_1_s43, 1, packet_operand_salu2_0_0_s49);
  int  packet_operand_salu2_0_1_s51=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_0_1(output_1_0_s41, output_1_1_s43, 0, packet_operand_salu2_0_1_s51);
  int  packet_operand_salu2_1_0_s53=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_1_0(output_1_0_s41, output_1_1_s43, 1, packet_operand_salu2_1_0_s53);
  int  packet_operand_salu2_1_1_s55=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_1_1(output_1_0_s41, output_1_1_s43, 0, packet_operand_salu2_1_1_s55);
  int  state_operand_salu_2_0_state_0_s244=state_and_packet_state_group_0_state_0_s235;
  int  old_state_group_2_0_s57_state_0_s255=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0(state_operand_salu_2_0_state_0_s244, 1, packet_operand_salu2_0_0_s49, packet_operand_salu2_0_1_s51, 3, 0, 1, 0, 3, 1, 1, 1, old_state_group_2_0_s57_state_0_s255, constant_vector__ANONYMOUS_s203);
  int  old_state_group_2_1_s59_state_0_s257=0;
  int  state_operand_salu_2_1_state_0_s245=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1(state_operand_salu_2_1_state_0_s245, 0, packet_operand_salu2_1_0_s53, packet_operand_salu2_1_1_s55, 0, 1, 0, 0, 0, 2, 0, 3, old_state_group_2_1_s59_state_0_s257, constant_vector__ANONYMOUS_s203);
  int  output_2_0_s61=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_2_0(old_state_group_2_0_s57_state_0_s255, old_state_group_2_1_s59_state_0_s257, destination_2_0_s45, 2, output_2_0_s61);
  int  output_2_1_s63=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_2_1(old_state_group_2_0_s57_state_0_s255, old_state_group_2_1_s59_state_0_s257, destination_2_1_s47, 0, output_2_1_s63);
  std::cout<<"Output mux results 2: " << output_2_0_s61 << " , " << output_2_1_s63<<"\n";
  _out_pkt_0_s237 = output_2_0_s61;
  _out_state_group_0_state_0_s238 = state_operand_salu_2_0_state_0_s244;
  _out_state_group_1_state_0_s239 = state_operand_salu_0_1_state_0_s241;
  return;
}
void program(int state_and_packet_pkt_0_s228, int state_and_packet_state_group_0_state_0_s229, int state_and_packet_state_group_1_state_0_s230, int& _out_pkt_0_s231, int& _out_state_group_0_state_0_s232, int& _out_state_group_1_state_0_s233) {
  if (!(!(!(!((state_and_packet_pkt_0_s228) < (state_and_packet_state_group_1_state_0_s230)))))) {
    if (!(!((state_and_packet_pkt_0_s228) < (state_and_packet_state_group_1_state_0_s230)))) {
      if ((state_and_packet_pkt_0_s228) < (state_and_packet_state_group_1_state_0_s230)) {
        state_and_packet_state_group_0_state_0_s229 = state_and_packet_state_group_0_state_0_s229 + 1;
      }
    }
  } else {
    if (!(!(!((state_and_packet_pkt_0_s228) < (state_and_packet_state_group_1_state_0_s230))))) {
      if (!((state_and_packet_pkt_0_s228) < (state_and_packet_state_group_1_state_0_s230))) {
        state_and_packet_state_group_1_state_0_s230 = state_and_packet_pkt_0_s228;
      }
    }
  }
  _out_pkt_0_s231 = state_and_packet_pkt_0_s228;
  _out_state_group_0_state_0_s232 = state_and_packet_state_group_0_state_0_s229;
  _out_state_group_1_state_0_s233 = state_and_packet_state_group_1_state_0_s230;
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_0(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s189/* len = 4 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s189[immediate_operand_hole_local]);
  int  pkt_0_s179=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_0_mux1(input0, input1, mux1_ctrl_hole_local, pkt_0_s179);
  int  pkt_1_s181=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_0_mux2(input0, input1, mux2_ctrl_hole_local, pkt_1_s181);
  int  pkt_2_s183=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_0_mux3(input0, input1, mux3_ctrl_hole_local, pkt_2_s183);
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_1(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s184/* len = 4 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s184[immediate_operand_hole_local]);
  int  pkt_0_s173=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_1_mux1(input0, input1, mux1_ctrl_hole_local, pkt_0_s173);
  int  pkt_1_s175=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_1_mux2(input0, input1, mux2_ctrl_hole_local, pkt_1_s175);
  int  pkt_2_s177=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_1_mux3(input0, input1, mux3_ctrl_hole_local, pkt_2_s177);
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_0_0(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_0_0_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_0_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_0_1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_0_1_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_0_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_1_0(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_1_0_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_1_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_1_1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_1_1_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_1_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0(int& state_group_state_0_s273, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Opt_0, int Opt_1, int const_0, int const_1, int output_mux, int rel_op_0, int& _out_state_0_s274, int* constant_vector__ANONYMOUS_s207/* len = 4 */) {
  int  old_state_group_state_0_s275=state_group_state_0_s273;
  int  state_0=0;
  state_0 = state_group_state_0_s273;
  int  _out_s159=0;

  std::cout<<"STATEFUL ALU 0 0. pkt0 : " << pkt_0 <<" pk1: " <<pkt_1<<" state: " << state_0<<"\n";

  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0_Opt_0(state_group_state_0_s273, Opt_0, _out_s159);
  int  _out_s161=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0_C_0(const_0, _out_s161, constant_vector__ANONYMOUS_s207);
  int  _out_s163=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0_Mux3_0(pkt_0, pkt_1, _out_s161, Mux3_0, _out_s163);
  int  _out_s165=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0_rel_op_0(_out_s159, _out_s163, rel_op_0, _out_s165);
  if ((_out_s165) == (1)) {
    int  state_0_s167=0;
    marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0_Opt_1(state_group_state_0_s273, Opt_1, state_0_s167);
    int  state_0_s169=0;
    marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0_C_1(const_1, state_0_s169, constant_vector__ANONYMOUS_s207);
    int  state_0_s171=0;
    marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0_Mux3_1(pkt_0, pkt_1, state_0_s169, Mux3_1, state_0_s171);
    state_0 = state_0_s167 + state_0_s171;
  }
  state_group_state_0_s273 = state_0;
  if ((output_mux_0) == (1)) {
    _out_state_0_s274 = old_state_group_state_0_s275;
    std::cout<<"RETURNING STATE: " << _out_state_0_s274<<"\n";
    return;
  } else {

    _out_state_0_s274 = state_0;

    std::cout<<"RETURNING STATE: " << _out_state_0_s274<<"\n";
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1(int& state_group_state_0_s270, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Opt_0, int Opt_1, int const_0, int const_1, int output_mux, int rel_op_0, int& _out_state_0_s271, int* constant_vector__ANONYMOUS_s210/* len = 4 */) {
  int  old_state_group_state_0_s272=state_group_state_0_s270;
  int  state_0=0;
  state_0 = state_group_state_0_s270;
  int  _out_s145=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1_Opt_0(state_group_state_0_s270, Opt_0, _out_s145);
  int  _out_s147=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1_C_0(const_0, _out_s147, constant_vector__ANONYMOUS_s210);
  int  _out_s149=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1_Mux3_0(pkt_0, pkt_1, _out_s147, Mux3_0, _out_s149);
  int  _out_s151=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1_rel_op_0(_out_s145, _out_s149, rel_op_0, _out_s151);
  if ((_out_s151) == (1)) {
    int  state_0_s153=0;
    marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1_Opt_1(state_group_state_0_s270, Opt_1, state_0_s153);
    int  state_0_s155=0;
    marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1_C_1(const_1, state_0_s155, constant_vector__ANONYMOUS_s210);
    int  state_0_s157=0;
    marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1_Mux3_1(pkt_0, pkt_1, state_0_s155, Mux3_1, state_0_s157);
    state_0 = state_0_s153 + state_0_s157;
  }
  state_group_state_0_s270 = state_0;
  if ((output_mux_0) == (1)) {
    _out_state_0_s271 = old_state_group_state_0_s272;
    return;
  } else {
    _out_state_0_s271 = state_0;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_0_0(int input0, int input1, int input2, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_0_0_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_0_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_0_0_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      _out = input2;
      return;
    }
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_0_1(int input0, int input1, int input2, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_0_1_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_0_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_0_1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      _out = input2;
      return;
    }
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_0(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s197/* len = 4 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s197[immediate_operand_hole_local]);
  int  pkt_0_s139=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_0_mux1(input0, input1, mux1_ctrl_hole_local, pkt_0_s139);
  int  pkt_1_s141=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_0_mux2(input0, input1, mux2_ctrl_hole_local, pkt_1_s141);
  int  pkt_2_s143=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_0_mux3(input0, input1, mux3_ctrl_hole_local, pkt_2_s143);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s139 + pkt_1_s141;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s139 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s139 - pkt_1_s141;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s139 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s139;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s139) != (pkt_1_s141);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s139) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s139) == (pkt_1_s141);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s139) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s139) >= (pkt_1_s141);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s139) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s139) < (pkt_1_s141);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s139) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s139) != (0)) {
                                  _out = pkt_1_s141;
                                  return;
                                } else {
                                  _out = pkt_2_s143;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s139) != (0)) {
                                    _out = pkt_1_s141;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s139) != (0)) || ((pkt_1_s141) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s139) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s139) != (0)) && ((pkt_1_s141) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s139) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s139) == (0);
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_1(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s193/* len = 4 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s193[immediate_operand_hole_local]);
  int  pkt_0_s133=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_1_mux1(input0, input1, mux1_ctrl_hole_local, pkt_0_s133);
  int  pkt_1_s135=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_1_mux2(input0, input1, mux2_ctrl_hole_local, pkt_1_s135);
  int  pkt_2_s137=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_1_mux3(input0, input1, mux3_ctrl_hole_local, pkt_2_s137);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s133 + pkt_1_s135;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s133 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s133 - pkt_1_s135;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s133 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s133;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s133) != (pkt_1_s135);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s133) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s133) == (pkt_1_s135);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s133) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s133) >= (pkt_1_s135);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s133) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s133) < (pkt_1_s135);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s133) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s133) != (0)) {
                                  _out = pkt_1_s135;
                                  return;
                                } else {
                                  _out = pkt_2_s137;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s133) != (0)) {
                                    _out = pkt_1_s135;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s133) != (0)) || ((pkt_1_s135) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s133) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s133) != (0)) && ((pkt_1_s135) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s133) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s133) == (0);
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_0_0(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_0_0_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_0_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_0_1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_0_1_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_0_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_1_0(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_1_0_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_1_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_1_1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_1_1_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_1_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0(int& state_group_state_0_s267, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Opt_0, int Opt_1, int const_0, int const_1, int output_mux, int rel_op_0, int& _out_state_0_s268, int* constant_vector__ANONYMOUS_s208/* len = 4 */) {
  int  old_state_group_state_0_s269=state_group_state_0_s267;
  int  state_0=0;
  state_0 = state_group_state_0_s267;
  int  _out_s119=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0_Opt_0(state_group_state_0_s267, Opt_0, _out_s119);
  int  _out_s121=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0_C_0(const_0, _out_s121, constant_vector__ANONYMOUS_s208);
  int  _out_s123=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0_Mux3_0(pkt_0, pkt_1, _out_s121, Mux3_0, _out_s123);
  int  _out_s125=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0_rel_op_0(_out_s119, _out_s123, rel_op_0, _out_s125);
  if ((_out_s125) == (1)) {
    int  state_0_s127=0;
    marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0_Opt_1(state_group_state_0_s267, Opt_1, state_0_s127);
    int  state_0_s129=0;
    marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0_C_1(const_1, state_0_s129, constant_vector__ANONYMOUS_s208);
    int  state_0_s131=0;
    marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0_Mux3_1(pkt_0, pkt_1, state_0_s129, Mux3_1, state_0_s131);
    state_0 = state_0_s127 + state_0_s131;
  }
  state_group_state_0_s267 = state_0;
  if ((output_mux_0) == (1)) {
    _out_state_0_s268 = old_state_group_state_0_s269;
    return;
  } else {
    _out_state_0_s268 = state_0;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1(int& state_group_state_0_s264, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Opt_0, int Opt_1, int const_0, int const_1, int output_mux, int rel_op_0, int& _out_state_0_s265, int* constant_vector__ANONYMOUS_s206/* len = 4 */) {
  int  old_state_group_state_0_s266=state_group_state_0_s264;
  int  state_0=0;
  state_0 = state_group_state_0_s264;
  int  _out_s105=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1_Opt_0(state_group_state_0_s264, Opt_0, _out_s105);
  int  _out_s107=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1_C_0(const_0, _out_s107, constant_vector__ANONYMOUS_s206);
  int  _out_s109=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1_Mux3_0(pkt_0, pkt_1, _out_s107, Mux3_0, _out_s109);
  int  _out_s111=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1_rel_op_0(_out_s105, _out_s109, rel_op_0, _out_s111);
  if ((_out_s111) == (1)) {
    int  state_0_s113=0;
    marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1_Opt_1(state_group_state_0_s264, Opt_1, state_0_s113);
    int  state_0_s115=0;
    marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1_C_1(const_1, state_0_s115, constant_vector__ANONYMOUS_s206);
    int  state_0_s117=0;
    marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1_Mux3_1(pkt_0, pkt_1, state_0_s115, Mux3_1, state_0_s117);
    state_0 = state_0_s113 + state_0_s117;
  }
  state_group_state_0_s264 = state_0;
  if ((output_mux_0) == (1)) {
    _out_state_0_s265 = old_state_group_state_0_s266;
    return;
  } else {
    _out_state_0_s265 = state_0;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_1_0(int input0, int input1, int input2, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_1_0_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_1_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_1_0_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      _out = input2;
      return;
    }
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_1_1(int input0, int input1, int input2, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_1_1_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_1_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_1_1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      _out = input2;
      return;
    }
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_0(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s200/* len = 4 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s200[immediate_operand_hole_local]);
  int  pkt_0_s99=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_0_mux1(input0, input1, mux1_ctrl_hole_local, pkt_0_s99);
  int  pkt_1_s101=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_0_mux2(input0, input1, mux2_ctrl_hole_local, pkt_1_s101);
  int  pkt_2_s103=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_0_mux3(input0, input1, mux3_ctrl_hole_local, pkt_2_s103);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s99 + pkt_1_s101;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s99 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s99 - pkt_1_s101;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s99 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s99;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s99) != (pkt_1_s101);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s99) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s99) == (pkt_1_s101);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s99) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s99) >= (pkt_1_s101);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s99) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s99) < (pkt_1_s101);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s99) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s99) != (0)) {
                                  _out = pkt_1_s101;
                                  return;
                                } else {
                                  _out = pkt_2_s103;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s99) != (0)) {
                                    _out = pkt_1_s101;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s99) != (0)) || ((pkt_1_s101) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s99) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s99) != (0)) && ((pkt_1_s101) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s99) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s99) == (0);
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_1(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s186/* len = 4 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s186[immediate_operand_hole_local]);
  int  pkt_0_s93=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_1_mux1(input0, input1, mux1_ctrl_hole_local, pkt_0_s93);
  int  pkt_1_s95=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_1_mux2(input0, input1, mux2_ctrl_hole_local, pkt_1_s95);
  int  pkt_2_s97=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_1_mux3(input0, input1, mux3_ctrl_hole_local, pkt_2_s97);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s93 + pkt_1_s95;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s93 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s93 - pkt_1_s95;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s93 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s93;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s93) != (pkt_1_s95);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s93) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s93) == (pkt_1_s95);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s93) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s93) >= (pkt_1_s95);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s93) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s93) < (pkt_1_s95);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s93) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s93) != (0)) {
                                  _out = pkt_1_s95;
                                  return;
                                } else {
                                  _out = pkt_2_s97;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s93) != (0)) {
                                    _out = pkt_1_s95;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s93) != (0)) || ((pkt_1_s95) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s93) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s93) != (0)) && ((pkt_1_s95) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s93) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s93) == (0);
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_0_0(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_0_0_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_0_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_0_1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_0_1_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_0_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_1_0(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_1_0_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_1_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_1_1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_1_1_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_1_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0(int& state_group_state_0_s261, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Opt_0, int Opt_1, int const_0, int const_1, int output_mux, int rel_op_0, int& _out_state_0_s262, int* constant_vector__ANONYMOUS_s205/* len = 4 */) {
  int  old_state_group_state_0_s263=state_group_state_0_s261;
  int  state_0=0;
  state_0 = state_group_state_0_s261;
  int  _out_s79=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0_Opt_0(state_group_state_0_s261, Opt_0, _out_s79);
  int  _out_s81=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0_C_0(const_0, _out_s81, constant_vector__ANONYMOUS_s205);
  int  _out_s83=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0_Mux3_0(pkt_0, pkt_1, _out_s81, Mux3_0, _out_s83);
  int  _out_s85=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0_rel_op_0(_out_s79, _out_s83, rel_op_0, _out_s85);
  if ((_out_s85) == (1)) {
    int  state_0_s87=0;
    marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0_Opt_1(state_group_state_0_s261, Opt_1, state_0_s87);
    int  state_0_s89=0;
    marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0_C_1(const_1, state_0_s89, constant_vector__ANONYMOUS_s205);
    int  state_0_s91=0;
    marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0_Mux3_1(pkt_0, pkt_1, state_0_s89, Mux3_1, state_0_s91);
    state_0 = state_0_s87 + state_0_s91;
  }
  state_group_state_0_s261 = state_0;
  if ((output_mux_0) == (1)) {
    _out_state_0_s262 = old_state_group_state_0_s263;
    return;
  } else {
    _out_state_0_s262 = state_0;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1(int& state_group_state_0_s258, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Opt_0, int Opt_1, int const_0, int const_1, int output_mux, int rel_op_0, int& _out_state_0_s259, int* constant_vector__ANONYMOUS_s209/* len = 4 */) {
  int  old_state_group_state_0_s260=state_group_state_0_s258;
  int  state_0=0;
  state_0 = state_group_state_0_s258;
  int  _out_s65=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1_Opt_0(state_group_state_0_s258, Opt_0, _out_s65);
  int  _out_s67=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1_C_0(const_0, _out_s67, constant_vector__ANONYMOUS_s209);
  int  _out_s69=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1_Mux3_0(pkt_0, pkt_1, _out_s67, Mux3_0, _out_s69);
  int  _out_s71=0;
  marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1_rel_op_0(_out_s65, _out_s69, rel_op_0, _out_s71);
  if ((_out_s71) == (1)) {
    int  state_0_s73=0;
    marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1_Opt_1(state_group_state_0_s258, Opt_1, state_0_s73);
    int  state_0_s75=0;
    marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1_C_1(const_1, state_0_s75, constant_vector__ANONYMOUS_s209);
    int  state_0_s77=0;
    marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1_Mux3_1(pkt_0, pkt_1, state_0_s75, Mux3_1, state_0_s77);
    state_0 = state_0_s73 + state_0_s77;
  }
  state_group_state_0_s258 = state_0;
  if ((output_mux_0) == (1)) {
    _out_state_0_s259 = old_state_group_state_0_s260;
    return;
  } else {
    _out_state_0_s259 = state_0;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_2_0(int input0, int input1, int input2, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_2_0_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_2_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_2_0_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      _out = input2;
      return;
    }
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_2_1(int input0, int input1, int input2, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_2_1_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_2_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_2_1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      _out = input2;
      return;
    }
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_0_mux1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_0_mux1_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_0_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_0_mux2(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_0_mux2_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_0_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_0_mux3(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_0_mux3_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_0_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_1_mux1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_1_mux1_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_1_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_1_mux2(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_1_mux2_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_1_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_1_mux3(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_1_mux3_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_1_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0_Opt_0(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0_C_0(int const_var, int& _out, int* constant_vector__ANONYMOUS_s194/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s194[const_var]);
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0_Mux3_0(int op1, int op2, int op3, int choice, int& _out) {
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0_rel_op_0(int operand1, int operand2, int opcode, int& _out) {
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0_Opt_1(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0_C_1(int const_var, int& _out, int* constant_vector__ANONYMOUS_s199/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s199[const_var]);
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0_Mux3_1(int op1, int op2, int op3, int choice, int& _out) {
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1_Opt_0(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1_C_0(int const_var, int& _out, int* constant_vector__ANONYMOUS_s190/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s190[const_var]);
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1_Mux3_0(int op1, int op2, int op3, int choice, int& _out) {
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1_rel_op_0(int operand1, int operand2, int opcode, int& _out) {
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1_Opt_1(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1_C_1(int const_var, int& _out, int* constant_vector__ANONYMOUS_s198/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s198[const_var]);
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1_Mux3_1(int op1, int op2, int op3, int choice, int& _out) {
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_0_mux1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_0_mux1_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_0_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_0_mux2(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_0_mux2_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_0_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_0_mux3(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_0_mux3_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_0_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_1_mux1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_1_mux1_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_1_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_1_mux2(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_1_mux2_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_1_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_1_mux3(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_1_mux3_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_1_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0_Opt_0(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0_C_0(int const_var, int& _out, int* constant_vector__ANONYMOUS_s188/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s188[const_var]);
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0_Mux3_0(int op1, int op2, int op3, int choice, int& _out) {
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0_rel_op_0(int operand1, int operand2, int opcode, int& _out) {
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0_Opt_1(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0_C_1(int const_var, int& _out, int* constant_vector__ANONYMOUS_s192/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s192[const_var]);
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0_Mux3_1(int op1, int op2, int op3, int choice, int& _out) {
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1_Opt_0(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1_C_0(int const_var, int& _out, int* constant_vector__ANONYMOUS_s201/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s201[const_var]);
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1_Mux3_0(int op1, int op2, int op3, int choice, int& _out) {
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1_rel_op_0(int operand1, int operand2, int opcode, int& _out) {
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1_Opt_1(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1_C_1(int const_var, int& _out, int* constant_vector__ANONYMOUS_s185/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s185[const_var]);
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1_Mux3_1(int op1, int op2, int op3, int choice, int& _out) {
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_0_mux1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_0_mux1_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_0_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_0_mux2(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_0_mux2_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_0_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_0_mux3(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_0_mux3_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_0_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_1_mux1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_1_mux1_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_1_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_1_mux2(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_1_mux2_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_1_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_1_mux3(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_1_mux3_ctrl_local, int& _out) {
  if ((marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_1_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0_Opt_0(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0_C_0(int const_var, int& _out, int* constant_vector__ANONYMOUS_s187/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s187[const_var]);
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0_Mux3_0(int op1, int op2, int op3, int choice, int& _out) {
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0_rel_op_0(int operand1, int operand2, int opcode, int& _out) {
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0_Opt_1(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0_C_1(int const_var, int& _out, int* constant_vector__ANONYMOUS_s191/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s191[const_var]);
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0_Mux3_1(int op1, int op2, int op3, int choice, int& _out) {
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1_Opt_0(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1_C_0(int const_var, int& _out, int* constant_vector__ANONYMOUS_s195/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s195[const_var]);
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1_Mux3_0(int op1, int op2, int op3, int choice, int& _out) {
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1_rel_op_0(int operand1, int operand2, int opcode, int& _out) {
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
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1_Opt_1(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1_C_1(int const_var, int& _out, int* constant_vector__ANONYMOUS_s196/* len = 4 */) {
  _out = (constant_vector__ANONYMOUS_s196[const_var]);
  return;
}
void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1_Mux3_1(int op1, int op2, int op3, int choice, int& _out) {
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

}
