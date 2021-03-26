pub mod file {
  use crate::afdn::{Node, NodeVec, LAMBDA};
  use std::{collections::HashMap, fs};

  /// # ParsedFile
  /// Struct usada para segurar todas as informações do arquivo de entrada (centraliza)
  #[derive(Debug)]
  pub struct ParsedFile {
    initial_states: String,
    finish_states: Vec<String>,
    pub states: HashMap<String, NodeVec>,
    word: String,
    is_afd: bool,
  }
  impl ParsedFile {
    pub fn get_initial_state(&self) -> &String {
      &self.initial_states
    }
    pub fn get_finish_state(&self) -> &Vec<String> {
      &self.finish_states
    }
    pub fn get_word(&self) -> &String {
      &self.word
    }
    pub fn is_afd(&self) -> &bool {
      &self.is_afd
    }
  }

  /// Faz o parse de um arquivo de entrada.
  ///
  /// <pre>
  /// Exemplo de um arquivo de entrada (note que pode haver transição lambda):
  ///
  /// s0 ; s2
  /// s0 a > s0
  /// s0 b > s1
  /// s1 a > s1
  /// s1 b > s2
  /// s2 a > s2
  /// s2 b > s2
  /// wrd : aabb
  /// </pre>
  ///
  /// ## Arguments
  ///
  /// * `filepath` - Nome relativo do arquivo partindo do diretório raíz.
  ///
  /// ## Examples
  ///
  /// ```rust
  /// let infos = parse("./input.txt");
  /// ```
  pub fn parse(filepath: &String) -> ParsedFile {
    debug!("Parsing file...");

    let parsed_file: String =
      fs::read_to_string(filepath).expect("Houve um problema ao ler o arquivo");
    debug!("File readed");

    // separa em linhas
    let file_in_lines: Vec<&str> = parsed_file.split("\n").collect();
    debug!("File splitted into lines: {:#?}", &file_in_lines);

    // estados iniciais (até N estados, separados por N*espaço) ; estados finais (até n estados, j*espaço)
    let initial_end_states: Vec<&str> = file_in_lines[0].split(";").collect();
    let initial_states: Vec<&str> = initial_end_states[0]
      .split_whitespace()
      .collect::<Vec<&str>>();
    let end_states: Vec<String> = initial_end_states[1]
      .split_whitespace()
      .collect::<Vec<&str>>()
      .iter()
      .map(|e| e.to_string())
      .collect();
    debug!(
      "Splitted initial {:#?} and final states {:#?}",
      &initial_states, &end_states
    );

    // última linha será a palavra de teste
    let max = file_in_lines.len() - 1;
    debug!("The file has {} states", max - 1);

    // obtém os estados e suas transições
    debug!("Creating the hashmap");
    let mut is_afdn = false;
    let mut afdn: HashMap<String, NodeVec> = HashMap::new();
    debug!("Iterating over states:");
    for line_index in 1..max {
      let line: Vec<&str> = file_in_lines[line_index].split_whitespace().collect();
      debug!("\t#{} Line -> {:#?}", line_index, &line);

      let current_node = line[0].to_string();
      debug!("\t - Current node: {}", &current_node);

      let character: String = line[1].to_string();
      let destination: String = line.last().unwrap().to_string();
      debug!("\t - {} -> {}", &character, &destination);

      let new_node = Node {
        character: String::from(&character),
        destination,
      };

      debug!("\t - Check if exist this key ({}) in Hash", &current_node);
      if afdn.contains_key(&current_node) {
        debug!("\t - Already exists. Adding new node");
        let mut cvec: NodeVec = afdn.get(&current_node).unwrap().to_vec();

        // checa se é um AFN (símbolo Lambda ou repetidos)
        if character != LAMBDA.to_string() {
          cvec.insert(0, new_node);

          // precisa checar se já é AFDN, para evitar sobrescrever a flag
          if !is_afdn {
            debug!("\t - Checking if is AFDN...");
            // se não for afdn ainda, faz um map e confere se tem chave duplicada.
            let repeated: Vec<String> = cvec.iter().map(|e| e.character.to_string()).collect();
            debug!("\t - Repeated_vector {:#?}", &repeated);
            is_afdn = repeated
              .iter()
              .enumerate()
              // analisa do ponto atual para frente, i.e, se no futuro irá existir um elemento igual/repetido
              .any(|(index, word)| repeated[index + 1..].contains(&word));

            debug!("\t - Is it an AFDN? R.: {:?}", is_afdn);
          }
        } else {
          // se for lambda, insere no fim e muda para um AFD
          is_afdn = true;
          cvec.push(new_node);
        }

        debug!("\t - Updated nodes: {:?}", cvec);
        afdn.insert(current_node, cvec);
      } else {
        debug!("\t - Key didn't exists. Adding into HashMap");
        afdn.insert(current_node, vec![new_node]);
      }
    }

    // obtém a palavra a ser testada
    let word: Vec<&str> = file_in_lines.last().unwrap().trim().split(":").collect();
    let word: String = word.last().unwrap().trim().to_string();

    debug!("Initial states: {:#?}", &initial_states);
    debug!("Finish states:  {:#?}", &end_states);
    debug!("Word:           {:#?}", &word);
    debug!("HashMap:        {:#?}", &afdn);

    // return three things
    ParsedFile {
      // estou assumindo que só é possível ter um único estado inicial
      initial_states: initial_states[0].to_string(),
      finish_states: end_states,
      word,
      states: afdn,
      is_afd: !is_afdn,
    }
  }
}
