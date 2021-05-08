<h1>Bloco de notas desenvolvido com Rust e GTK</h1>
<h3>O Glade 3.38.2 foi usado para a construção da interface</h3>

<h2>Requisitos:</h2>
<ul>
  <li>Ter instalada a <strong>libgtk-3-dev<strong> na distribuição linux</li>
  <h3>Como dependências do projeto:</h3>
  <ul>
    <li>gtk = "0.9.2"</li>
    <li>gio = "0.9.1"</li>
  </ul>
</ul>

<h3>Para testar rodar no terminal: <strong>Cargo run</strong></h3>

<h3>Funcionalidades prontas<h3>
<ul>
  <li>
    Abre qualquer documento de texto do disco
  </li>
  <li>
    Salva o texto digitado no textView em documento .txt em disco
  </li>
</ul>

<h3>Melhorias e implementações necessárias<h3>
<ul>
  <li>
    Implementar salvar como
  </li>
  <li>
    Implementar copiar e colar
  </li>
  <li>
    Implementar recortar
  </li>
  <li>
    Implementar refazer e desfazer
  </li>
  <li>
    Implementar fechar documento solicitando se deseja salvar ou descartar modificações
  </li>
  <li>
    Implementar criar novo documento
  </li>
</ul>

- Refatorar o código, principalmente para resolver a clonagem do Widget text_view na linha 23 e 24, apesar de funcional é menos performático termos que clonar algum Widget. Problema encontrado: O mesmo widget text_view precisa ser manipulado em duas funções distintas handler_open_file e handler_save_file chamadas em:

  ```
    button_save.connect_clicked(move |_| {
        handler_save_file(&text_view);
    });

    button_open.connect_clicked(move |_elem| {
        let dialog_file_chooser = handler_open_file(&window, &ref_to_text_view);
    });

  ```

  Precisa-se evitar o sequestro de ownership em função Closeru! solução temporária clonagem do widget na linha 23 e 24

  ```
  let text_view: TextView = builder.get_object("text_area").unwrap();
  let ref_to_text_view: TextView = text_view.clone(); //Encontrar solução para não clonar um Widget

  ```

<h3>Este Bloco de notas é um projeto é experimental em Rust, aos interessados em contribuir com melhorias, pull request serão bem vindas</h3>

![screen 1](/screenshot/screen.png)
