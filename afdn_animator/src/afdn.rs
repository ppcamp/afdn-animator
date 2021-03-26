use crate::file::ParsedFile;

pub const LAMBDA: &str = "/";

#[derive(Debug, Clone)]
pub struct Node {
  pub character: String,
  pub destination: String,
}
pub type NodeVec = Vec<Node>;
use std::io;

/// Exibe o menu de escolha (para salvar o arquivo dot)
///
/// ## Arguments
/// - `word` - Palavra até agora
///
/// ## Examples
///
/// ```rust
/// let option:u8 = menu(); // returns 0 or 1 (save)
/// ```
fn menu(word: String) -> u8 {
  debug!("Show menu");
  let line = "-".repeat(100);
  println!("{}\n\t\t\t\t\tAnimador de AFN/AFD\n{}", &line, &line);
  println!("Deseja gerar um arquivo do estado atual? [0/1] \n*Caso 1 , irá gerar um arquivo do estado atual.\n");
  println!("Palavra até agora: {}", word);

  debug!("Reading input");
  let mut buffer = String::new();
  io::stdin()
    .read_line(&mut buffer)
    .expect("Não foi possível reconhecer o caracter de entrada 😔. Saindo...");

  debug!("Parsing input: '{}'", &buffer);
  let input = match buffer.trim().parse::<u8>() {
    Ok(val) => val,
    Err(err) => panic!(
      "Houve um problema ao realizar o parse para int. Erro: {}",
      err
    ),
  };

  debug!("Returning values");
  input
}

/// Exporta o hashmap para um arquivo dot equivalente.
/// Todos os nós terão configuração padrão, com excessão do principal.
///
/// ## Arguments
///
/// * `hashmap` - Um hashmap contendo o grafo.
/// * `node_to_color` - Key do nó que será colorido (atual).
/// * `edge_to_color` - Aresta que será colorida (indica o nó futuro).
///
/// ## Examples
///
/// ```rust
/// save_dot_file(&hashmap, &"s0", &"0");
/// ```
fn save_dot_file(
  infos: &ParsedFile,
  node_to_color: &String,
  edge_to_color: &String,
  filename: &String,
) {
  debug!("#Graph Saving graph in file.");

  let mut output = String::new();

  // cria o cabeçalho
  output.push_str("digraph G {\n");
  output.push_str(
    "\trankdir=LR;\n\toverlap=scale;\n\tsep=0.1;\n\tpad=1;\n\tnodesep=0.5;\n\tranksep=1;\n\n",
  );
  output.push_str("\tnode [style=\"rounded,filled\"]\n");
  output.push_str("\tedge [color=\"gray\"]\n\n");
  debug!("#Graph Header saved");

  // configura os nós terminais
  for node in infos.get_finish_state() {
    let mut terminal: String = "\t".to_string() + node;
    terminal.push_str(" [peripheries=2]\n");
    output.push_str(&terminal);
  }
  debug!("#Graph Setup terminal nodes");

  // configura a "seta" do nó inicial apontado para o primeiro nó
  output.push_str("\n\tstart [label= \"\", shape=none,height=.0,width=.0]\n");
  let initial: String = "\tstart -> ".to_string() + infos.get_initial_state() + "\n";
  output.push_str(&initial);
  debug!("#Graph Setup start node");

  // configura o nó atual (cor)
  output.push_str(&format!(
    "\n\t{} [color=\"#467050\", fontcolor=\"white\"]",
    node_to_color
  ));
  debug!("#Graph Setup current_node: {}", &node_to_color);

  // insere as arestas (e nós)
  debug!("#Graph - Walking in keys... {:#?}", infos.states.keys());
  for origin in infos.states.keys() {
    debug!("#Graph -\t CurrentNode: {}", origin);

    let nodes = infos.states.get(origin).unwrap();
    debug!("#Graph -\t ChildrenNodes: {:#?}", &nodes);

    for node in nodes {
      // configura a cor e label das arestas
      let mut additional_configs: String = format!("[label=\"{}\"", &node.character);

      if (origin == node_to_color) && (node.character == edge_to_color.to_string()) {
        additional_configs = format!("{}, color=\"#ad2a2a\"", &additional_configs).to_string();
      }
      additional_configs = format!("{}]", &additional_configs);

      output.push_str(&format!(
        "\t{} -> {} {}\n",
        &origin, &node.destination, &additional_configs
      ));
      debug!("#Graph -\t Settled")
    }
  }

  output.push('}');

  // salva em arquivo
  match std::fs::write(filename, output) {
    Ok(_) => debug!("Saved file {}", filename),
    Err(err) => panic!("{}", err),
  }
}

