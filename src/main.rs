fn main() {
    //Println
    //Usamos {} para indicar que queremos imprimir el valor
    // de una argumento en la función(macro)
    //Estos valores se asignan a los argumentos en el orden en que se pasan
    println!(
        "The first letter of the English alphabet is: {} and the last letter is: {}",
        'A', 'Z'
    );

    //Declarar una variable con let
    let x = 5;
    
    //Declarar una variable String
    let s = "Hello, World!";

    /* Podemos observar que rust infiere el tipo de dato 
    ! no podemos reasignar el valor de una variable si no es mutable
     x=15;
    Podemos volver a declarar la variable sin problema. */
    let x = 15;

    /* Mutable variable
    Las variables mutables son variables que pueden cambiar su valor */
    let mut y = 5;
    y = 10;

    println!("El valor de x es: {}", x);
    println!("El valor de y es: {}", y);
    println!("El valor de s es: {}", s);
    print!("This charming man ")

}
