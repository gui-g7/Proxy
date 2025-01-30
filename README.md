<h1>Bem vindo ao meu projeto de proxy de baixo nível, ele será escrito em Rust</h1>

Sua arquitetura é basicamente:

``` bash
+--------------------------------------------------+
|             Logger/Monitoramento                 |
+--------------------------------------------------+
            ↑            ↑             ↑
+-----------+------------+-------------+-----------+
|  Camada de Entrada   →  Filtragem/Análise         |
+--------------------------------------------------+
            ↓                       ↓
   +--------+---------+       +------+---------+
   | Processamento    |       | Processamento  |
   | de Dados Simples |       | Dados Complexos|
   +------------------+       +----------------+
            ↓                       ↓
     +-------------------------------------+
     |        Camada de Saída (Sender)     |
     +-------------------------------------+
```
<br><br><br><hr><br><br><br>
Agora especificações melhores de cada camada e um fracionamento de suas funcionalidades.

### 1. **Camada de Entrada (Listener/Receiver)**
Esta camada será responsável por capturar pacotes da rede. Aqui, você pode dividir a funcionalidade dependendo do tipo de protocolo (TCP/UDP) ou do tipo de dado que você espera receber.

#### Funcionalidades:
- **Captura de Pacotes (Raw Sockets)**: Para analisar dados diretamente da rede, usando raw sockets.
  - **Função**: Captura pacotes em nível baixo, permitindo acessar dados brutos.
  
- **Socket Listener (TCP/UDP)**: Configura listeners para diferentes protocolos e portas.
  - **Função**: Escuta portas específicas (ex.: porta 80 para HTTP) e captura dados quando uma conexão é iniciada.

- **Timeouts e Conexões Ativas**: Gerencia timeouts e verifica quais conexões estão ativas.
  - **Função**: Define tempos de inatividade e gerencia conexões de longo prazo.

#### Exemplos:
- Captura de pacotes TCP:
  ```rust
  // Exemplo básico de captura com raw socket
  let socket = std::net::TcpListener::bind("0.0.0.0:8080").unwrap();
  ```

<br><hr><br>

### 2. **Camada de Filtragem/Análise**
Depois que o pacote é capturado, você precisa determinar como tratá-lo: ele será redirecionado, precisa de análise profunda ou apenas de inspeção básica?

#### Funcionalidades:
- **Identificação de Protocolo**: Identifica se o pacote é HTTP, HTTPS, TCP ou UDP.
  - **Função**: Usar cabeçalhos ou metadados para determinar o tipo de dado e como será processado.

- **Análise de Cabeçalhos**: Filtra cabeçalhos e identifica metadados importantes.
  - **Função**: Examina pacotes de rede, como os cabeçalhos HTTP, e verifica se precisam de modificações.
  
- **Filtragem de Pacotes**: Verifica se o pacote precisa de manipulação profunda (análise byte a byte) ou se pode ser tratado rapidamente.
  - **Função**: Determina o destino do pacote. Se for simples, pode ser redirecionado; se for complexo, precisa de análise detalhada.

- **Regras de Bloqueio/Aceitação**: Define regras de filtragem, bloqueando ou permitindo certos pacotes (como regras de firewall).
  - **Função**: Implementa políticas de segurança, bloqueando IPs, portas, ou padrões de dados específicos.

#### Exemplos:
- Análise de cabeçalhos HTTP:
  ```rust
  // Pseudocódigo para ler cabeçalhos HTTP de um pacote
  if packet_is_http {
      parse_http_headers(packet);
  }
  ```

<br><hr><br>

### 3. **Camada de Processamento de Dados Simples**
Aqui, pacotes que não necessitam de análise byte a byte são processados. É uma camada mais rápida e eficiente.

#### Funcionalidades:
- **Redirecionamento Simples**: Encaminha pacotes de uma origem para um destino sem grandes modificações.
  - **Função**: Redireciona pacotes de um cliente para um servidor ou entre servidores.

- **Modificação de Cabeçalhos**: Altera cabeçalhos de pacotes (ex.: HTTP headers ou TCP).
  - **Função**: Insere ou remove cabeçalhos simples, como adicionar um `X-Forwarded-For` em requisições HTTP.

- **Cache de Pacotes Simples**: Implementa caching básico para evitar reprocessamento de pacotes simples.
  - **Função**: Cacheia pacotes HTTP ou TCP que são recorrentes para reduzir a latência.

#### Exemplos:
- Redirecionamento de pacotes:
  ```rust
  // Exemplo de proxy simples TCP redirecionando uma requisição
  fn redirect_packet(src_addr: &str, dst_addr: &str) {
      let mut src_socket = TcpStream::connect(src_addr).unwrap();
      let mut dst_socket = TcpStream::connect(dst_addr).unwrap();
      std::io::copy(&mut src_socket, &mut dst_socket).unwrap();
  }
  ```

<br><hr><br>

### 4. **Camada de Processamento de Dados Complexos (Byte a Byte)**
Esta é a camada onde a análise detalhada acontece. Você vai lidar com a manipulação de dados em nível mais baixo, analisando e modificando os bytes individuais do pacote.

#### Funcionalidades:
- **Análise Byte a Byte**: Examina cada byte do pacote para entender o conteúdo.
  - **Função**: Analisa e processa dados em nível granular, útil para pacotes criptografados ou compactados.

- **Reestruturação de Pacotes**: Reorganiza ou modifica o conteúdo dos pacotes em nível de byte.
  - **Função**: Altera dados brutos do pacote, como recalcular checksums ou fragmentar pacotes grandes.

