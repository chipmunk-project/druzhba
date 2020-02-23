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



  
    
// Stateful ALU blackbox
blackbox stateful_alu conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateful_alu_0_0_blackbox {
    
    reg                       : reg_0;
    condition_lo              : (((-(ipv4.pkt_4))) + (register_hi)+0) == (0);
    condition_hi              : (((ipv4.pkt_3)) - (register_lo)+1) > (0);
    update_lo_1_predicate     : not(condition_hi);
    update_lo_1_value         : (ipv4.pkt_3) - (0);
    update_lo_2_predicate     : (condition_hi) and (condition_lo);
    update_lo_2_value         : (ipv4.pkt_3) - (0);
    update_hi_1_predicate     : not((condition_hi) or (condition_lo));
    update_hi_1_value         : (ipv4.pkt_4);
    update_hi_2_predicate     : (not(condition_hi)) and (condition_lo);
    update_hi_2_value         : (ipv4.pkt_4);
    output_predicate          : 1;
    output_value              : 0;
    output_dst                : ipv4.pkt_6;
    
    initial_register_lo_value : 19; // Magic value TODO: needs to be changed.
    initial_register_hi_value : 0;

    
}

// Stateful ALU Action
action conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateful_alu_0_0_action () {
    conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateful_alu_0_0_blackbox.execute_stateful_alu(0);
    // TODO: Replace 0 with appropriate value for array-based registers. The
    // appropriate value can be determined by parsing the .c file using the
    // Domino compiler.
}

// Stateful ALU table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_0_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_1_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_2_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_3_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_4_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateful_alu_0_0_table

@pragma stage 0
table conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateful_alu_0_0_table {
    actions {
        conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateful_alu_0_0_action;
    }
    default_action: conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateful_alu_0_0_action;
}

  



  

// Stateless ALU action





action conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_0_action () {
    
    
    modify_field(ipv4.pkt_0, ipv4.pkt_0);
    
}

// Stateless ALU table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_0_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_1_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_2_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_3_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_4_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateful_alu_0_0_table

@pragma stage 0
table conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_0_table {
    actions {
        conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_0_action;
    }
    default_action:  conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_0_action;
}

  

// Stateless ALU action





action conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_1_action () {
    
    
    max(ipv4.pkt_1, ipv4.pkt_0, 0);
    
}

// Stateless ALU table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_0_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_1_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_2_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_3_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_4_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateful_alu_0_0_table

@pragma stage 0
table conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_1_table {
    actions {
        conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_1_action;
    }
    default_action:  conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_1_action;
}

  

// Stateless ALU action





action conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_2_action () {
    
    
    max(ipv4.pkt_2, ipv4.pkt_0, 0);
    
}

// Stateless ALU table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_0_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_1_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_2_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_3_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_4_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateful_alu_0_0_table

@pragma stage 0
table conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_2_table {
    actions {
        conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_2_action;
    }
    default_action:  conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_2_action;
}

  

// Stateless ALU action





action conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_3_action () {
    
    
    modify_field(ipv4.pkt_3, ipv4.pkt_3);
    
}

// Stateless ALU table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_0_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_1_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_2_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_3_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_4_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateful_alu_0_0_table

@pragma stage 0
table conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_3_table {
    actions {
        conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_3_action;
    }
    default_action:  conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_3_action;
}

  

// Stateless ALU action





action conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_4_action () {
    
    
    max(ipv4.pkt_4, ipv4.pkt_5, 0);
    
}

// Stateless ALU table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_0_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_1_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_2_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_3_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_4_table
@pragma ignore_table_dependency conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateful_alu_0_0_table

@pragma stage 0
table conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_4_table {
    actions {
        conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_4_action;
    }
    default_action:  conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_4_action;
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
    
      
        
          apply(conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_0_table);
        
      
        
          apply(conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_1_table);
        
      
        
          apply(conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_2_table);
        
      
        
          apply(conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_3_table);
        
      
        
          apply(conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateless_alu_0_4_table);
        
      
      
        
          apply(conga_equivalent_10_canonicalizer_equivalent_1_tofino_stateless_alu_for_tofino_1_5_stateful_alu_0_0_table);
        
      
    
    // MAC Forwarding by default
    apply(mac_forward);
}

control egress {

}
