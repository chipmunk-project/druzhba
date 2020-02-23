#include "tofino/intrinsic_metadata.p4"
#include "tofino/stateful_alu_blackbox.p4"

/* Declare Header */
header_type ethernet_t {
    fields {
        dstAddr : 48;
        srcAddr : 48;
        etherType : 16;
    }
}

header ethernet_t ethernet;

header_type ipv4_t {
    fields {
        // TODO: Have a hard limit on 5 fields for now. Ensure this in the tofino code generator.
        pkt_0 : 32;
        pkt_1 : 32;
        pkt_2 : 32;
        pkt_3 : 32;
        pkt_4 : 32;
    }
}

header ipv4_t ipv4;

/* Declare Parser */
parser start {
	return select(current(96,16)){
		0x0800: parse_ethernet;
	}
}

parser parse_ethernet {
    extract(ethernet);
    return select(latest.etherType) {
        /** Fill Whatever ***/
        0x0800     : parse_ipv4;
        default: ingress;
    }
}
parser parse_ipv4 {
    extract(ipv4);
    return ingress;
}

// TODO: Derive MAX_SIZE from Domino program.
#define MAX_SIZE 10

register reg_0 {
    width : 64;
    instance_count : MAX_SIZE;
}

register reg_1 {
    width : 64;
    instance_count : MAX_SIZE;
}

register reg_2 {
    width : 64;
    instance_count : MAX_SIZE;
}



  
    
  
    
  
    
// Stateful ALU blackbox
blackbox stateful_alu learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_0_2_blackbox {
    
    reg                       : reg_2;
    condition_lo              : (((ipv4.pkt_0)) + (register_hi)+0) < (0);
    condition_hi              : (((-(ipv4.pkt_0)))+0) > (0);
    update_lo_1_predicate     : not((condition_hi) or (condition_lo));
    update_lo_1_value         : 1;
    update_lo_2_predicate     : false;
    update_lo_2_value         : (ipv4.pkt_0);
    update_hi_1_predicate     : (condition_hi) and (not(condition_lo));
    update_hi_1_value         : 1;
    update_hi_2_predicate     : (condition_hi) and (not(condition_lo));
    update_hi_2_value         : (3);
    output_predicate          : 1;
    output_value              : ipv4.pkt_0;
    output_dst                : ipv4.pkt_1;
    
    initial_register_lo_value : 19; // Magic value TODO: needs to be changed.
    initial_register_hi_value : 0;

    
}

// Stateful ALU Action
action learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_0_2_action () {
    learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_0_2_blackbox.execute_stateful_alu(0);
    // TODO: Replace 0 with appropriate value for array-based registers. The
    // appropriate value can be determined by parsing the .c file using the
    // Domino compiler.
}

// Stateful ALU table
@pragma ignore_table_dependency learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateless_alu_0_0_table
@pragma ignore_table_dependency learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_0_2_table
@pragma ignore_table_dependency learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_0_table
@pragma ignore_table_dependency learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_1_table

@pragma stage 0
table learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_0_2_table {
    actions {
        learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_0_2_action;
    }
    default_action: learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_0_2_action;
}

  

  
    
// Stateful ALU blackbox
blackbox stateful_alu learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_0_blackbox {
    
    reg                       : reg_0;
    condition_lo              : (((ipv4.pkt_0)) + (register_lo)+1) < (0);
    condition_hi              : (((0))+0) < (0);
    update_lo_1_predicate     : (condition_hi) or (not(condition_lo));
    update_lo_1_value         : (3) - (2);
    update_lo_2_predicate     : (condition_lo);
    update_lo_2_value         : (ipv4.pkt_0) + (0);
    update_hi_1_predicate     : false;
    update_hi_1_value         : 1;
    update_hi_2_predicate     : false;
    update_hi_2_value         : (ipv4.pkt_1) + (0);
    output_predicate          : 1;
    output_value              : ipv4.pkt_1;
    output_dst                : ipv4.pkt_0;
    
    initial_register_lo_value : 19; // Magic value TODO: needs to be changed.
    initial_register_hi_value : 0;

    
}

