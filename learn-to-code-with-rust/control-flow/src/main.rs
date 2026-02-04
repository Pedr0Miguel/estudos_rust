fn countdown(seconds: i32) {
    if seconds == 0 {
        println!("Acabou.");
        return;
    }

    println!("{seconds} para acabar...");

    countdown(seconds - 1);
}

fn main() {
    countdown(5);
}
