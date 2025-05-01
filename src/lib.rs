pub mod validadores {
  
  pub fn cpf(cpf: &str) -> bool {
    let cpf: Vec<u8> = cpf
      .chars()
      .filter(|c| c.is_digit(10))
      .map(|c| c.to_digit(10).unwrap() as u8)
      .collect();
    
    if cpf.len() != 11 || cpf.iter().all(|&digit| digit == cpf[0]) {
      return false;
    }
    
    let mut sum1 = 0;
    let mut sum2 = 0;
    
    for i in 0..9 {
      sum1 += (cpf[i] as usize) * (10 - i);
      sum2 += (cpf[i] as usize) * (11 - i);
    }
    
    sum2 += cpf[9] as usize * 2;
    
    let digit1 = (sum1 * 10) % 11 % 10;
    let digit2 = (sum2 * 10) % 11 % 10;
    
    cpf[9] == digit1 as u8 && cpf[10] == digit2 as u8
  }
  
  pub fn cnpj(cnpj: &str) -> bool {
    let vec: Vec<u8> = cnpj
      .chars()
      .filter_map(|c| c.to_digit(10))
      .map(|d| d as u8)
      .collect();
    
    if vec.len() != 14 || vec.iter().all(|&d| d == vec[0]) {
      return false;
    }
    
    let validate_digit = |digit: usize| -> bool {
      let mut sum = 0;
      let mut weight = digit - 7;
      for &num in vec.iter().take(digit) {
          sum += num as usize * weight;
          weight = if weight == 2 { 9 } else { weight - 1 };
      }
      let remainder = sum % 11;
      let digit = if remainder < 2 { 0 } else { 11 - remainder };
      digit == vec[digit - 1] as usize
    };
    
    validate_digit(13) && validate_digit(14)
  }
  
}
