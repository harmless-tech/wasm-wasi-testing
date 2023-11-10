use wasi_help::{u32_to_i32, i32_to_u32};

wit_bindgen::generate!({
    world: "testing",
    exports: {
        world: GG,
    },
    path: "../../wit/tcomp.wit",
});

struct GG;
impl Guest for GG {
    fn h_world() -> String {
        println!("Hello from guest!");
        format!("Guest {}", hello())
    }
    
    fn data_process(mut data: Vec<u32>) -> Option<Vec<u32>> {
        if data.len() > 3 {
            for i in data.iter_mut() {
                let mut a: i32 = u32_to_i32(*i);
                a -= 2;
                *i = i32_to_u32(a);
            }
            return Some(data);
        }
        
        None
    }
}
