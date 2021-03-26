#[derive(Debug, Clone, PartialEq)]
pub struct Node {
  pub character: String,
  pub destination: String,
}
pub type NodeVec = Vec<Node>;

pub mod afd {

  use crate::util::{file::ParsedFile, menu};

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
      "\n\t{} [color=\"#467050\", fontcolor=\"white\"]\n",
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
  pub fn run(infos: &ParsedFile) -> bool {
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
}

pub mod afn {
  use crate::file::ParsedFile;
  use crate::util::menu;

  pub const LAMBDA: &str = "/";

  #[inline]
  fn pad(n: usize) -> String {
    "\t".repeat(n)
  }

  /// Exporta o hashmap para um arquivo dot equivalente.
  /// Todos os nós terão configuração padrão, com excessão do principal.
  ///
  /// ## Arguments
  ///
  /// * `infos` - Arquivo parsed, contendo as informações
  /// * `node_to_color` - Key do nó que será colorido (atual).
  /// * `edge_to_color` - Posição da aresta que será colorida no vetor interno de cada key de infos.
  /// * `filename` - Nome e local de onde salvar os arquivos dot.
  ///
  /// ## Examples
  ///
  /// ```rust
  /// save_dot_file(&hashmap, &"s0", &"0");
  /// ```
  fn save_dot_file(
    infos: &ParsedFile,
    node_to_color: &String,
    edge_to_color: usize,
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
      "\n\t{} [color=\"#467050\", fontcolor=\"white\"]\n",
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

  fn afn_walking(
    word: &Vec<String>,
    curr_pos: usize,
    current_state: &String,
    infos: &ParsedFile,
    p: Option<super::NodeVec>,
  ) -> (bool, Option<super::NodeVec>) {
    debug!("{}  Word index: {}", self::pad(curr_pos), curr_pos);
    debug!(
      "{}> CURRENT STATE: #{}",
      self::pad(curr_pos),
      &current_state
    );
    let mut path = p.clone();

    // verifica se é uma folha (se estourou o máximo de char perdido pela palavra)
    if curr_pos == word.len() {
      // então verifica se atingiu um estado final
      if infos.get_finish_state().contains(&current_state) {
        return (true, Some(path.unwrap()));
      } else {
        // caso contrário, não aceita a palavra
        debug!(
          "{}~~~~Overflow, back one leaf.`t'~~~~\n",
          self::pad(curr_pos)
        );
        return (false, None);
      }
    }

    // obtêm todos os estados alcançáveis a partir do estado atual
    let possible = infos.states.get(current_state).unwrap();

    debug!("{}++ Walking over node possibilities", self::pad(curr_pos),);
    for p in possible {
      if p.character == LAMBDA {
        debug!(
          "\t{}-- State: {} => Matched {}",
          self::pad(curr_pos),
          &current_state,
          &p.character
        );
        let next_state = p.destination.to_string();
        debug!("\t{}-- Next state: {}", self::pad(curr_pos), &next_state);

        // adiciona o próximo estado no path
        let next = super::Node {
          character: p.character.to_string(),
          // aqui, destination na verdade trata-se do nó atual
          destination: next_state.to_string(),
        };
        let mut pt: super::NodeVec = path.unwrap();
        pt.push(next);

        path = Some(pt);

        // muda para o próximo estado
        let (found, _) = self::afn_walking(word, curr_pos, &next_state, infos, path.clone());
        debug!("{}Backed: Curr_pos {:#?}", self::pad(curr_pos), curr_pos);
        if found {
          debug!("\t{}-- *FOUND* the word. It matches", self::pad(curr_pos),);
          return (true, path);
        }
      } else if word[curr_pos] == p.character {
        // consome a letra
        debug!(
          "\t{}-- State: {} => Matched {}",
          self::pad(curr_pos),
          &current_state,
          &p.character
        );
        let next_state = p.destination.to_string();
        debug!("\t{}-- Next state: {}", self::pad(curr_pos), &next_state);

        // adiciona o próximo estado no path
        let next = super::Node {
          character: p.character.to_string(),
          // aqui, destination na verdade trata-se do nó atual
          destination: next_state.to_string(),
        };
        let mut pt: super::NodeVec = path.unwrap();
        pt.push(next);

        path = Some(pt);

        // muda para o próximo estado
        let (found, all_path) =
          self::afn_walking(word, curr_pos + 1, &next_state, infos, path.clone());
        debug!("{}Backed: Curr_pos {:#?}", self::pad(curr_pos), curr_pos);
        if found {
          debug!("\t{}-- *FOUND* the word. It matches", self::pad(curr_pos),);
          return (true, all_path);
        } else {
          // como adicionou antes de decer, é necessário remover antes de continuar
          let mut pt = path.unwrap();
          pt.pop();
          path = Some(pt);
        }
      }
    }
    return (false, None);
  }

  /// Retorna um possível caminho que leva à resposta
  ///
  /// ## Arguments
  ///
  /// * `infos` - As informações com base no arquivo de entrada
  ///
  /// ## Examples
  ///
  /// ```rust
  /// let o = run_afn(&infos);
  /// ```
  pub fn run(infos: &ParsedFile) -> bool {
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
    let mut current_state: String = infos.get_initial_state().to_string();

    debug!("Start walking");
    let (worked, walked_path) =
      self::afn_walking(&word, pos, &current_state, infos, Some(Vec::new()));

    debug!(" @ AFN -- Worked?: {}\t PATH: {:?}", &worked, &walked_path);
    if !worked {
      // force exit
      return false;
    }

    // anda sobre o vetor de saída (caminho)
    for node in walked_path.unwrap() {
      // exibe menu
      loop {
        debug!("\t - Showing menu");
        let option: u8 = menu(word[0..pos].join(""));
        if option == 1 {
          debug!("Menu called");
          // obtém os estados alcançáveis
          let possible_states = infos.states.get(&current_state).unwrap();
          debug!("Possible states {:#?}", &possible_states);

          // obtém a posição da aresta que bate com o node
          debug!("Looping");
          let mut edge_to_color = 0;
          loop {
            debug!(
              "Comparing if {:?} is equal to {:?}",
              &node, possible_states[edge_to_color]
            );
            if node == possible_states[edge_to_color] {
              break;
            }
            edge_to_color += 1;
          }

          debug!("+ Creating dot file");
          self::save_dot_file(
            infos,
            &current_state,
            edge_to_color,
            &format!("./dot/dotfile_{}.dot", pos),
          );

          current_state = node.destination.clone();
          // exit this loop
          pos += 1;
          break;
        } else if option == 0 {
          debug!("Menu called");
          pos += 1;
          break;
        }
      }
    }

    return worked;
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn nodes_are_equal() {
    let a = super::Node {
      character: "b".to_string(),
      destination: "b".to_string(),
    };
    let b = super::Node {
      character: "b".to_string(),
      destination: "b".to_string(),
    };
    assert_eq!(a, b, "Nodes aren't equals");
  }

  #[test]
  fn nodes_are_different() {
    let a = super::Node {
      character: "b".to_string(),
      destination: "b".to_string(),
    };
    let b = super::Node {
      character: "c".to_string(),
      destination: "b".to_string(),
    };
    assert_ne!(a, b, "Nodes are equals");
  }
}
