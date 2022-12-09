use std::io;

fn main() {
    println!("Are you are a Public Servant? YES = 1 , NO = 2");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Could not read it");
    let ans:i32 = input1.trim().parse().expect("Not valid");

    if ans == 1 {
        println!("\nWhat professional are you?");
        println!("Office Admin = 1");
        println!("Academic = 2");
        println!("Lawyer = 3");
        println!("Teacher = 4");
        
        let mut prof = String::new();
        io::stdin().read_line(&mut prof).expect("INVALID");
        let tprof:i32 = prof.trim().parse().expect("invalid");

        println!("\nHow many years experience do you have?");
        let mut xp = String::new();
        io::stdin().read_line(&mut xp).expect("inv");
        let xp:i32 = xp.trim().parse().expect("nd");

        if xp >= 1 && tprof ==1 {
            println!("Staff is Intern");
        }
        else if xp >3  && tprof ==1 {
            println!("Staff is Administrator");
        }
        else if xp >5  && tprof ==1 {
            println!("Staff is Senior Administrator");
        }
        else if xp >8  && tprof ==1 {
            println!("Staff is office manager");
        }
        else if xp >10  && tprof ==1 {
            println!("Staff is Director");
        }
        else if xp ==14  && tprof ==1 {
            println!("Staff is CEO");
        }
        
        //----------------
        if xp >= 1 && tprof ==2 {
            println!("Staff is -");
        }
        else if xp >3  && tprof ==2 {
            println!("Staff is PhD Research Assistan");
        }
        else if xp >5  && tprof ==2 {
            println!("Staff is PhD candidate");
        }
        else if xp >8  && tprof ==2 {
            println!("Staff is Post-Doc Researcherr");
        }
        else if xp >10  && tprof ==2 {
            println!("Staff is senior lecturer");
        }
        else if xp ==14  && tprof ==2 {
            println!("Staff is Dean");
        }


        else {
            println!("You are not eligible to take this poll");
        }


    }
}
