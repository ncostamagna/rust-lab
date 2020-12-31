// libreria IO
use std::io;

fn main() {
    println!("Enter your weight (kg): ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    dbg!(&input);

    let weight: f32 = input.trim().parse().unwrap();

    // este es el que se lo lleva, (weight)
    // si quiero asignarlo a otra funcion no voy a poder
    let mars_weight = calculate_weight_on_mars(weight);

    prueba_1();
    // {}kg por la variable, macro
    println!("Weight on Mars: {}kg", mars_weight);
}

// nombreVariable: tipo de dato -> lo que retorna
fn calculate_weight_on_mars(weight: f32) -> f32 {
    // esto es lo mismo que: return (weight / 9.81) * 3.711;
    (weight / 9.81) * 3.711
}

fn prueba_1(){

    // le paso el mutable borrow a s1, va a pinchar cuando lo asigne a s2
    //let s1 = &mut i;

/*     
    let s1 = &other_input;
    let s2 = &other_input;
    println!("{} {}", s1, s2); */

    let mut other_input = String::new();

    io::stdin().read_line(&mut other_input).unwrap();


    borrow_string(&other_input);
    own_strings(other_input);

}

// solo lo pide prestado
fn borrow_string(s: &String){
    println!("{}", s);
}

// trae el valor directamente aca y se adueña de el
fn own_strings(s: String){
    println!("{}", s);
}

/*
    Ejecutamos en modo debug
    cargo build
    cd ..
    cd target/debug
    gdb mars_calc
        b borrow_string
        b own_strings
        b main.rs:41 //breakpoint en linea 8
        r        // corremos el programa
        c        // para seguir
        info locals
        info args
*/

// locals line 41
/*
    other_input = alloc::string::String {
        vec: alloc::vec::Vec<u8> {
            buf: alloc::raw_vec::RawVec<u8, alloc::alloc::Global> {
                ptr: core::ptr::unique::Unique<u8> {
                    pointer: 0x5555555a8d70 "23\n\000", 
                    _marker: core::marker::PhantomData<u8>
                }, 
                cap: 8, 
                alloc: alloc::alloc::Global
            }, 
            len: 3
        }
    }
*/

// borrow_string
// s = 0x7fffffffd4f8
// ejecutando esto me muesta el puntero:
//  p *s

// mueve el valor a la funcion directamente
// own_strings
/*
    s = alloc::string::String {
        vec: alloc::vec::Vec<u8> {
            buf: alloc::raw_vec::RawVec<u8, alloc::alloc::Global> {
                ptr: core::ptr::unique::Unique<u8> {
                    pointer: 0x5555555a8d70 "23\n\000",
                    _marker: core::marker::PhantomData<u8>},
                    cap: 8, 
                    alloc: alloc::alloc::Global
                },
                len: 3
            }
        }
*/