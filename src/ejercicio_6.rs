
/* 
### Ejercicio 6: Sistema de Cache con RwLock ⭐⭐⭐
**Objetivo:** Implementar un cache thread-safe optimizado para lecturas

*/
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use rand::Rng;
use std::thread;

type Video = HashMap<String, Vec<u8>>;
                            
struct VideoCache {
    data: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl VideoCache {
    fn new() -> Self {
        Self{
            data: Arc::new(RwLock::new(HashMap::new()))
        }
    }
    
    fn get(&self, key: &str) -> Option<Vec<u8>> {
        let cache = self.data.read().unwrap(); 
        if cache.contains_key(key){
            return cache.get(key).cloned(); 
        }
        else{None}
    }
    
    fn set(&self, key: String, value: Vec<u8>) {
        let mut cache = self.data.write().unwrap(); 
        if !cache.contains_key(&key){
            println!("✍️ Se introdujo el video {} con los datos {:?}", key, value);
            cache.insert(key, value); 
        }
    }
    
    fn contains(&self, key: &str) -> bool {
        let cache = self.data.read().unwrap(); 
        if cache.contains_key(key){
            return true;
        }else{return false;}
    }
}

// RETO: Crea 10 hilos lectores y 2 escritores
pub fn ejercicio_6() {
    let video_size = 5; 
    print!("\tEJERCICIO 6 VIDEO CACHE\n");
    let cache = Arc::new(VideoCache::new()); 
    let mut handlers = vec![]; 
    let num_videos = 100;
    
    for escritor_id in 1..=2{
        let cache_clone = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            for v in 1..=num_videos{
                let mut datos_video = Vec::new();
                for _ in 0..video_size {
                    datos_video.push(rng.gen_range(0..255));
                }
                let nombre = format!("video_escritor{}_{}", escritor_id, v);
                cache_clone.set(nombre, datos_video);
            }
        }); 
        handlers.push(handle);
    } 

    let mut hanlders_lectores = vec![];

    for lector_id in 1..=5{
        let cache_clone = Arc::clone(&cache); 
        let handle = thread::spawn(move || {
            for v in 1..=num_videos{
                for trys in 1..=6{
                    let escritor_id = (trys%2)+1; 
                    let nombre = format!("video_escritor{}_{}", 
                        escritor_id, v);
                    if cache_clone.contains(&nombre){
                        let datos_video = cache_clone.get(&nombre).unwrap();
                        println!("🔍 Lector {} encontró el video {} con los datos {:?}", 
                            lector_id, nombre, datos_video);
                        break;  // Encontró el video, sal del loop de intentos
                    }
                    else {
                        println!("Lector {} fallo en leer los datos", lector_id); 
                        println!("Intento {trys} de ... 6"); 
                    }
                }
            }
        });
        hanlders_lectores.push(handle);
    }

    // Esperar a que todos los hilos terminen
    for handle in handlers {
        handle.join().unwrap();
    }

    for handle in hanlders_lectores{
        handle.join().unwrap(); 
    }
    
    println!("\n✅ Escritores y lectores terminaron");
}