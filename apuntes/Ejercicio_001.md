# Rust

## Cargo
Cargo es el "Gestor de dependencias"

| Commando  | Argumento | Descripcion                                   |
|-----------|-----------|-----------------------------------------------|
| cargo     | new       | Crea un proyecto Rust con Cargo               |
| cargo     | run       | Compila y ejecuta el programa                 |
| cargo     | build     | Compila el programa                           |
| cargo     | check     | Comprueba que el programa compila (no ejecuta)|


## Metodo Main

```rust
//Este es el metodo main, solo puede haber uno por programa
    fn main(){

    }
```

## Print

>Rust cuenta con dos macros <i>"print!()"</i> y <i>"println!()"</i>

```rust
    fn main (){
        print!("This charming man!")
    }
```

## Variables

>Rust usa la palabra reservada let para las variables

```rust
fn main(){
    let x = 10;
}
```

>Rust por defecto tiene las variables inmutables (No pueden cambiar el valor de la variable), pero si puedes volver a declarar la variable (Aunque el compilador te dice que manda una alerta)
Esto con el fin de ser memory safety

```rust
fn main(){
    let x = 5;
    let x = "8";

    //!ESTO NO ES POSIBLE!
    /* let x=5;
    x=24; */
}
```

> Es importante aclarar que Rust, cuenta con una inferencia de tipos, por lo que podemos realizar la re-asigancion de variables como vimos en el ejemplo anterior.

### Variables Mutables

>Rust cuenta con la palabra reservada mut, con esta palabra podemos habilitar que las variables puedan cambiar su valor en tiempo de ejecución