/// Anda sobre o afd.
/// Considerações:
/// - Assume-se que somente haverá **um único** estado inicial.
///
/// ## Arguments
///
/// * `infos` - O arquivo de entrada lido e mapeado.
///
/// ## Examples
///
/// ```rust
/// let resultado:bool = run(&infos); // true or false
/// ```
pub fn run_afd(infos: &ParsedFile) -> bool {
  debug!("Running afd...");

  let word: Vec<String> = infos
    .get_word()
    .chars()
    .collect::<Vec<char>>()
    .iter()
    .map(|e| e.to_string())
    .collect();
  debug!("Splitted the word into vec of chars");

  let mut pos: usize = 0;
  let mut flag: bool;
  let mut current_state: String = infos.get_initial_state().to_string();

  debug!("Start walking");
  while pos < word.len() {
    debug!("\n\t [Word {} is valid]", word[0..pos].join(""));
    // obtêm todos os estados alcançáveis
    let possible = infos.states.get(&current_state).unwrap();

    let mut node_to_color = String::new();
    let mut edge_to_color = String::new();

    // flag de aceito
    flag = false;
    debug!("\t - Walking over all possibilities");
    for p in possible {
      if word[pos] == p.character {
        // aceita, continua a busca pela palavra
        flag = true;
        // consome a letra
        pos += 1;

        debug!(
          "\t -- State: {} => Matched {}",
          &current_state, &p.character
        );

        // altera o valor dos locais para colorir
        node_to_color = current_state.clone();
        edge_to_color = p.character.clone();

        // muda para o próximo estado
        let next_state = p.destination.to_string();
        debug!("\t -- Next state: {}", &next_state);
        current_state = next_state;
        // sai deste for
        break;
      }
    }
    // ocorreu um erro durante o percurso do afd/afn
    if !flag {
      eprintln!(
        "Palavra {} inválida. Válida até {}. Char de conflito {}",
        infos.get_word(),
        word[0..pos].join(""),
        word[pos]
      );
      return false;
    }

    // exibe menu
    loop {
      debug!("\t - Showing menu");
      let option: u8 = menu(word[0..pos].join(""));
      if option == 1 {
        self::save_dot_file(
          infos,
          &node_to_color,
          &edge_to_color,
          &format!("./dot/dotfile_{}.dot", pos),
        );
        // exit this loop
        break;
      } else if option == 0 {
        break;
      }
    }
  }

  // se atingiu um estado final
  if infos.get_finish_state().contains(&current_state) {
    return true;
  } else {
    // caso contrário, não aceita a palavra
    return false;
  }
}

pub fn run_afn(infos: &ParsedFile) -> bool {
  debug!("Running AFN...");

  let word: Vec<String> = infos
    .get_word()
    .chars()
    .collect::<Vec<char>>()
    .iter()
    .map(|e| e.to_string())
    .collect();
  debug!("Splitted the word into vec of chars");

  let mut pos: usize = 0;
  let mut flag: bool;
  let mut current_state: String = infos.get_initial_state().to_string();

  debug!("Start walking");
  while pos < word.len() {
    debug!("\n\t [Word {} is valid]", word[0..pos].join(""));
    // obtêm todos os estados alcançáveis
    let possible = infos.states.get(&current_state).unwrap();

    let mut node_to_color = String::new();
    let mut edge_to_color = String::new();

    // flag de aceito
    flag = false;
    debug!("\t - Walking over all possibilities");
    for p in possible {
      if word[pos] == p.character {
        // aceita, continua a busca pela palavra
        flag = true;
        // consome a letra
        pos += 1;

        debug!(
          "\t -- State: {} => Matched {}",
          &current_state, &p.character
        );

        // altera o valor dos locais para colorir
        node_to_color = current_state.clone();
        edge_to_color = p.character.clone();

        // muda para o próximo estado
        let next_state = p.destination.to_string();
        debug!("\t -- Next state: {}", &next_state);
        current_state = next_state;
        // sai deste for
        break;
      }
    }
    // ocorreu um erro durante o percurso do afd/afn
    if !flag {
      eprintln!(
        "Palavra {} inválida. Válida até {}. Char de conflito {}",
        infos.get_word(),
        word[0..pos].join(""),
        word[pos]
      );
      return false;
    }

    // exibe menu
    loop {
      debug!("\t - Showing menu");
      let option: u8 = menu(word[0..pos].join(""));
      if option == 1 {
        self::save_dot_file(
          infos,
          &node_to_color,
          &edge_to_color,
          &format!("./dot/dotfile_{}.dot", pos),
        );
        // exit this loop
        break;
      } else if option == 0 {
        break;
      }
    }
  }

  // se atingiu um estado final
  if infos.get_finish_state().contains(&current_state) {
    return true;
  } else {
    // caso contrário, não aceita a palavra
    return false;
  }
}