// Stateful ALU Action
action learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_0_action () {
    learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_0_blackbox.execute_stateful_alu(0);
    // TODO: Replace 0 with appropriate value for array-based registers. The
    // appropriate value can be determined by parsing the .c file using the
    // Domino compiler.
}

// Stateful ALU table
@pragma ignore_table_dependency learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateless_alu_0_0_table
@pragma ignore_table_dependency learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_0_2_table
@pragma ignore_table_dependency learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_0_table
@pragma ignore_table_dependency learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_1_table

@pragma stage 1
table learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_0_table {
    actions {
        learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_0_action;
    }
    default_action: learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_0_action;
}

  
    
// Stateful ALU blackbox
blackbox stateful_alu learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_1_blackbox {
    
    reg                       : reg_1;
    condition_lo              : (((ipv4.pkt_0)) + (register_lo)+0) < (0);
    condition_hi              : (((0)) + (register_hi)+1) == (0);
    update_lo_1_predicate     : (condition_hi) ^ (condition_lo);
    update_lo_1_value         : (1) + (0);
    update_lo_2_predicate     : not((condition_hi) or (condition_lo));
    update_lo_2_value         : (1);
    update_hi_1_predicate     : (condition_hi) and (not(condition_lo));
    update_hi_1_value         : (register_hi);
    update_hi_2_predicate     : (condition_hi) ^ (condition_lo);
    update_hi_2_value         : (register_hi) - (ipv4.pkt_0);
    output_predicate          : 1;
    output_value              : register_lo;
    output_dst                : ipv4.pkt_1;
    
    initial_register_lo_value : 19; // Magic value TODO: needs to be changed.
    initial_register_hi_value : 0;

    
}

// Stateful ALU Action
action learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_1_action () {
    learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_1_blackbox.execute_stateful_alu(0);
    // TODO: Replace 0 with appropriate value for array-based registers. The
    // appropriate value can be determined by parsing the .c file using the
    // Domino compiler.
}

// Stateful ALU table
@pragma ignore_table_dependency learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateless_alu_0_0_table
@pragma ignore_table_dependency learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_0_2_table
@pragma ignore_table_dependency learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_0_table
@pragma ignore_table_dependency learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_1_table

@pragma stage 1
table learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_1_table {
    actions {
        learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_1_action;
    }
    default_action: learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_1_action;
}

  
    
  



  

// Stateless ALU action





action learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateless_alu_0_0_action () {
    
    
    max(ipv4.pkt_0, ipv4.pkt_0, 1);
    
}

// Stateless ALU table
@pragma ignore_table_dependency learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateless_alu_0_0_table
@pragma ignore_table_dependency learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_0_2_table
@pragma ignore_table_dependency learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_0_table
@pragma ignore_table_dependency learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_1_table

@pragma stage 0
table learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateless_alu_0_0_table {
    actions {
        learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateless_alu_0_0_action;
    }
    default_action:  learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateless_alu_0_0_action;
}

  

  

  

  

  


// Required: mac_forward table for forwarding to switch CPU.
action set_egr(egress_spec) {
    modify_field(ig_intr_md_for_tm.ucast_egress_port, egress_spec);
}
table mac_forward {
    reads {
        ethernet.dstAddr : exact;
    }
    actions {
        set_egr;
    }
    size:1;
}

control ingress {
    // Call all the required ALUs.
    
      
        
          apply(learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateless_alu_0_0_table);
        
      
        
      
      
        
      
        
      
        
          apply(learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_0_2_table);
        
      
    
      
        
      
        
      
      
        
          apply(learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_0_table);
        
      
        
          apply(learn_filter_modified_for_test_tofino_stateless_alu_for_tofino_2_2_stateful_alu_1_1_table);
        
      
        
      
    
    // MAC Forwarding by default
    apply(mac_forward);
}

control egress {

}
