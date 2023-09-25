fn main() {
    // Cria um vetor vazio
    let mut vec = Vec::new();

    // Preenche o vetor com 2 elementos
    vec.push(1);
    vec.push(2);

    // Testa o tamanho e o conteúdo do vetor
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);

    // Remove o último elemento do vetor
    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);

    // Acessa o primeiro elemento do vetor e o modifica para 7
    vec[0] = 7;
    assert_eq!(vec[0], 7);

    // Adiciona mais elementos ao vetor
    vec.extend([1, 2, 3]);

    // Itera sobre os elementos do vetor
    for x in &vec {
        println!("{x}");
    }

    // Testa o tamanho e o conteúdo do vetor
    assert_eq!(vec, [7, 1, 2, 3]);
}
