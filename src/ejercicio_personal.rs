/*
    Este es un ejercicio que se me ocurrio, se trata de implmentar casillas
    de votacion un vector de personas que representan a los votantes,
    cada persona tiene voto por uno de los 4 partidos, y hay tres casillas 
    de votacion, cada casilla tiene un contador de votos para cada partido,
    cada casilla representa un hilo, y cada persona vota en una casilla aleatoria. 

*/

use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use rand::Rng; 
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::time::Instant;  // ✅ Agregar


type FilaVotos = Arc<Mutex<VecDeque<u128>>>; 

struct Voto{
    voto: u128
}

impl Voto {
    fn new() -> Self{
        let mut rng = rand::thread_rng();
        Self {
            voto: rng.gen_range(0..4)  
        }
    }
}
struct Casilla{
    id_casilla: u128,
    fila_votos: FilaVotos, 
    partido_atole: i32, 
    partido_pnp: i32,
    partido_ntc: i32,
    patido_hereje: i32, 
    total_votos: i32,
    handle_count: Option<JoinHandle<(i32, i32, i32, i32)>>,
    handle_queue: Option<JoinHandle<()>>
}

impl Casilla{

    fn new(id_casilla:u128) -> Self{
        Self{ 
            id_casilla: id_casilla, 
            fila_votos: Arc::new(Mutex::new(VecDeque::new())),  // ✅ Crear el Ar,
            partido_atole: 0, 
            partido_pnp: 0,
            partido_ntc: 0,
            patido_hereje: 0, 
            total_votos: 0,
            handle_count: None,
            handle_queue: None
        }
    }

    fn votacion(&mut self){ 
        let fila = Arc::clone(&self.fila_votos);   
        let id = self.id_casilla; 
        self.handle_queue = Some(thread::spawn(move || {
            let inicio = Instant::now();
            let mut time_pass = inicio.elapsed();
            
            while time_pass.as_secs() < 3{
                let vote = Voto::new();
                let vote = vote.voto;
                {
                    let mut fila = fila.lock().unwrap(); 
                    fila.push_back(vote);
                } 
                
                println!("Se ha insertado voto en la casilla {}", id); 
                time_pass = inicio.elapsed(); 
            }
        }));
    }

    fn count_votes(&mut self){
        let fila = Arc::clone(&self.fila_votos);
        let id = self.id_casilla;
        self.handle_count = Some(thread::spawn(move || {
            let inicio = Instant::now();
            let mut time_pass = inicio.elapsed();
            let mut atole = 0;
            let mut ntc = 0;
            let mut pnp = 0;
            let mut hereje = 0; 
            let mut vote;  
            while time_pass.as_secs() < 5{
                {
                    let mut fila = fila.lock().unwrap(); 
                    if fila.is_empty(){
                        thread::sleep(Duration::from_millis(1));
                    }
                    vote = fila.pop_front(); 
                }
                match vote {
                    Some(0) => atole += 1,
                    Some(1) => ntc +=1, 
                    Some(2) => pnp +=1, 
                    Some(3) => hereje +=1, 
                    None => {}, 
                    _ => {},  // ✅ Cualquier otro valor
                }
                println!("Contando votos en la casilla {}, Atole: {}, NTC: {}, PNP: {}, Hereje: {}", 
                    id, atole, ntc, pnp, hereje); 
                println!("Para un total de {} votos contados", atole+ntc+pnp+hereje);
                time_pass = inicio.elapsed(); 
            } 
            (atole, ntc, pnp, hereje)
        }));
    }

    fn get_results(&mut self){
        if let Some(handle) = self.handle_queue.take() {
            handle.join().unwrap();
        } 
        if let Some(handle) = self.handle_count.take() {
            let (atole, ntc, pnp, hereje) = handle.join().unwrap(); 
            self.partido_atole = atole; 
            self.partido_ntc = ntc;
            self.partido_pnp = pnp;
            self.patido_hereje = hereje;
            self.total_votos = atole + ntc + pnp + hereje;
        }
    }
}

pub fn votaciones(){
    let mut casillas = vec![Casilla::new(1), Casilla::new(2), Casilla::new(3)];

    for casilla in &mut casillas{
        casilla.votacion();
        casilla.count_votes();
    }
    for casilla in &mut casillas{
        casilla.get_results();
    }
    println!("\nResultados finales de las casillas:");
    let mut total_atole = 0;
    let mut total_ntc = 0;
    let mut total_pnp = 0;
    let mut total_hereje = 0;
    let mut total_votos = 0;
    for casilla in &casillas{
        println!("Resultados de la casilla {}: Atole: {}, NTC: {}, PNP: {}, Hereje: {}", 
            casilla.id_casilla, casilla.partido_atole, casilla.partido_ntc, 
            casilla.partido_pnp, casilla.patido_hereje);
        println!("Total de votos en la casilla {}: {}", casilla.id_casilla, casilla.total_votos);
        println!("---------------------------------------------");
        total_atole += casilla.partido_atole;
        total_ntc += casilla.partido_ntc;
        total_pnp += casilla.partido_pnp;
        total_hereje += casilla.patido_hereje;
        total_votos += casilla.total_votos;
    }
    println!("Resultados totales: Atole: {}, NTC: {}, PNP: {}, Hereje: {}, Total: {}", 
        total_atole, total_ntc, total_pnp, total_hereje, total_votos);
    println!("---------------------------------------------");
    let votos_max = total_atole.max(total_ntc).max(total_pnp).max(total_hereje);
    let ganador = if votos_max == total_atole {
        "Atole"
    } else if votos_max == total_ntc {
        "NTC"
    } else if votos_max == total_pnp {
        "PNP"
    } else {
        "Hereje"
    };
    println!("El ganador es: {}", ganador);
}