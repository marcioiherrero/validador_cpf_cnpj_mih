Instalação:
-----------
No arquivo Cargo.toml de seu projeto, adicionar a dependência na seção: "dependencies" como abaixo:
```rust
[dependencies]
validador_cpf_cnpj_mih = "0.1.1"
```

Exemplo de utilização:
----------------------
```rust
use validador_cpf_cnpj_mih as vd;
use std::io;

fn main() {
  // TESTE COM CPF
  println!("Digite o cpf: ");
  let mut cpf = String::new();
  match io::stdin().read_line(&mut cpf) {
    Ok(_) => {
      println!("Você digitou: {}", cpf.trim());
    },
    Err(e) => {
      println!("Erro ao ler entrada: {}", e);
    }
  }
  
  let validado = vd::validadores::cpf(cpf.as_str());
  
  if validado {
    println!("O CPF é válido.");
  } else {
    println!("O CPF é inválido.");
  }
  
  // TESTE COM CNPJ
  println!("Digite o cnpj: ");
  let mut cnpj = String::new();
  match io::stdin().read_line(&mut cnpj) {
    Ok(_) => {
      println!("Você digitou: {}", cnpj.trim());
    },
    Err(e) => {
      println!("Erro ao ler entrada: {}", e);
    }
  }
  
  let validado = vd::validadores::cnpj(cnpj.as_str());
  
  if validado {
    println!("O CNPJ é válido.");
  } else {
    println!("O CNPJ é inválido.");
  }
  
}
```
