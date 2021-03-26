#!/bin/bash

cd dot;

# Gera um arquivo de imagem para cada arquivo dot
printf "\e\033[1;34m Gerando os arquivos de imagem...\n\e\033[0m"

for arquivo in *.dot; do
  echo " - Arquivo: ${arquivo}"
  line="dot  -Tjpeg ${arquivo} -o ${arquivo%.dot}.jpg"
  $line
done

printf "\e\033[1;34m Gerando o arquivo gif...\n\e\033[0m"
convert -delay 60 -loop 0 *.jpg output.gif

printf "\e\033[1;34m Arquivo gerado. Fim!\n\e\033[0m"
cd ..