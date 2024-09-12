# Indice
- [Introduccion](#introduccion)
- [Instalacion](#instalacion)
- [Memoria](#memoria)
- [Tipos de datos](#tipos-de-datos)
- [Cargo expand](#cargo-expand)
- [Variables](#variables)
- [Reglas](#reglas)
- [Structs](#structs)
- [String](#string)
- [Enums](#enums)
- [Modules](#modules)
- [Errors](#errors)
- [Loops](#loops)
- [Tuples](#tuples)


# Introduccion
https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number

### Rust
- Rapido y manejo de memoria eficiente
- Muy bueno para la seguridad
- No existe null
- No existen Excepciones
- Buen manejador de paquetes (como npm)

# Instalacion
You can install rust from the oficial site: https://www.rust-lang.org/learn/get-started
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```sh
rustc --version # Compilador
cargo --version # manejador de paquetes
rustup --version # instalador, podemos actualizar rustc o cargo
```

Instalamos la extension **rust**, **better TOML** y **crates** del VS Code. <br />

Standart libraries: https://doc.rust-lang.org/std/prelude/index.html

### Create new project
We use cargo

```sh
cargo new my-project
```

### Cargo.toml 
Tenemos las dependencias, metadatas y config de compilacion, podemos ir agregando crates.io -> https://crates.io/
- entramos al sitio
- buscamos una dependencia como rand
- copiamos el package = version
- lo pegamos en el archivo TOML
- y por ultimo ejecutamos el build


### Instalacion y compilacion

```sh
cargo build # instalamos dependencias
cargo run # compilamos
cargo check # quickly check

cargo update # update the dependencies

cargo install cargo-expand # instalamos con cargo directamente

cargo expand
# Si nos tira error de toolchain
rustup toolchain list # enlistamos para ver que version tenemos, y vemos que solo esta la estabe
rustup toolchain install nightly-x86_64-unknown-linux-gnu # instalamos tambien la version nightly

```

ejecutamos lo siguiente para ejecutar en modo debug
```bash
gdb mars_calc
```

La carpeta **target** es como el node_module de node, donde van a estar nuestras dependencias


# Common Concept

## Statements and Expressions

- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resultant value. Let’s look at some examples.

# Ownership
Ownership in Rust is a system of rules that govern memory management, ensuring that programs use memory efficiently without the need for garbage collection or manual memory management. The Rust compiler enforces these rules, and if they are violated, the program will not compile. This system helps create safe and efficient code without affecting runtime performance. Although ownership may be challenging to grasp initially, becoming familiar with it is crucial for mastering Rust’s unique features. The concept is introduced through examples, particularly focusing on string data structures.

## Variables

- Mutable Variable: The same memory location is reused and updated.
- Constant: Stored in a read-only section of memory.
- Shadowing: Each shadowed variable gets a new memory location, and previous variables are no longer accessible within the new scope.

```
Mutable Variable (mut): When you declare a variable as mutable and update its value, the same memory location is reused. This means that no additional memory is allocated for the new value; the existing memory is simply updated.

Shadowing: When you shadow a variable, a new memory location is allocated for each new value. The previous variable is no longer accessible within the new scope, but it still occupies memory until it goes out of scope and is cleaned up by Rust's ownership system.
```


## Stack & Heap
In Rust, understanding the stack and the heap is crucial due to its role as a systems programming language. The stack and heap are two types of memory used during runtime. The stack operates on a last-in, first-out basis, storing data in a fixed order and removing it in reverse. It is fast but requires data to have a known, fixed size. The heap, on the other hand, is more flexible but less organized. It allows for storing data with unknown or variable sizes by allocating space dynamically and returning a pointer to that space.
<br /><br />
Stack operations are faster because they occur in a predefined order, while heap operations are slower as they involve searching for available memory and managing pointers. Data on the heap must be accessed via pointers, making it slower to retrieve compared to data on the stack.
<br /><br />
When a function is called, its parameters and local variables are stored on the stack. Once the function finishes, this data is removed. Ownership in Rust manages heap data, ensuring that memory is efficiently allocated and deallocated, minimizing duplication, and preventing memory leaks. While Rust’s ownership model simplifies memory management, understanding the stack and heap provides insight into why Rust operates the way it does.

### Stack

```rust
fn main() {
    let a = 2;
    stack_only(a);
}

fn stack_only(b: i32) -> i32 {
    let c = 3;
    return c;
}
```
En este caso vamos a tener 2 stacks:
- stack main: a = 2
- stack stack_only: con c = 3 y b = 2
- una vez que salimos de la funcion el stack_only desaparece

<br />
Si llamamos infinitas funciones

```rust
fn infinite() {
    call infinite
}
```

va a llegar un punto que el sistema va a colapsar por falta de recursos, el manejo de los stacks depende de la aquitectura del equipo, si mandamos infinitas funciones va a llegar un punto que va a generar un error de **STACK OVERFLOW**


### Heap

la region de los procesos de memoria que no son automaticos, liberacion de memoria manual.<br/ > No tiene restricciones de tamaño.<br/ > 
Es posible acceder por alguna funcion en cualquier lugar del programa
```rust
fn main() {
    let a = 2;
    stack_only(a);
}

fn stack_only(b: i32) -> i32 {
    let c = 3;
    return c;
}

fn stack_and_heap() -> i32 {
    let d = 5;
    pointer e = allocate integer 7;
}
```
En este caso se guarda en e la direccion de memoria,
cuando termina la funcion se libera el stack pero el 7 sigue en el 
Heap almacenado en la direcccion de memoria

![imagen](images/1.jpg)

Rust no es como java, javascript o Go que el heap se limpia automaticamente, nosotros en Rust tenemos que liberar el espacio de memoria, para esto podemos utilizar SMART POINTER (como Box::new), para que se libere el espacio de memoria una vez que la funcion se libere del stack

## Reference and Borrowing
we can use reference to access a some value without lose the ownership

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### Mutable References

if we want to change the value, we must use mut

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

**Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.**

```rust
let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // ERROR: second mutable borrow occurs here

    println!("{}, {}", r1, r2);
```

As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:

```rust
let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

```

Rust enforces a similar rule for combining mutable and immutable references. This code results in an error:

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);

```

however, Multiple immutable references are allowed since reading the data doesn’t affect others.​⬤

```rust
// when we use &mut s, we don't use r1 and r2 anymore
let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");
```

### Dangling References

s is created inside dangle, when the code of dangle is finished, s will be deallocated. But we tried to return a reference to it. That means this reference would be pointing to an invalid String. That’s no good! Rust won’t let us do this.

```rust
fn main() {
    let reference_to_nothing = dangle();
}


fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

The solution here is to return the String directly:

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

# Tipos de datos

- boolean
- characters
- integer
- floats

## Integer

| Length | Signed | Unsigned |
|--------|--------|----------|
| 8-bit  | i8     | u8       |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| 128-bit | i128  | u128     |
| arch   | isize  | usize    |


You can write integer literals in any of the forms shown in Table 3-2. Note that number literals that can be multiple numeric types allow a type suffix, such as 57u8, to designate the type. Number literals can also use _ as a visual separator to make the number easier to read, such as 1_000, which will have the same value as if you had specified 1000.

| Number literals | Example       |
|-----------------|---------------|
| Decimal         | 98_222        |
| Hex             | 0xff          |
| Octal           | 0o77          |
| Binary          | 0b1111_0000   |
| Byte (u8 only)  | b'A'          |

## String

```rust
let s = "hello"
```

- Type: &str (string slice)
- Description: This is a string slice, which is a reference to a string literal. String literals are stored in the binary's read-only memory and are immutable, fast and efficient.
- Memory: The string data is stored in the program's binary and is not allocated on the heap.
- Usage: String slices are typically used for static strings that do not need to be modified.

```rust
let s = String::from("hello")
```

- Type: String
- Description: This is a String object, which is a growable, mutable, owned string type. It is part of Rust's standard library.
- Memory: The string data is allocated on the heap, and the String object owns this data. The memory is automatically managed and freed when the String goes out of scope.
- Usage: String is used when you need a mutable string that can be modified or when you need to perform operations that require ownership of the string dat

# Cargo expand
Descargamos de github, nos ayuda a ver nuestro codigo expandido, con lo que hace internamente cada funcion como **println!**

# Variables
Las variables son inmutables por defecto, esto significa que los valores que se le asignan a una variable no pueden ser modificados, para eso debemos declararlo como mutable
```rust
let mut input = String::new();
```
Definir tipo de dato
```rust
// defino que es un flotante de 32bytes
let weight: f32 = input.trim().parse().unwrap();
```

# Reglas

- Each value in Rust is owned by a variable
- When the owner goes out of scope, the value will be deallocated
- There can only be ONE owner at a time (Solo debe haber un dueño del valor para las variables)

```rust
fn main(){
    let mut input = String::new();
    some_fn(input)

    // me va a generar error porque ya utilice input arriba en some_fn
    io::stdin().read_line(&mut input);
}
// pasa a ser el owner aca y pincha cuando queramos volver a utilizar input en el main
fn some_fn(s: String){}
```

```rust
fn main(){
    let mut input = String::new();
    some_fn(&mut input)

    // me va a generar error porque ya utilice input arriba en some_fn
    io::stdin().read_line(&mut input);
}

// de esta manera le definimos que no sea el owner

// Si modificamos en la funcion se va a ver reflejado afuera
fn some_fn(s: &mut String){}

// Si modificamos en la funcion NO se va a ver reflejado afuera
fn some_fn(s: &String){}
```

# Structs

```rust

struct MyStruct {
    addr: String,

}

// bloque de implementacion
impl MyStruct{

    // generamos metodo new
    // devuelve la instancia del struct
    fn new(adde: String) -> Self{
        Self {
            //addr: addr
            addr
        }
    }

    // metodo del struct
    // self representa la instancia del struct
    // &self no lo toma como dueño (ownership) &mut self y lo toma como mutable
    fn run(self){
        println!("Listening in {}", self.addr);
    }
}
```

# String
```rust
let string = String::from("test");

// string_slice hace borrow de string
let string_slice = &string[10..14]; // desde el 10 hasta el 14 (str)
// no significa que va hasta el caracter 10, significa que va hasta el byte 10
// puede haber caracteres (letras japonesas, etc..) que tenga mas de un caracter
// si corto el caracter a la mitad pincha el programa

let string_borrow: &str = &string;
let string_literal = "1234" // nuevo espacio de memoria, es inmutable

dbg!(string); // por eso pincharia aca (lo tiene string_slice)
// para eso deberiamos utilizar
// dbg!(&string);
dbg!(string_slice);
dbg!(string_borrow);
dbg!(string_literal);

```
![imagen](images/2020-12-30-19-18-28.png)

# Enums
En memoria, los enums representan un siemple numero, empezando por 0 hasta n+1
```rust
enum Method {
    GET,
    PUT,
    //POST,
    POST = 5, // le puedo definir un valor, desde el get empezaria con 0 
              // y a partir de aca iria de 5 (6,7,8,...)
    DELETE,
    OPTIONS,
    HEAD,
}

// para traer el valor
let get = Method::GET;

```
![imagen](images/2020-12-31-01-21-21.png)
<br />

```rust
enum Method {
    GET(String),
    PUT(u64),
    POST,
    DELETE,
    OPTIONS,
    HEAD,
}

let get = Method::GET("asds".to_string());
let delete = Method::DELETE(100);
```

### OPTION ENUM
Representa **None** si no hay valores y **Some** si tiene algun valor
```rust
//option.rs
pub enum Option<T> {
    None,
    Some(T),
}
```
```rust
struct Request {
    query_string: Option<String>
}
```

# Growing Projects
- Packages: A Cargo feature that lets you build, test, and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a struct, function, or module

## Packages and Crates
A crate is the smallest amount of code that the Rust compiler considers at a time. Even if you run rustc rather than cargo and pass a single source code file  the compiler considers that file to be a crate. Crates can contain modules, and the modules may be defined in other files that get compiled with the crate, as we’ll see in the coming sections.
<br />
A crate can come in one of two forms: a binary crate or a library crate.

### Binary crates
Binary crates are programs you can compile to an executable that you can run, such as a command-line program or a server. Each must have a function called main that defines what happens when the executable runs. All the crates we’ve created so far have been binary crates.

### Library crates
Library crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects.
Most of the time when Rustaceans say “crate”, they mean library crate, and they use “crate” interchangeably with the general programming concept of a “library“.


### Crate root
The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate

## Packages
A package is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates. Cargo is actually a package that contains the binary crate for the command-line tool you’ve been using to build your code. The Cargo package also contains a library crate that the binary crate depends on. Other projects can depend on the Cargo library crate to use the same logic the Cargo command-line tool uses.

## Modules
we can declare new modules with **mod** for example **mod garden** <br />
We can create semimodules for example: mod vegetables; in src/garden.rs <br />

## Libraries
If we want to create a new library, we use

```sh
cargo new R071-restaurant --lib
```

# Modules
Por defecto los metodos y funciones son privadas, si necesito que sean publicas debo indicarselo
```rust
fn main(){
    let server = mystruct::MyStruct::new("sarasa");
}

// genero otro namespace con los modulos, en este caso mystruct
mod mystruct{

    // por defecto son privadas, debo indicarlas como publicas
    pub struct MyStruct {
        addr: String,

    }

    impl MyStruct{

        pub fn new(adde: String) -> Self{
            Self {
                addr
            }
        }

        pub fn run(self){
            println!("Listening in {}", self.addr);
        }
    }
}
```
Podemos utilizar **use** para no tener que llamar a todo el modulo siempre
```rust
use mystruct::MyStruct

fn main(){
    let server = MyStruct::new("sarasa");
}
```
### Sub Moduls
```rust
mod http {
    pub mod request {
        struct Request {
            path: String,
            query_string: Option<String>,
            // con super hacemos referencia al modulo padre
            method: super::method::Method,
        }
    }

    pub mod method {
        pub enum Method {
            GET,
            PUT,
            POST,
            DELETE,
            OPTIONS,
            HEAD,
        }
    }
}

```

### Modules files
El modulo sera el mismo nombre que el archivo
```rust
//server.rs

pub struct Server {
    addr: String,

}

impl Server{

    pub fn new(adde: String) -> Self{
        Self {
            addr
        }
    }

    pub fn run(self){
        println!("Listening in {}", self.addr);
    }
}

```

```rust
// main.rs

use server::Server;

// el mismo nombre del archivo, para hacer referencia a el
mod server;

fn main(){
    let server = Server::new("sarasa");
}
```


### Sub Modules files
Creamos el folder http y dentro los archivos request.rs y method.rs<br />
para eso debemos crear un archivo **mod.rs**
```rust
//http/method.rs

pub enum Method {
    GET,
    PUT,
    POST,
    DELETE,
    OPTIONS,
    HEAD,
}
```

```rust
//http/mod.rs
pub use request:Request;
pub use method:Method;

pub mod request;
pub mod method;
```

```rust
// main.rs

use http::Request;
use http::Method;

// el mismo nombre del folder, para hacer referencia a el
mod http;

fn main(){
    // TODO
}
```

# Errors
Se dividen en 2 categorias:
- recoverable: ej-> file not found
- unrecoverable: ej-> intentar ingresar a un index mas alla de donde termina el array
<br />

**RUST NO SOPORTA EXCEPCIONES**<br />
Podemos utilizar el enum **Result**
```rust
pub enum Result<T, E> {
    Ok(T), //Success value
    Err(E) //Error value
}
```

# Loops

```rust
// Loop infinito
loop {
    // TODO
}

// Podemos utilizar los breaks para salir
loop {
    break;
}

// Podemos hacer referecia al loop padre para salir de el desde el hijo
'outer: loop {
    loop {
        break 'outer;
    }
}

// Al igual que hacer un continue
'outer: loop {
    loop {
        continue 'outer;
    }
}
```

# Tuples
Puedo tener varios tipos de datos
```rust
let tup = (5, "String", listener);
```

```rust

fn run() -> (i32, &str, std::net::TcpListener) {
    (5, "a", listener)
}
```

# Match
Nos sirve para controlar las Tuples

```rust
// listenet.accept() nos devuelve una tupla de 2 valores,
// nos ayuda a manejar los enums
// en este caso solo tenemos Ok y Err
match listenet.accept() {
    // Ok tiene dos valores en este caso de la tupla (TcpStream y SocketAddr)
    Ok( (stream, _) ) => {
        // para ignorar utilizamos _ como en Golang
    },
    Err(e) => {
        println!("{}", e)
    },
    _ => {
        // ninguna de las variantes anteriores
    }
}



match "abdc" {
    "abdc" => {},
    "a" | "b" => {}, //or
    _ => {},
}
```

# Arrays
En Rust siempre debemos definir el tamaño del array y el contenido que va a tener al inicializar (y evitar que tenga basura como en C)<br /><br />
https://stackoverflow.com/questions/34684261/how-to-set-a-rust-array-length-dynamically
```rust
let a = [1,2,3,4]; //[i32, 4] [tipo de dato, size]

fn arr(a: [u8; 5]){} //debemos definirle el tipo y el tamaño

//Si queremos pasar el array y no sabemos el tamaño tenemos que pasarle la referencia
fn arr(a: &[u8]) {}

let mut buffer = [0;1024] // debemos definir al inicializar el tamaño del array, en este caso 1024 (1024 bytes, y le asignamos todos los valores en 0
```