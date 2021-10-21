pub mod recursive_backtracking;

pub enum Method {
    RecursiveBacktracking,
} 

pub fn new(grid: &mut Vec<Vec<u8>>, method: &Method) {
    match method {
        Method::RecursiveBacktracking => recursive_backtracking::generate(2, 2, grid),
    }; 
}