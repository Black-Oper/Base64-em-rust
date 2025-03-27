mod utils;
use utils::menu::{limpar_tela, exibir_menu, esperar_enter};
use utils::codificar::converter_string_base64;
use utils::decodificar::decodificar_string_base64;
use std::process::exit;

fn main() {
    limpar_tela();
    loop {
        let itens = ["Codificar", "Decodificar"];

        let selecionado = exibir_menu("BASE64", &itens, true);
        limpar_tela();

        match selecionado {
            1 => converter_string_base64(),
            2 => decodificar_string_base64(),
            _ => exit(0),
        }

        esperar_enter();
    }
  
}