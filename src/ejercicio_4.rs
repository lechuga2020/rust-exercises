
/*
### Ejercicio 4: Manejo de Errores ⭐⭐
**Objetivo:** Crear un pipeline de procesamiento con manejo robusto de errores

```rust

*/
use std::fs::File;
use std::io::{self, Read};

// Implementa estas funciones con manejo de errores correcto
fn leer_archivo(ruta: &str) -> Result<String, io::Error> {
    /* TU CÓDIGO */
    let mut file = File::open(ruta)?; 
    let mut content = String::new(); 
    file.read_to_string(&mut content)?;
    Ok(content) 
}

fn parsear_numero(contenido: &str) -> Result<i32, std::num::ParseIntError> {
    /* TU CÓDIGO */
    let num = contenido.trim().parse::<i32>()?;
    Ok(num) 
}

fn duplicar_numero(n: i32) -> i32 {
    n * 2
}

// Pipeline completo usando ?
fn procesar_archivo(ruta: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let contenido = leer_archivo(ruta)?;
    let numero = parsear_numero(&contenido)?;
    Ok(duplicar_numero(numero))
}

pub fn ejercicio_4() {
    // Manejo de errores sin detener el programa
    match procesar_archivo("numero.txt") {
        Ok(resultado) => println!("Resultado: {}", resultado),
        Err(e) => {
            eprintln!("Error: {}", e);
            // Sistema continúa funcionando
        }
    }
}

