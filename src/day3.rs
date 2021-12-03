use std::borrow::Borrow;
use crate::{Command, Submarine, utils};

pub fn get_result() -> isize {
    let report=utils::read_file("inputs/3.txt");
    let len = report.get(0).unwrap().len();
    let mut ones = vec![0;len];
    let mut zeros = vec![0;len];

    for line in report.iter() {
        for i in 0..len {
            let mut chars=line.chars();
            let chara = chars.nth(i).unwrap();
            match chara {
                '0'  => {
                    let val= zeros.get(i).unwrap() + 1;
                    zeros[i]=val;
                },
                '1'  => {

                    let val= ones.get(i).unwrap() + 1;
                    ones[i]=val;
                },
                _ => panic!("WTF")
            }
        }
    }
    let mut gamma_rate = String::new();
    let  mut epsilon_rate = String::new();
    for i in 0..len {
        if zeros[i] > ones[i] {
            epsilon_rate.push('1');
            gamma_rate.push('0');
        } else if zeros[i] < ones [i]{
            epsilon_rate.push('0');
            gamma_rate.push('1');
        } else{
            panic!("Damn")
        }
    }
    let gamma = isize::from_str_radix(gamma_rate.borrow(), 2).unwrap();
    let epsilon = isize::from_str_radix(epsilon_rate.borrow(), 2).unwrap();
    let power_consumption = gamma * epsilon;



    println!("Power consumption: {}",power_consumption);
    return power_consumption;
}