- **Fragmentação e Reagrupamento**: Fragmenta pacotes grandes e os reagrupa na ordem correta.
  - **Função**: Lida com pacotes de grande volume ou que são divididos em várias partes.

#### Exemplos:
- Reestruturação de um pacote:
  ```rust
  fn process_raw_packet(packet: &mut [u8]) {
      // Modifica os primeiros bytes (exemplo: cabeçalhos)
      packet[0] = 0x12;
      packet[1] = 0x34;
      // Recalcular checksums ou outros valores críticos
  }
  ```

<br><hr><br>

### 5. **Camada de Saída (Sender/Dispatcher)**
Após o processamento, os pacotes são enviados de volta para o destinatário. Esta camada será responsável por fazer isso de forma eficiente, lidando com retransmissões ou erros.

#### Funcionalidades:
- **Envio de Pacotes Processados**: Transmite pacotes de volta ao destinatário após serem modificados.
  - **Função**: Envia os dados processados para o servidor ou cliente adequado.

- **Gerenciamento de Retransmissões**: Lida com falhas de envio e retransmissões automáticas.
  - **Função**: Garante que os pacotes cheguem ao destino, mesmo em caso de falhas.

- **Compressão/Descompressão de Dados**: Implementa mecanismos de compressão e descompressão de pacotes.
  - **Função**: Comprime pacotes antes do envio para otimizar o tráfego e descomprime pacotes recebidos

<br><br><br><br><br><br><br>

<h1>Check-point</h1>
<p>(Daqui para baixo as coisas estão um pouco incertas.)</p>

<br><br><br><br><br><br><br>

```plaintext
proxy_project/
│
├── Cargo.toml
│
├── db/                                   # Banco de dados.
│   ├── dns.db                            # DNS em SQL.
│   └── package.rdb                       # Dados de uso, armazenados em Redis.
│
└── src/
    ├── main.rs                           # Arquivo principal que inicializa o proxy.
    │
    ├── listener/                         # Diretório para a camada de entrada.
    │   ├── raw_sockets/                  # Módulo de captura de pacotes brutos.
    │   │   ├── mod.rs                    # Ponto de entrada para o módulo de raw sockets.
    │   │   ├── tcp.rs                    # Captura de pacotes TCP.
    │   │   ├── udp.rs                    # Captura de pacotes UDP.
    │   │   └── icmp.rs                   # Captura de pacotes ICMP.
    │   │
    │   ├── socket_listener/              # Módulo de listeners de sockets.
    │   │   ├── mod.rs                    # Ponto de entrada para o módulo de listeners.
    │   │   ├── tcp_listener.rs           # Listener para conexões TCP.
    │   │   └── udp_listener.rs           # Listener para conexões UDP.
    │   │
    │   ├── connection_management/        # Módulo de gerenciamento de conexões.
    │   │   ├── mod.rs                    # Ponto de entrada para o gerenciamento de conexões.
    │   │   ├── timeout.rs                # Implementação de timeouts para conexões.
    │   │   └── connection_manager.rs     # Gerenciamento do ciclo de vida das conexões.
    │   │
    │   ├── packet_logging/               # Módulo de logging de pacotes.
    │   │   ├── mod.rs                    # Ponto de entrada para o módulo de logging.
    │   │   └── packet_logger.rs          # Funções para log de pacotes.
    │   │
    │   ├── mtrics/                       # Módulo de métricas.
    │   │   ├── mod.rs                    # Ponto de entrada para o módulo de métricas.
    │   │   └── traffic_metrics.rs        # Funções para calculo de métricas.
    │   │
    │   └── data/                         # Módulo de conexão com o banco de dados.
    │       ├── mod.rs                    # Ponto de entrada para o módulo de dados.
    │       │.
    │       ├── dns/                      # Módulo de conexão com a DNS em SQL
    │       │   ├── mod.rs                # Ponto de entrada para o módulo da DNS.
    │       │   └── sql.rs                # Funções de acesso a DNS.
    │       │
    │       └── redis                     # Módulo de conexão com o banco de dados dos pacotes
    │           ├── mod.rs                # Ponto de entrada para o módulo do banco de dados.
    │           └── redis.rs              # Funções de acesso ao banco de dados.
    │
    └── lib.rs                            # Biblioteca geral do projeto.
```

## **Atualmente trabalhando em:**

- API para identificação dos dominios dos IPs.

- DNS particular para armazenamento dos dominios e IPs.

- Banco de dados em Redis com informações de todos os pacotes.

- Implementação dos módulos redundantes até agora.

## **Proxymos passos:**

- Proteger o proxy com uma senha.

- Criar um reconhecimento de padrões binários, bloqueando malwares e usando códigos menores para representar padrões maiores, semelhante a compressão zip.

<br><br><hr></hr></br></br>

Resultado por enquanto:

```bash

=== Pacote TCP ===
Endereço IP de Origem: 00.000.00.000
Endereço IP de Destino: 11.1.11.111
Porta de Origem: 432
Porta de Destino: 34562
Número de Sequência: 3154729458
Número de Reconhecimento: 1794639226
Tamanho da Janela: 82
Tamanho do Payload: 32 bytes
=== Pacote UDP ===
Endereço IP de Origem: 2.2.2.2
Endereço IP de Destino: 333.333.333.333
Porta de Origem: 58
Porta de Destino: 67
Tamanho do Payload: 308 bytes


```

(exceto o tamanho dos pacotes, todos os números são ilustrativos)
