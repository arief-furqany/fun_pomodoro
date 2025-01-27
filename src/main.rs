#[warn(clippy::pedantic)]
use std::io;
use std::thread;
use std::time::Duration;
use std::process::Command;


fn main() {
    println!("Halo Pemalas!");

    //input jumlah menit belajar
    let menit_belajar = loop {
        println!("tumben mau belajar, berapa menit doang emangnya?");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("wkwkwk.. saking malesnya ngehargain waktu");
        match input.trim().parse::<u32>(){
            Ok(minutes) => break minutes,
            Err(_) => println!("input tidak valid, masukkan angka")
        }
    };

    //input jumlah menit istirahat
    let menit_istirahat = loop {
        println!("mau males-malesan berapa menit bang?");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("berarti gaboleh istirahat");
        match input.trim().parse::<u32>(){
            Ok(minutes)=> break minutes,
            Err(_)=> println!("input tidak valid, masukkan angka")
        }
    };

    println!("mulai belajar: {} menit, istirahat: {} menit", menit_belajar, menit_istirahat);

    loop{
        //timer untuk belajar
        println!("mulai grinding selama {} menit", menit_belajar);

        //masukkan directory lagu proses pomodoro
        start_music("/home/kanoy/Documents/rust/fun_pomodoro/Memory_reboot.mp3");
        countdown_timer(menit_belajar);
        stop_music();
        println!("bukan waktunya males-malesan");

        //timer untuk istirahat
        println!("mulai istirahat selama {} menit", menit_istirahat);

        //masukkan directory untuk relax
        start_music("/home/kanoy/Documents/rust/fun_pomodoro/Sailor_Song.flac");
        countdown_timer(menit_istirahat);
        stop_music();
        print!("waktunya lanjut grinding");
    }

    fn countdown_timer(minutes: u32){
    println!("");
        for remaining in (1..=minutes * 60).rev() {
            let minutes_left = remaining / 60;
            let second_left = remaining % 60;
            print!("\r{:02}:{:02} tersisa", minutes_left, second_left);
            io::Write::flush(&mut io::stdout()).expect("gagal menyegarkan output");
            thread::sleep(Duration::from_secs(1));
        }
        println!("\n Waktu Habis");
    }
    
    fn start_music(file_path: &str){
        Command::new("ffplay")
            .arg("-nodisp")
            .arg("-autoexit")
            .arg(file_path)
            .spawn()
            .expect("gagal memutar musik");
    }

    fn stop_music(){
        Command::new("pkill")
            .arg("-f")
            .arg("ffplay")
            .output()
            .expect("gagal menghentikan music");
    }

}
