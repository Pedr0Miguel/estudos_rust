fn main() {
    let values = [4, 5, 45, 23, 75, 22];
    let my_slice = &values[..2];

    dbg!(my_slice);

    let my_slice = &values[2..6];
    dbg!(my_slice);

    let my_slice = &values[2..];
    dbg!(my_slice);

    let my_slice: &[i32] = &values[..];
    dbg!(my_slice);

    let my_slice: &[i32; 6] = &values;
    dbg!(my_slice);
}
