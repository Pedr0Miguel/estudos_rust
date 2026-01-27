fn main() {
    let vip = false;
    let admin = false;
    let pode_entrar = vip || admin;

    println!("O usuário pode entrar? {}", pode_entrar)
}
