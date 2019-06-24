// TODO: File representing a single pipeline stage

mod alu;
mod phv; 

use rand::{thread_rng, Rng};

pub struct Stage{
    atoms: Vec<alu::ALU>
}

impl Stage{

    /*Ask Anirudh how neccesary it is to have an initializer list constructor.*/

    pub fn new(vec_of_atoms : Vec<alu::ALU>) -> Self {
        return Stage{ atoms: vec_of_atoms };
    }

    pub fn tick(single_packet : phv::Phv, &self) -> phv::Phv {
        if (single_packet.bubble){
            return phv::Phv::new(); 
        }
        else{
            let ret = phv::Phv::new();
            thread_rng().shuffle(self.atoms);
            for atom in self.atoms {
                let current_phv = single_packet;
                atom(current_phv);
                ret += current_phv.clone();
            }
            return ret;
        }
    }
}

fn main(){
    println!("hello world");
}


