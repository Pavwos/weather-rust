# Clima

Aplicativo desktop em Rust que mostra o clima atual e a previsão de cinco dias usando a API gratuita Open-Meteo.

## Executar e compilar

```sh
cargo run
cargo build --release
```

O executável é gerado em `target/release/clima.exe` no Windows e `target/release/clima` no Linux/macOS.

No Windows, para criar um atalho, copie o executável para uma pasta fixa (por exemplo, `C:\Apps\Clima`) e clique nele com o botão direito > **Criar atalho**. Mova o atalho para a Área de Trabalho ou para o Menu Iniciar.

Digite o nome da cidade na busca e pressione Enter ou clique em **Buscar**. Use **°C / °F** para alternar unidades; a cidade padrão pode ser salva no próprio app. A previsão é atualizada automaticamente a cada 15 minutos.

## Configuração

O arquivo `config.toml` fica na pasta de configuração do sistema, dentro do diretório `clima`. O app guarda a cidade padrão, a última cidade pesquisada e as unidades.

Os arquivos `assets/NotoEmoji-Regular.ttf` e `assets/icon.png` são opcionais. Quando presentes, a fonte de emoji e o ícone são incorporados ao app.
