// ### Ejercicio 3: Contador Concurrente ⭐⭐
// **Objetivo:** Implementar un contador thread-safe

// **Tarea:** 
//Completa este código para que 5 hilos 
//incrementen un contador 100 veces cada uno.

// ```rust
use std::sync::{Arc, Mutex};
use std::thread;

pub fn ejercicio_3() {
    let contador = Arc::new(Mutex::new(0));  
    let mut handles = vec![];
    
    for _ in 0..5 {
        /* TU CÓDIGO: Clona el contador y crea un hilo */
        let count = contador.clone(); 
        let handle = thread::spawn(move || {
            for _ in 0..100{
                let mut num = count.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap(); 
    }
    /* TU CÓDIGO: Espera a que todos los hilos terminen */
    
    println!("Resultado: {}", contador.lock().unwrap());
    // Debe imprimir: 500
}

