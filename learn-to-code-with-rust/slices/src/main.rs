fn main() {
    let values = [4, 5, 45, 23, 75, 22];

    let regular_ref: &[i32; 6] = &values;

    diz_tamanho(regular_ref);

    let slice_3: &[i32] = &values[..3];
    diz_tamanho(slice_3);
}

fn diz_tamanho(reference: &[i32; 6]) {
    println!("{}", reference.len());
}
