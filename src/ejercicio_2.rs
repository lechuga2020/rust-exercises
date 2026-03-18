//### Ejercicio 2: Lifetimes ⭐⭐
//**Objetivo:** Implementar una 
//función que retorna referencias con lifetimes correctos
// //**Tarea:**
// 1. Agrega los lifetimes necesarios
// 2. Escribe un `main()` que use esta función
// 3. Intenta crear un caso donde `resultado` viva más que uno de los videos (debe dar error)

//use std::string;

// Implementa esta función para que compile y funcione
pub struct Video<'a> {
    pub titulo: &'a String,
    pub duracion: u32,
}

// ❌ Falta lifetime
pub fn obtener_titulo<'a>(v1: &'a Video, v2: &'a Video) -> &'a str {
    if v1.duracion > v2.duracion {
        &v1.titulo
    } else {
        &v2.titulo
    }
}

pub fn ejercio_2(){
    let v1 = Video
    {
        titulo: &String::from("Video 1"), duracion: 300
    }; 

    let v2 = Video{
        titulo: &String::from("Video 2"), duracion: 6520
    }; 

    let res = obtener_titulo(&v1, &v2); 
    println!("El video mas largo es: {}", res); 
    let res  = obtener_titulo(&v1, &v2); 
    println!("El video mas largo es: {}", res);
    let result_fail; 
    {
        let v3 = Video
        { 
            titulo: &String::from("v3"),
            duracion: 350
        };
        result_fail = obtener_titulo(&v1, &v3);
        println!("El video mas largo es: {}", result_fail);  
    }
    //Lo sigiente da error porque result_fail vive más que v3, 
    //y v3 es necesario para obtener el resultado
    /*
        let res = obtener_titulo(&v1, &v3);
        println!("El video mas largo es: {}", res);  
     */
    //println!("El video mas largo es: {}", result_fail);  
    

    //res = obtener_titulo(v1, v2); 


}