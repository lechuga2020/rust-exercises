use crate::ejercicio_1;
use crate::ejercicio_2;
use crate::ejercicio_3;
use crate::ejercicio_4;
use crate::ejercicio_5;
use crate::ejercicio_6;

use crate::ejercicio_personal;

pub fn seleccionar_ejercicio(num: u8) {
    match num {
        1 => ejercicio_1::ejericio_1(),
        2 => ejercicio_2::ejercio_2(),
        3 => ejercicio_3::ejercicio_3(), 
        4 => ejercicio_4::ejercicio_4(),
        5 => ejercicio_5::ejercicio_5(), 
        6 => ejercicio_6::ejercicio_6(),

        100 => ejercicio_personal::votaciones(),
        _ => println!("Ejercicio no disponible"),
    } 
}

