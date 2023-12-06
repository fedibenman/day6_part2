use std::fs::File;
use std::io::{self, BufRead, Read};

fn main() -> io::Result<()> {
    let file = File::open("./file.txt")?;
    let mut reader = io::BufReader::new(file);
let mut res = 1 ;
    let mut line_numbers: Vec<String> = Vec::new();
    let mut Time :Vec<i32> = Vec::new(); 
    let mut Distance:Vec<i32> = Vec::new();
    let mut time_int ="".to_string() ; 
    let mut dist_int ="".to_string() ; 
    let firstline = reader.by_ref().lines().nth(0);
match firstline {
    Some(Ok(text)) =>{
        let (_,s) = text.split_once(":").unwrap();
        Time = s.split_whitespace().map(|num| num.parse().unwrap()).collect();
        for (i,time) in Time.iter().enumerate(){
            time_int.push_str(&time.to_string()) ; 
        }
        
        print!("{:?}",Time) ;
    },
    Some(Err(err)) => eprintln!("Error reading line: {}", err),
    None => eprintln!("No line at index 0"),
}
let secondline = reader.lines().nth(0);
match secondline {
    Some(Ok(text)) =>{
        let (_,s) = text.split_once(":").unwrap();
        Distance= s.split_whitespace().map(|num| num.parse().unwrap()).collect() ;
        print!("{:?}",Distance);
     
        for (i,dist) in Distance.iter().enumerate(){
            dist_int.push_str(&dist.to_string()) ; 
        }
        },

    Some(Err(err)) => eprintln!("Error reading line: {}", err),
    None => eprintln!("No line at index 0"),
}



     
   let mut first_possible_speed = 0 ;
   print!("dist int {}",dist_int);
        let dist = dist_int.parse::<i64>().unwrap() ;
        let time_integer = time_int.parse::<i64>().unwrap() ;
        print!("{}",time_integer) ;
        print!("{}",dist) ;
        let mut sum_possibilities :i64= 0 ; 
        let mut last_speed:i64 = 0;
        for  vitess in 0..time_integer {
            
            {if ((time_integer-vitess)*vitess) > dist{
                if first_possible_speed ==0 {
                first_possible_speed = vitess ; 
                }
                 }

                if ((time_integer-vitess)*vitess) > dist && ((time_integer-vitess-1)*vitess-1) < dist  
            {last_speed = vitess ;
               break ;
               }
        }   }
        print!(" sum posiibilites {} ",last_speed-first_possible_speed+1) ;
   
      
    Ok(())
}
