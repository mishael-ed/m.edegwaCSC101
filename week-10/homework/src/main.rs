use std::io::Write;
use std::io;
use std::fs::OpenOptions;

fn main() {
    //code reader for the HR
    println!("Enter code to create user files: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("failed to read input");
    let incode:i32 = input1.trim().parse().expect("failed");
    
    
    //function selector
    if incode == 7 {
        code_7();
        println!("\nSuccessfully Created");
    }
    else if incode == 8 {
        code_8();
        println!("\nSuccessfully Created");
    }
    else if incode == 9 {
        code_9();
        println!("\nSuccessfully Created");
    }
    else {
        println!("Your code was not found. Try again!");
    }
    
}

fn code_7() {
    let name = ["Aigona Juliet", "Ehis Ero", "Adamu Sagamu", "Akpevwe Iloka", "Maria Akinsola", "Gbenga Daniels"];
    let dept = ["Consulting", "Strategy", "Tax", "Assurance", "Transactions and corporate finance", "People and workforce"];
    let qual = ["B.Sc", "M.Sc", "B.Sc", "HND", "M.Sc", "HND"];
    let code = ["7", "9", "8", "7", "9", "8"];

    //THIS IS THE CODE FOR AIGONA'S STUFF
    //getting the information into a variable for later appending
    let aj_info = "NAME: ".to_owned() + name[0] + "\nDEPARTMENT: " + dept[0] + "\nQUALIFICATION: " + qual[0] + "\nCODE: " + code[0];
    let consulting_services = "\n\n----- CONSULTING SERVICES OF EY -----\nAnalytics consulting services \nCustomer Experience \nCybersecurity, risk, compliance and resilience \nDigital transformation \nRisk consulting services \nSupply chain and operations \nTechnology transformation";

    //creating the file... 
    let mut aj = std::fs::File::create("Aigona Juliet.txt").expect("create failed");
    
    //uploading data to file...
    aj.write_all(aj_info.as_bytes()).expect("write failed");
    
    //appending further data
    let mut append_aj = OpenOptions::new().append(true).open("Aigona Juliet.txt").expect("Could not open file");
    
    append_aj.write_all(consulting_services.as_bytes()).expect("write failed");

    //THIS IS THE CODE FOR AKPEVWE'S STUFF
    //getting the information into a variable for later appending
    let ai_info = "NAME: ".to_owned() + name[3] + "\nDEPARTMENT: " + dept[3] + "\nQUALIFICATION: " + qual[3] + "\nCODE: " + code[3];
    let assurance_services = "\n\n----- ASSURANCE SERVICES OF EY -----\nAudit Services \nClimate change and sustainability services \nFinancial accounting advisory services \nForensic and integrity services \nPrivate client audit experience \nAccounting Link \nAssurance";

    //creating the file... 
    let mut ai = std::fs::File::create("Akpevwe Iloka.txt").expect("create failed");
    
    //uploading data to file...
    ai.write_all(ai_info.as_bytes()).expect("write failed");
    
    //appending further data
    let mut append_ai = OpenOptions::new().append(true).open("Akpevwe Iloka.txt").expect("Could not open file");
    
    append_ai.write_all(assurance_services.as_bytes()).expect("write failed");
 
}

fn code_8() {
    let name = ["Aigona Juliet", "Ehis Ero", "Adamu Sagamu", "Akpevwe Iloka", "Maria Akinsola", "Gbenga Daniels"];
    let dept = ["Consulting", "Strategy", "Tax", "Assurance", "Transactions and corporate finance", "People and workforce"];
    let qual = ["B.Sc", "M.Sc", "B.Sc", "HND", "M.Sc", "HND"];
    let code = ["7", "9", "8", "7", "9", "8"];

    //THIS IS THE CODE FOR ADAMU'S STUFF
    //getting the information into a variable for later appending
    let as_info = "NAME: ".to_owned() + name[2] + "\nDEPARTMENT: " + dept[2] + "\nQUALIFICATION: " + qual[2] + "\nCODE: " + code[2];
    let tax_services = "\n\n----- TAX SERVICES OF EY -----\nTax planning \nTax function operations \nTax policy and controversy \nGlobal trade \nTax accounting \nTax compliance \nTransaction tax";

    //creating the file... 
    let mut pas = std::fs::File::create("Adamu Sagamu.txt").expect("create failed");
    
    //uploading data to file...
    pas.write_all(as_info.as_bytes()).expect("write failed");
    
    //appending further data
    let mut append_as = OpenOptions::new().append(true).open("Adamu Sagamu.txt").expect("Could not open file");
    
    append_as.write_all(tax_services.as_bytes()).expect("write failed");

    //THIS IS THE CODE FOR GBENGA'S STUFF
    //getting the information into a variable for later appending
    let gd_info = "NAME: ".to_owned() + name[5] + "\nDEPARTMENT: " + dept[5] + "\nQUALIFICATION: " + qual[5] + "\nCODE: " + code[5];
    let pw_services = "\n\n----- PEOPLE AND WORKFORCE SERVICES OF EY -----\nChange management and experience \nHR transformation \nIntegrated workforce mobility \nLearning and development consulting \nRecognition and reward advisory \nWorkforce analytics \nPeople and workforce";

    //creating the file... 
    let mut gd = std::fs::File::create("Gbenga Daniels.txt").expect("create failed");
    
    //uploading data to file...
    gd.write_all(gd_info.as_bytes()).expect("write failed");
    
    //appending further data
    let mut append_gd = OpenOptions::new().append(true).open("Gbenga Daniels.txt").expect("Could not open file");
    
    append_gd.write_all(pw_services.as_bytes()).expect("write failed");
 
}

fn code_9() {
    let name = ["Aigona Juliet", "Ehis Ero", "Adamu Sagamu", "Akpevwe Iloka", "Maria Akinsola", "Gbenga Daniels"];
    let dept = ["Consulting", "Strategy", "Tax", "Assurance", "Transactions and corporate finance", "People and workforce"];
    let qual = ["B.Sc", "M.Sc", "B.Sc", "HND", "M.Sc", "HND"];
    let code = ["7", "9", "8", "7", "9", "8"];

    //THIS IS THE CODE FOR EHIS' STUFF
    //getting the information into a variable for later appending
    let ee_info = "NAME: ".to_owned() + name[1] + "\nDEPARTMENT: " + dept[1] + "\nQUALIFICATION: " + qual[1] + "\nCODE: " + code[1];
    let strategy_services = "\n\n----- STRATEGY SERVICES OF EY -----\nStrategy Consulting \nCoporate and growth strategy \nTransaction strategy and execution \nRestructuring and turnaroud strategy \nIndustry Strategy \nDigital business building \nCommercial Strategy";

    //creating the file... 
    let mut ee = std::fs::File::create("Ehis Ero.txt").expect("create failed");
    
    //uploading data to file...
    ee.write_all(ee_info.as_bytes()).expect("write failed");
    
    //appending further data
    let mut append_ee = OpenOptions::new().append(true).open("Ehis Ero.txt").expect("Could not open file");
    
    append_ee.write_all(strategy_services.as_bytes()).expect("write failed");

    //THIS IS THE CODE FOR MARIA'S STUFF
    //getting the information into a variable for later appending
    let ma_info = "NAME: ".to_owned() + name[4] + "\nDEPARTMENT: " + dept[4] + "\nQUALIFICATION: " + qual[4] + "\nCODE: " + code[4];
    let tcf_services = "\n\n----- TRANSACTIONS AND CORPORATE FINANCE OF EY -----\nCorporate finance \nDivestments and carve-outs \nSustainability and ESG Services \nM&A Advisory \nM&A Integration \nM&A Technology and tools \nM&A advanced analytics";

    //creating the file... 
    let mut ma = std::fs::File::create("Maria Akinsola.txt").expect("create failed");
    
    //uploading data to file...
    ma.write_all(ma_info.as_bytes()).expect("write failed");
    
    //appending further data
    let mut append_ma = OpenOptions::new().append(true).open("Maria Akinsola.txt").expect("Could not open file");
    
    append_ma.write_all(tcf_services.as_bytes()).expect("write failed");
 
}


    



    
/*
    for i in 0..name.len() {
        let mut file = std::fs::File::create("name[i].txt").expect("Create failed");
        
        let mut info = [name[i], dept[i], qual[i], code[i]];
        

        file.write_all(info.as_bytes()).expect("Write failed");
    }*/
