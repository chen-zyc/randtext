use rand::Rng;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "randtext", about = "随机生成指定数量的汉字(For PeiMin Hang)")]
struct Args {
    #[structopt(short = "n", long = "nums", default_value = "10")]
    num_of_chars: u32,
}

fn main() {
    let args = Args::from_args();
    gen_random_text(args.num_of_chars);
}

fn gen_random_text(n: u32) {
    let r = (0x4E00 as u32, 0x9FA5 as u32);
    let mut rng = rand::thread_rng();
    let max_chars_to_print = 1000 as usize;
    let mut buf = String::with_capacity(1024); // 每个汉字大概是 3 字节。
    for _i in 0..n {
        let c = std::char::from_u32(rng.gen_range(r.0, r.1)).unwrap();
        buf.push(c);
        if buf.len() >= max_chars_to_print {
            print_content(&mut buf);
        }
    }
    if buf.len() > 0 {
        print_content(&mut buf);
    }
}

fn print_content(c: &mut String) {
    print!("{}", c);
    c.clear();
}
