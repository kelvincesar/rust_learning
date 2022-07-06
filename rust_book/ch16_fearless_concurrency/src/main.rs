use std::{thread, time::Duration, sync::mpsc, vec, sync::Mutex};

use std::sync::Arc;

fn main() {
    println!("Chapter 16 - Fearless Concurrency");

    // A thread morre quando a main terminar...
    // utilizar join para aguardar a finalização.

    let handle = thread::spawn(|| {
        for i in 1..10{
            println!("number [{}] from spawn thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for i in 1..5 {
        println!("number [{}] from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    //handle.join().unwrap();

    let v = vec![1, 2, 3];

    let handle_2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle_2.join().unwrap();

    /*
        Usando channels
    */
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        //let msg = String::from("Kelvin Andrade");
        //tx.send(msg).unwrap();
        
        // Gera erro pois poderia estar alterando a mensagem...
        //println!("msg is {}", msg);

        let msgs = vec![
            String::from("Olá"),
            String::from("meu nome é"),
            String::from("Kelvin César"),
            String::from("de Andrade")
        ];
        
        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    // Criando dois produtores:
    thread::spawn(move || {
        let msgs = vec![
            String::from("Hoje"),
            String::from("é o dia"),
            String::from("05/07/2022"),
            String::from(":)")
        ];
        
        for msg in msgs {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    /* 
    // try_recv para nao bloquear;
    let received = rx.recv().unwrap();

    println!("Recebido: {}", received);
    */

    for received in rx{
        println!("Mensagem recebida: {}", received);
    }

    /*
        Usando mutex e compartilhamento de variáveis
    */

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } // release é realizado automaticamente ao sair do escopo do lock
    println!("m = {:?}", m);

    // Contador incrementado por multiplas threads

    // Spawn a few threads to increment a shared variable (non-atomically), and
    // let the main thread know once all increments are done.
    //
    // Here we're using an Arc to share memory among threads, and the data inside
    // the Arc is protected with a mutex.

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // The shared state can only be accessed once the lock is held.
        // Our non-atomic increment is safe because we're the only thread
        // which can access the shared state when the lock is held.
        //
        // We unwrap() the return value to assert that we are not expecting
        // threads to ever fail while holding the lock.
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            // the lock is unlocked here when `data` goes out of scope.
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Contagem final: {:?}", *counter);

}
