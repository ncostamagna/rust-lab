# Indice
- [Introduccion](#introduccion)
- [V8 y JavaScript Engines](#v8-y-javascript-engines)
- [NodeJS](#nodejs)
- [ES6](#es6)
- [Autentificacion](#autentificacion)
- [Unit Test](#unit-test)
- [Sincronia vs Asicnronia](#sincronia-vs-asicnronia)
- [Base de datos](#base-de-datos)
- [Arquitectura Apis](#arquitectura-apis)
- [Microservicios](#microservicios)


# Introduccion

### Rust
- Rapido y manejo de memoria eficiente
- Muy bueno para la seguridad
- No existe null
- No existen Excepciones
- Buen manejador de paquetes (como npm)

# Instalacion
Debemos entrar al sitio oficial de rust  y realizar las inastalaciones

```sh
rustc --version # Compilador
cargo --version # manejador de paquetes
rustup --version # instalador, podemos actualizar rustc o cargo
```

Instalamos la extension **rust**, **better TOML** y **crates** del VS Code

### Creamos proyecto
Utilizaremos cargo

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


# Memoria

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

# Leguaje

### Tipos de datos

- boolean
- characters
- integer
- floats

### Cargo expand
Descargamos de github, nos ayuda a ver nuestro codigo expandido, con lo que hace internamente cada funcion como **println!**

### Variables
Las variables son inmutables por defecto, esto significa que los valores que se le asignan a una variable no pueden ser modificados, para eso debemos declararlo como mutable
```rust
let mut input = String::new();
```
Definir tipo de dato
```rust
// defino que es un flotante de 32bytes
let weight: f32 = input.trim().parse().unwrap();
```

### Reglas

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

### Structs

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

### String
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