/// Trunca um valor pelo **tamanho**, caso contrário retorna o valor **intacto**.
///
/// # Argumentos
///
/// * `value` - Uma referência à uma String.
/// * `size` - O tamanho limte
///
/// # Retorno
///
/// * A String modificada

pub fn trunc<T: ToString>(value: &Option<T>, size: usize) -> Option<String> {
    if let Some(s) = value {
        let mut temp = s.to_string().clone();
        temp.truncate(size);
        Some(temp)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_chars() {
        let s = String::from("Avelino");
        assert_eq!("Ave", trunc(&Some(s), 3).unwrap());
    }
}
