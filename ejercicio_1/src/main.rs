use std::thread;
use std::sync::mpsc;
use std::time::Duration;

struct Account(i32);

impl Account {

    fn deposit(&mut self, amount: i32) {
        println!("op: deposit {}, available funds: {:?}", amount, self.0);
        self.0 += amount;
    }
    
    fn withdraw(&mut self, amount: i32) {
        println!("op: withdraw {}, available funds: {}", amount, self.0);
        if self.0 >= amount {
            self.0 -= amount;
        } else {
            panic!("Error: Insufficient funds.")
        }
    }
    
    fn balance(&self) -> i32 {
        self.0
    }
}

/*
    El problema que hay en este caso es que se está transfiriendo el ownership de la cuenta en cada move
    Por lo tanto para solucionar este problema vamos a usar la comunicacion entre canales.

    Por otro lado saqué la variable estatica (mutable) desde varios hilos a la vez generando race conditions
*/

fn main() {

    let mut cuenta_bancaria: Account = Account(0);

    let (primer_emisor, primer_receptor): (mpsc::Sender<Account>, mpsc::Receiver<Account>) = mpsc::channel();
    let (segundo_emisor, segundo_receptor): (mpsc::Sender<Account>, mpsc::Receiver<Account>) = mpsc::channel();
    let (tercer_emisor, tercer_receptor): (mpsc::Sender<Account>, mpsc::Receiver<Account>) = mpsc::channel();
    let (cuarto_emisor, cuarto_receptor): (mpsc::Sender<Account>, mpsc::Receiver<Account>) = mpsc::channel();
    /*
        The way Rust’s standard library implements channels means a channel can have multiple 
        sending ends that produce values but only one receiving end that consumes those values
    */

    let customer1_handle = thread::spawn(move || -> Result<(),mpsc::RecvError>{
        let mut cuenta_bancaria = cuenta_bancaria;
        cuenta_bancaria.deposit(40);

        if let Err(_error) = primer_emisor.send(cuenta_bancaria){
            return Err(mpsc::RecvError);
        }
        Ok(())
    });
    
    let customer2_handle = thread::spawn(move || -> Result<(),mpsc::RecvError>{
        if let Ok(mut cuenta_bancaria) = primer_receptor.recv(){
            cuenta_bancaria.withdraw(30);
            segundo_emisor.send(cuenta_bancaria);
        }else{
            return Err(mpsc::RecvError);
        }
        Ok(())

        //let mut cuenta_bancaria = primer_receptor.recv().unwrap();
        //cuenta_bancaria.withdraw(30);
        //segundo_emisor.send(cuenta_bancaria).unwrap();

        /*
        match cuenta_bancaria{
            Account => println!("sos una cuenta pa"),
            RecvError => return Err(mpsc::RecvError),
        }
        */

        /*
        let mut cuenta_bancaria_ = match primer_receptor.recv(){
            Ok(_) => cuenta_bancaria,
            Err(RecvError) => return Err(mpsc::RecvError),
        };
        cuenta_bancaria.withdraw(30);
        segundo_emisor.send(cuenta_bancaria);
        Ok(())
        */
    });
    
    let customer3_handle = thread::spawn(move || -> Result<(),mpsc::RecvError>{

        if let Ok(mut cuenta_bancaria) = segundo_receptor.recv(){
            cuenta_bancaria.deposit(60);
            tercer_emisor.send(cuenta_bancaria);
        }else{
            return Err(mpsc::RecvError);
        }
        Ok(())

        //let mut cuenta_bancaria = segundo_receptor.recv().unwrap();
        //cuenta_bancaria.deposit(60);
        //tercer_emisor.send(cuenta_bancaria).unwrap();
        
        /*
        let mut cuenta_bancaria_ = match segundo_receptor.recv(){
            Ok(_) => cuenta_bancaria,
            Err(RecvError) => return Err(mpsc::RecvError),
        };
        cuenta_bancaria.deposit(60);
        tercer_emisor.send(cuenta_bancaria);
        Ok(())
        */
    });
    
    let customer4_handle = thread::spawn(move || -> Result<(),mpsc::RecvError>{

        if let Ok(mut cuenta_bancaria) = tercer_receptor.recv(){
            cuenta_bancaria.withdraw(70);
            cuarto_emisor.send(cuenta_bancaria);
        }else{
            return Err(mpsc::RecvError);
        }
        Ok(())

        //let mut cuenta_bancaria = tercer_receptor.recv().unwrap();
        //cuenta_bancaria.withdraw(70);
        //cuarto_emisor.send(cuenta_bancaria).unwrap();
        /*
        let mut cuenta_bancaria_ = match tercer_receptor.recv(){
            Ok(_) => cuenta_bancaria,
            Err(RecvError) => return Err(mpsc::RecvError),
        };
        cuenta_bancaria.withdraw(70);
        Ok(())
        */
    });
    
    let handles = vec![
    customer1_handle,
    customer2_handle,
    customer3_handle,
    ];
    
    for handle in handles {
        if let Err(_error) = handle.join(){
            panic!("Error con los threads");
        }
    }

    if let Ok(cuenta_bancaria) = cuarto_receptor.recv(){
        let savings = cuenta_bancaria.balance();
    
        println!("Balance: {:?}", savings);
    }else{
        panic!("Error con los threads");
    }

}