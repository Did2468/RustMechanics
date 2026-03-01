#[derive(Default,Clone,Copy)]
struct Process {
    pid: i32,
    at: i32,
    bt: i32,
    ct: i32,
    tat: i32,
    rt: i32,
    wt: i32
}

use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter the number of processes:");
    io::stdin().read_line(&mut input).expect("failed to read line");
    let n: i32 = input.trim().parse().expect("Number expected");
    let mut p = vec![Process::default();n as usize];

    let mut buffer = String::new();
    
    // taking input logic
    for i in 0..n {
        p[i as usize].pid = i;
        println!("Enter the arrival time of process {}",i);
        buffer.clear();
        io::stdin().read_line(&mut buffer).expect("failed");
        let x: i32 = buffer.trim().parse().expect("number expected");
        println!("Enter the burst time of process {}",i);
        buffer.clear();
        io::stdin().read_line(&mut buffer).expect("error");
        let y: i32 = buffer.trim().parse().expect("number");
        p[i as usize].at = x;
        p[i as usize].bt = y;
        p[i as usize].rt = p[i as usize].bt;
    }

    let mut completed: i32 = 0;
    let mut time: i32 = 0;

    while completed<n {
        let mut idx: i32 = -1;
        let mut min_rt: i32 = 10000;

        for i in 0..n {
            if p[i as usize].at<=time && p[i as usize].rt>0 && p[i as usize].rt < min_rt {
                idx = i;
                min_rt = p[i as usize].rt;
            }
        }

        if idx == -1 {
            time = time+1;
        }
        else {
            p[idx as usize].rt = p[idx as usize].rt -1;
            time = time +1;
            if p[idx as usize].rt==0 {
                p[idx as usize].ct = time;
                p[idx as usize].tat = p[idx as usize].ct - p[idx as usize].at;
                p[idx as usize].wt = p[idx as usize].tat - p[idx as usize].bt;
                completed = completed +1;
            }
        }
    }
    println!("PID\tAT\tBT\tCT\tTAT\tWT\n");
    for i in 0..n {
        println!("{}\t{}\t{}\t{}\t{}\t{}\n",p[i as usize].pid,p[i as usize].at,p[i as usize].bt,p[i as usize].ct,p[i as usize].tat,p[i as usize].wt);
    }
}
