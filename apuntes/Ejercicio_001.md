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

Rust cuenta con dos macros <i>"print!()"</i> y <i>"println!()"</i>

```rust
    fn main (){
        print!("This charming man!")
    }
```

## Variables

>[!IMPORTANT]
>Rust usa la palabra reservada let para las variables

```rust
fn main(){
    let x = 10;
}
```
>[!NOTE]
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

Es importante aclarar que Rust, cuenta con una inferencia de tipos, por lo que podemos realizar la re-asigancion de variables como vimos en el ejemplo anterior.

### Variables Mutables

>[!IMPORTANT]
Rust cuenta con la palabra reservada mut, con esta palabra podemos habilitar que las variables puedan cambiar su valor en tiempo de ejecución

```rust
fn main(){
    let mut x = 25;
    x = 32;
    println!("El valor de x es: {}", x);

    println!("The first letter of the English alphabet is: {} and the last letter is: {}",'A','Z');

}
```
De esta manera podemos manipular el valor de las variables de mejor manera

## Tipos de datos

>[!NOTE]
>Rust es un lenguaje con establecimiento de tipos en modo estático. El compilador debe conocer el tipo de datos exacto de todas las variables del código para que el programa se compile y ejecute. Normalmente, el compilador puede inferir el tipo de datos de una variable en función del valor enlazado. No siempre es necesario indicar de forma explícita el tipo en el código. Cuando son posibles muchos tipos, debe informar al compilador del tipo específico mediante anotaciones de tipo.


```rust
let number: u32 = 14;
println!("The number is {}.", number);
```
>[!CAUTION]
Si ponemos el valor de la variable entre comillas dobles, el compilador interpreta el valor como texto en lugar de como un número. El tipo de datos deducido del valor no coincide con el tipo de datos u32 especificado
.

### Tipos de datos integrados

Rust cuenta con los siguientes datos primitivos

- Números enteros
- Números de coma flotante (float)
- Booleanos
- Characters
  
---

##### Números: Valores enteros y floats

Los enteros en Rust se identifican por el tamaño en bits  y la propiedad signed (Si cuenta con signo o es sin signo)

>[!NOTE]
> Sin signo es toma los numeros naturales (solo +)
> Con signo, toma todos los numeros enteros (incluye los -)

| Tamaño|Firmado|Sin signo|
|-------|-------|---------|
|8 bits |i8     |u8       |
|16 bits|i16    |u16      |
|32 bits|i32    |u32      |
|64 bits|i64    |u64      |
|depende de la arquitectura|isize|usize|

>[!IMPORTANT]
Los tipos isize y usize dependen del tipo de equipo en el que se ejecuta el programa. El tipo de 64 bits se usa en una arquitectura de 64 bits y el tipo de 32 bits, en una arquitectura de 32 bits. Si no especifica el tipo para un entero, y el sistema no puede deducir el tipo, asigna el tipo i32 (un entero de 32 bits con signo) de forma predeterminada.

---
Rust tiene dos tipos de datos de coma flotante

|   |f32|f64|
|---|---|---|
|Tamaño|32 bits| 64 bits|


```rust
let number_64 = 4.0;      // compiler infers the value to use the default type f64
let number_32: f32 = 5.0; // type f32 specified via annotation
```

```rust
fn main(){
// Addition, Subtraction, and Multiplication
println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);

// Integer and Floating point division
println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);
}
```

#### Booleanos
Los booleanoes se utilizan en expresiones condicionales, Si una instruccion bool o un valor true realiza una accion; de lo contrario realiza u accion distinta

```rust
// Declare variable to store result of "greater than" test, Is 1 > 4? -- false
let is_bigger = 1 > 4;
println!("Is 1 > 4? {}", is_bigger);  
```

#### Texto: Caracteres y cadenas 

Rust admite valores de texto con dos tipos de cadena básicos y un tipo de carácter. Un carácter es un elemento único, mientras que una cadena es una serie de caracteres. 

>[!IMPORTANT]
>Todos los tipos de texto son representaciones UTF-8 válidas.

##### Characteres

El tipo char es el más primitivo de los tipos de texto. El valor se especifica poniendo el elemento entre comillas simples:

```ruby
let uppercase_s = 'S';
let lowercase_f = 'f';
let smiley_face = '😃';
```
>[!NOTE]
Algunos lenguajes tratan sus tipos char como enteros de 8 bits sin signo, que es el equivalente del tipo u8 de Rust. El tipo char de Rust contiene puntos de código Unicode, pero no usa la codificación UTF-8. char en Rust es un entero de 21 bits que se ha agregado para ampliar a 32 bits.

##### Cadenas

Existen dos tipos de Cadenas
String y &str [Vease documentacion](https://doc.rust-lang.org/book/ch08-02-strings.html)

```rust
// Specify the data type "char"
let character_1: char = 'S';
let character_2: char = 'f';
   
// Compiler interprets a single item in quotations as the "char" data type
let smiley_face = '😃';

// Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
let string_1 = "miley ";

// Specify the data type "str" with the reference syntax "&str"
let string_2: &str = "ace";

println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);
```