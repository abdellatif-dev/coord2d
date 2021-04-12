#[allow(dead_code)]
pub mod coords;
pub mod vectors;
#[cfg(test)]
mod test {
    use crate::coords;
    use crate::vectors;
    
    #[test]
    fn test_coords () {

        let a: coords::Coord = coords::new(1.0, 2.0);
        let x: f64 = 3.0;
        let y: f64 = 3.0;
        let b: coords::Coord = coords::new(x, y);
        
        // math
        let c: coords::Coord = a + b;
        let d: coords::Coord = a - c;
        let e: coords::Coord = a * d;
        let f: coords::Coord = a / a;
        
        // to vec
        let g: Vec<f64> = coords::Coord::to_vec(a);
        
        // to tuple
        let i: (f64, f64) = coords::Coord::to_tuple(c);

        // split
        let (r, o) = coords::Coord::split(a);
        
        println!("let a: coord::Coord = coord::new(1, 2); | {:?}", a);
        println!("\nlet (x , y) = (3, 3);\nlet b: coord::Coord = coord::new(x, y); | {:?}", b);
        println!("\nlet c: coord::Coord = a + b; | {:?}", c);
        println!("\nlet d: coord::Coord = a - c; | {:?}", d);
        println!("\nlet e: coord::Coord = a * d; | {:?}", e);
        println!("\nlet f: coord::Coord = a / a; | {:?}", f);
        println!("\nlet g: Vec<f64> = coord::Coord::to_vec(a); | {:?}", g);
        println!("\nlet i: (f64, f64) = coords::Coord::to_tuple(a); | {:?}", i);
        println!("\nlet (r, o) = coords::Coord::split(a); | r={}, o={}" ,r,o);
    }

    #[test]
    fn test_vector () {


        let a: coords::Coord = coords::new(1.0, 2.0);
        let x: f64 = 3.0;
        let y: f64 = 3.0;
        let b: coords::Coord = coords::new(x, y);
        
        let d: coords::Coord = a - b;
        let e: coords::Coord = a * d;

        let h = vectors::new(a, b);
        let l = vectors::new(a, e);

        let m = h + l;
        let s = h + l;
        let w = m + h;
        let t = h + s;

        let mag = vectors::Vector::get_magnitude(t);

        
        
        println!("\nlet h = points::new(a, b); | {:?}", h);
        println!("\nlet m = h + l; | {:?}", m);
        println!("\nlet s = h + l; | {:?}", s);
        println!("\nlet w = m + h; | {:?}", w);
        println!("\nlet t = h + s; | {:?}", t);
        println!("\nlet mag = vectors::Vector::get_magnitude(t); | {}", mag);

    }

}