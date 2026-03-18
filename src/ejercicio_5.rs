
/*
### Ejercicio 5: Buffer de Video Productor-Consumidor ⭐⭐⭐
**Objetivo:** Implementar un buffer thread-safe para streaming de video

**Requerimientos:**
1. Buffer con capacidad máxima
2. Hilo productor que genera frames
3. Hilo consumidor que procesa frames
4. Manejo de buffer lleno/vacío
5. Señal de terminación

```rust
*/
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use rand::Rng; 
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
struct Frame {
    id: u32,
    data: Vec<u8>,
}

struct VideoBuffer {
    frames: Arc<Mutex<VecDeque<Frame>>>,
    max_size: usize,
}

impl VideoBuffer {
    fn new(max_size: usize) -> Self {
        Self{
            frames: Arc::new(Mutex::new(VecDeque::new())),
            max_size,
        }
    }
    
    fn producir_frame(&self, frame: Frame) -> bool {
        /* TU CÓDIGO: Retorna false si buffer está lleno */
        let mut buffer = self.frames.lock().unwrap();
        if buffer.len() < self.max_size{
            buffer.push_back(frame);
            return true; 
        } 
        return false;
    } 
    
    fn consumir_frame(&self) -> Option<Frame> {
        /* TU CÓDIGO: Retorna None si buffer está vacío */
        let mut buffer = self.frames.lock().unwrap(); 
        if !buffer.is_empty(){
            return buffer.pop_front();
        } 
        return None; 
    }
}

pub fn ejercicio_5() {
    let max_size: usize = 400;
    let buffer = Arc::new(VideoBuffer::new(max_size));
    let buffer_clon = buffer.clone(); 
    
    // Hilo productor
    let productor = thread::spawn(move || {
        let mut rng = rand::thread_rng();
        let mut produce_frame = true; 
        let mut id = 0; 
        while produce_frame{
            let mut data = vec![0u8; 5];
            rng.fill(&mut data[..]);
            let frame = Frame{id, data}; 
            produce_frame = buffer_clon.producir_frame(frame); 
            println!("\t Se produjo el buffer: {}", id); 
            id+=1; 
        }
    });
    
    // Hilo consumidor
    //thread::sleep(Duration::from_millis(10));
    let consume = thread::spawn(move || {
        let mut count = 0; 
        //let mut wait_time = 1;  
        let mut try2consume = 0;
        loop{
            if let Some(frame) = buffer.consumir_frame() {
                println!("Id frame: {}", frame.id);
                println!("Data frame: {:?}", frame.data);
                try2consume = 0;
            } else if try2consume < 3 
            { 
                try2consume += 1;  
                println!("Buffer vacío, intentando consumir... Intento: {}", try2consume);
                thread::sleep(Duration::from_millis(1));
            }
            else { break; }
            if count >= max_size*512{
                println!("Se alcanzó el límite de consumo, terminando...");
                break; 
            }
            count += 1;
        }
    });
    productor.join().unwrap(); 
    consume.join().unwrap();    
    println!("FInalizado el buffer\n"); 
    // Esperar terminación
    /* TU CÓDIGO */
}
