use rand::Rng;

fn rand_sequence(n: usize) -> String {
    let letters = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut rng = rand::thread_rng();

    let mut bytes: Vec<u8> = Vec::with_capacity(n);
    for i in 0..n {
        let n2: usize = rng.gen();
        let m = letters[n2 % letters.len()];
        bytes[i] = m
    }

    return format!("{:?}", &bytes);
}

pub fn generate_salt(mut length: usize) -> String {
    if length <= 0 {
        length = 50
    }
    return rand_sequence(length);
}
