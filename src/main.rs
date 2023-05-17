

#[derive(Debug)]
pub struct LRUStack<T> {
 	pub stack: Vec<Option<Box<T>>>,
}
	
//A*B = C
fn nmm(A: Vec<Vec<f32>>, B: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut C: Vec<Vec<f32>> = vec![vec![0.0; B[0].len()]; A.len()];

    for i in 0..A.len(){
        for j in 0..B[0].len(){
            for k in 0..A[0].len(){
                C[i][j] += A[i][k]*B[k][j];
            }
        }
    }

    C
}

impl<T: PartialEq> LRUStack<T> {

	pub fn new() -> LRUStack<T> {
		LRUStack {
			stack: Vec::<Option<Box<T>>>::new(),
		}
	}

	pub fn rec_access(&mut self, val: T) -> Option<u32> {
		if self.stack.len() == 0 {
			self.stack.push(Some(Box::new(val)));
			return None;
		}

	    if **self.stack[0].as_ref().unwrap() == val {
			return Some(1);
	    }

	    let mut last = self.stack[0].take(); 
	    for pos in 1..self.stack.len() {
			let temp = self.stack[pos].take();
			self.stack[pos] = last;
			last = temp; 
			if **last.as_ref().unwrap() == val {
		    	self.stack[0] = last;
				return Some(pos as u32 + 1);
			}
	    }
		// a cold miss
		self.stack.push( last ); // add to the end of the vector
	    self.stack[0] = Some(Box::new(val));
	    return None;
	}
}

pub struct LRUVec {
    pub stack: Vec<String>,
}

impl LRUVec {

   pub fn new() -> LRUVec {
       LRUVec {
           stack: Vec::new(),
       }
   }

   pub fn rec_access(&mut self, val: &str) -> Option<u32> {
       let mut dist: Option<u32> = None;
       if self.stack.contains(&val.to_string()){
           let pos = self.stack.iter().position(|x| *x == val).unwrap();
           dist = Some((self.stack.len() - pos) as u32);
           self.stack.remove(pos);
           self.stack.push(val.to_string());
       }
       else{
           self.stack.push(val.to_string());
       }
       return dist;
   }
}

use std::env;
use std::time::{Instant, Duration};

use std::error::Error as OError;
use csv::Writer;

use rand::Rng;

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string(),
    }
}

fn generate_data(size: usize, mode: &String, flag: u8) -> Vec<String>{
    
    match mode.as_str(){
        "Cyclic" => {
            let mut data = Vec::new();
            // for _ in 0..repeat{
                for i in 0..size{
                    // println!("{:?}", i);
                    data.push(i.to_string());
                }
            // }  
            data
        },
        "Sawtooth" => {
            let mut data = Vec::new();
            // // for r in 0..repeat{
                if flag == 0 {
                    for i in 0..size{
                        // println!("{:?}", i);
                        data.push(i.to_string());
                    }
                }else{
                    for i in (0..size).rev(){
                        // println!("{:?}", i);                                                
                        data.push(i.to_string());
                    }
                }
            // }
            data
        },
        "Random" => {
            let mut data = Vec::new();
            let mut rng = rand::thread_rng();
            // for _ in 0..repeat{
                for _ in 0..size{
                    data.push(rng.gen_range(0..size).to_string());
                }
            // }
            data
        }
        _ => {
            Vec::new()
        }
    } 
}

fn main() {
    let args: Vec<String> = env::args().collect(); 
    if args.len() < 6 {
        println!("Format: exe mode mem_size data_size test_mode");
        return;
    }

    let mode = &args[1];
    let mem_size = args[2].parse::<u32>().unwrap();
    // let re_path = &args[3];
    let d_size = &args[3].parse::<usize>().unwrap();
    let t_mode = &args[4];
    let repeat = &args[5].parse::<usize>().unwrap();
    let mut flag: u8 = 0;
    let mut total_time = Duration::new(0, 0);
    let mut total_mis = 0;
    for r in 0..*repeat{
        println!("repeating {} time.", r + 1);
        // println!("Data generation start.");
        let data = generate_data(*d_size, t_mode, flag);
        flag ^= 1;
        // println!("Data generation finish.");
        let all_size = &data.len();
        let mut miss = 0;
        let start = Instant::now();
        let mut count = 0;
        let res = match mode.as_str(){
            "Stack" =>{
                let mut analyzer = LRUStack::<String>::new();
                let mut dists = Vec::new();
                for c in &data {
                    if count % 1000 == 0{
                        // println!("{}", count);
                    }
                    count += 1;
                    let cur = analyzer.rec_access(c.to_string());
                    // println!("{:?}", cur);
                    dists.push((c, cur, if cur.unwrap_or(0) > mem_size 
                    {
                        miss += 1;
                        "Miss"
                    } else {"Hit"}));
                }
                dists            
            },
            "Vec" => {
                let mut analyzer = LRUVec::new();
                let mut dists = Vec::new();
                for c in &data {
                    // println!("{}", count);
                    count += 1;
                    let cur = analyzer.rec_access( &c.to_string() );
                    dists.push((c, cur, if cur.unwrap_or(0) > mem_size 
                    {
                        miss += 1;
                        "Miss"
                    } else {"Hit"}));
                }
                dists
            },
            _ => {
                println!("Mode Stack or Mode Vec.");
                Vec::new()
            },
        };
        let duration = start.elapsed();
        total_time += duration;
        println!("{:?}", duration);
        println!("missing rate: {:.3?}\n", miss as f32 / *all_size as f32);
        total_mis += miss;
        let csv_path = get_current_working_dir() + "/" + mode + "_" + t_mode + &(r + 1).to_string() + ".csv";
        match save_csv(&csv_path, &res){
            Ok(_) => {
                println!("csv path: {:?}", csv_path);
            },
            Err(m) => {
                println!("{:?}", m);
            },
        }
    }
    println!("Total Time: {:?}, Avg Time: {:?}", total_time, total_time / *repeat as u32);
    println!("Avg Miss Rate: {:.3?}", total_mis as f32 / (d_size * repeat) as f32);

}


fn save_csv(path: &String, data: &Vec<(&String, Option<u32>, &str)>) -> Result<(), Box<dyn OError>> {
    let mut wtr = Writer::from_path(path)?;
    for i in data{
        wtr.write_record(&[i.0, &i.1.unwrap_or(0).to_string(), i.2])?;
    }
    wtr.flush()?;
    Ok(())
}
