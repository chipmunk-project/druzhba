/*Chipmunk will choose which value is returned from the mux, 
so the mux function should maybe take in an index saying which value Sketch chose

Notes:
    self basically is just "this" in rust

TODO: 
- Compiling Correctly, now observe output
*/
struct Mux{
    n_values: Vec<i32>,
    index_to_return: usize,
}

impl Mux{
    //TODO: Add assertion statement
    pub fn return_value(&self) -> i32 {
        return self.n_values[self.index_to_return];
    }
}

fn main(){
    let ten_val_arr : Vec<i32> = vec![0,1,2,3,4,5,6,7,8,9];
    let index: usize = 3;
    let m = Mux{n_values:ten_val_arr, index_to_return:index};
    println!("The value returned from this mux is {}", m.return_value().to_string());
}


