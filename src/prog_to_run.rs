
use druzhba::pipeline_stage::PipelineStage;
use druzhba::pipeline::Pipeline;
use druzhba::phv::Phv;
use druzhba::phv_container::PhvContainer;
use druzhba::alu::ALU;

use std::collections::HashMap;

pub type StateScalar = PhvContainer <i32>;
pub type StateArray = PhvContainer <Vec <i32> >;

// Test ALUs to be used in pipeline that mutate
// Phv, StateScalar, and StateArray
fn atom0 (packet : &mut Phv, 
          state_scalar : &mut StateScalar, 
          state_array : &mut StateArray) 
{
  packet["new_hop"] = (packet["sport"] * packet["dport"]) % 10;
  if packet["arrival_time"] - state_scalar["last_time"] > 5{
    state_scalar["next_hop"]= packet["new_hop"];
  }
  state_scalar["last_time"]= packet["arrival_time"];
  packet["next_hop"] = state_scalar ["next_hop"];
  packet
}

fn atom1 (packet : &mut Phv, 
          state_scalar : &mut StateScalar, 
          state_array : &mut StateArray) {
  state_scalar ["new_hop"] = packet ["new_hop"];
  packet["new_hop"] = 0;
  state_scalar ["sport"] = packet ["sport"] * 5;
  packet ["sport"] = 23 - state_scalar ["dport"] * 2;
  if packet ["sport"] > 5{
    packet["sport"] -= 2;
  }
}

// atom2 and atom3 and run in the same pipeline_stage because
// they do not alter any existing fields
fn atom2 (packet : &mut Phv, 
          state_scalar : &mut StateScalar, 
          state_array : &mut StateArray) 
{

  packet["new_field"] = -1;
  state_scalar["dport"] = 2;
}
fn atom3 ( packet : &mut Phv, 
           state_scalar : &mut StateScalar, 
           state_array : &mut StateArray) 
{

  state_scalar ["last_time"] += 1;
  let mut v : Vec <i32> = Vec::new();
  v.push (3);
  v.push (10);
  state_array ["test_field"] = v;
}


// Initializes a new pipeline by creating ALUs based on the
// above atom functions, placing them each into a pipeline_stage,
// and adding each pipeline_stage into the pipeline.
pub fn init_pipeline (input_fields : &Vec <String>) -> Pipeline
{
    
  let mut stages : Vec<PipelineStage> = Vec::new();

  // Create new HashMap for the state_scalar
  let mut map : HashMap <String,i32> = HashMap::new();
  let vec_map : HashMap <String, Vec <i32> > = HashMap::new();

  // Initialize HashMap values with input fields to be used
  // in PhvContainer StateScalar
  input_fields.iter()
              .for_each ( |s| {
               map.insert(s.to_string(), 0); });


  let state_scalar : StateScalar = PhvContainer::with_map (map);

  let state_array : StateArray = PhvContainer::with_map (vec_map);
  
  let mut atoms0 : Vec<ALU> = Vec::new();

  // Initialize ALUs using above atom functions along with
  // state vectors and put them into vectors
  let alu0 : ALU = ALU::new(atom0, state_scalar.clone(), state_array.clone());
  atoms0.push (alu0);

  let mut atoms1 : Vec <ALU> = Vec::new();
  let alu1 : ALU = ALU::new (atom1, state_scalar.clone(), state_array.clone());
  atoms1.push (alu1);

  let mut atoms2 : Vec <ALU> = Vec::new();
  let alu2 : ALU = ALU::new (atom2, state_scalar.clone(), state_array.clone());
  atoms2.push (alu2);
  let alu3 : ALU = ALU::new (atom3, state_scalar.clone(), state_array.clone());

  // Create pipeline stages, initializing them with atom vectors
  let pipeline_stage0 : PipelineStage = PipelineStage::with_atoms(atoms0);
  let pipeline_stage1 : PipelineStage = PipelineStage::with_atoms(atoms1);
  let mut pipeline_stage2 : PipelineStage = PipelineStage::with_atoms(atoms2);
  // Append alu3 directly to pipeline_stage instead of adding to 
  // ALU vector
  pipeline_stage2.add_atom (alu3);

  // Add stages to the pipeline
  stages.push (pipeline_stage0);
  stages.push (pipeline_stage1);
  stages.push (pipeline_stage2);

  let pipeline : Pipeline = Pipeline::with_pipeline_stages(stages);

  pipeline 
}

