fn main() {
    
    geopo_merger();
    Pub_Service();
}

fn geopo_merger() {
    // The separate arrays in the cloud
    let name = ["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogobona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministry = ["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let geozone = ["South West", "North East", "South South", "South West", "South East"];

    // Printing out the data to the user
    println!("\n ---------- MERGED DATA FROM ARRAYS ----------");
    println!("NAME | MINISTRY | GEOPOLITICAL ZONE");
    println!("\n1. {} | {} | {}", name[0], ministry[0], geozone[0]);
    println!("2. {} | {} | {}", name[1], ministry[1], geozone[1]);
    println!("3. {} | {} | {}", name[2], ministry[2], geozone[2]);
    println!("4. {} | {} | {}", name[3], ministry[3], geozone[3]);
    println!("5. {} | {} | {}", name[4], ministry[4], geozone[4]);
}

use std::io;

fn Pub_Service() {
    println!("\nAre you are a Public Servant? YES = 1 , NO = 2");
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

        if xp == 1 || xp == 2 && tprof ==1 {
            println!("Staff is Intern");
        }
        else if xp == 3 || xp == 4 || xp == 5  && tprof ==1 {
            println!("Staff is Administrator");
        }
        else if xp == 6 || xp == 7  && tprof ==1 {
            println!("Staff is Senior Administrator");
        }
        else if xp == 8 || xp == 9 || xp == 10 && tprof ==1 {
            println!("Staff is office manager");
        }
        else if xp == 11 || xp == 12 || xp == 13 && tprof ==1 {
            println!("Staff is Director");
        }
        else if xp ==14  && tprof ==1 {
            println!("Staff is CEO");
        }
        
        //----------------
        if xp == 1 || xp == 2 && tprof ==2 {
            println!("Staff is -");
        }
        else if xp == 3 || xp == 4 || xp == 5  && tprof ==2 {
            println!("Staff is PhD Research Assistan");
        }
        else if xp == 6 || xp == 7  && tprof ==2 {
            println!("Staff is PhD candidate");
        }
        else if xp == 8 || xp == 9 || xp == 10 && tprof ==2 {
            println!("Staff is Post-Doc Researcherr");
        }
        else if xp == 11 || xp == 12 || xp == 13 && tprof ==2 {
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

