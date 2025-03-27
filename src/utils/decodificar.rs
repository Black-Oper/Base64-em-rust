use std::io::Write;

use crate::utils::codificar::separa_string_binaria;

pub fn decodificar_string_base64() {
    let mut input = String::new();

    print!("Informe uma string: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();

    let s = converte_base64_bin(&input);
    let sbin = separa_string_binaria(&s, 8);
    let sutt = binario_para_texto(&sbin);

    println!("\nPalavra: {}", sutt);
}

fn converte_base64_bin(string: &str) -> String {
    let str_b64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut str_bin = String::new();

    for caracter in string.chars() {
        if caracter == '=' {
            continue;
        }
        let mut i = 0;
        for cbin in str_b64.chars() {
            i += 1;
            if caracter == cbin {
                str_bin.push_str(&format!("{:06b}", i - 1));
                break;
            }
        }
    }
    str_bin
}

fn binario_para_texto(bin: &str) -> String {
    bin.split_whitespace()
       .filter_map(|byte| {
            if byte.len() == 8 {
                u8::from_str_radix(byte, 2).ok().map(|b| b as char)
            } else {
                None
            }
       })
       .collect()
}
