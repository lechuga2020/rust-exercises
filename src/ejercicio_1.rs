//### Ejercicio 1: Ownership Básico ⭐
//**Objetivo:** Entender move, clone y borrowing


// Corrige este código para que compile
pub fn ejericio_1() {
    println!("1. Ejercicio 1: Ownership Básico ⭐");
    let mensaje = String::from("Este es el ejericio 1");
    imprimir_mensaje(&mensaje);
    imprimir_mensaje(&mensaje); // Ahora funciona correctamente
}

fn imprimir_mensaje(msg: &String) {
    println!("{}", msg);
}
