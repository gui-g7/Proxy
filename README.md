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

<br><br><br><br><br><br><br>

Vamos detalhar ainda mais a primeira camada (Camada de Entrada - **Listener/Receiver**) para que você tenha uma visão clara dos módulos que deve criar e como organizar os arquivos no seu projeto Cargo. Para isso, vamos quebrar a funcionalidade de captura de pacotes em submódulos menores, que cuidam de diferentes aspectos da captura e gerenciamento das conexões.

### 1. **Divisão da Primeira Camada (Listener/Receiver) em Módulos**
A camada de entrada pode ser dividida em vários subcomponentes/módulos que desempenham tarefas específicas. Vamos criar um diretório para cada módulo, e cada diretório conterá arquivos responsáveis por funções individuais.

#### **Módulos e Arquivos:**
1. **Raw Sockets**: Responsável pela captura de pacotes em nível mais baixo, com suporte a diferentes protocolos.
   - Arquivos:
     - `mod.rs`: Ponto de entrada para o módulo.
     - `tcp.rs`: Implementação da captura de pacotes TCP.
     - `udp.rs`: Implementação da captura de pacotes UDP.
     - `icmp.rs`: Implementação da captura de pacotes ICMP (se necessário).
   
2. **Socket Listener**: Implementa os listeners para diferentes portas e protocolos.
   - Arquivos:
     - `mod.rs`: Ponto de entrada para o módulo.
     - `tcp_listener.rs`: Listener para conexões TCP.
     - `udp_listener.rs`: Listener para conexões UDP.

3. **Gerenciamento de Conexões**: Trata timeouts e a verificação de conexões ativas.
   - Arquivos:
     - `mod.rs`: Ponto de entrada para o módulo.
     - `timeout.rs`: Implementação de timeout para conexões inativas.
     - `connection_manager.rs`: Gerencia o status das conexões ativas.

4. **Log de Pacotes**: Módulo responsável por logar os pacotes recebidos para fins de monitoramento e auditoria.
   - Arquivos:
     - `mod.rs`: Ponto de entrada para o módulo.
     - `packet_logger.rs`: Funções para logar pacotes capturados (pode ser configurado para armazenar em arquivos, banco de dados, etc.).

#### **Estrutura do Diretório Cargo**
Aqui está como o diretório do seu projeto poderia ser organizado com base nesses módulos e arquivos:

```plaintext
proxy_project/
│
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── listener/
│   │   ├── raw_sockets/
│   │   │   ├── mod.rs
│   │   │   ├── tcp.rs
│   │   │   ├── udp.rs
│   │   │   ├── icmp.rs
│   │   ├── socket_listener/
│   │   │   ├── mod.rs
│   │   │   ├── tcp_listener.rs
│   │   │   ├── udp_listener.rs
│   │   ├── connection_management/
│   │   │   ├── mod.rs
│   │   │   ├── timeout.rs
│   │   │   ├── connection_manager.rs
│   │   ├── packet_logging/
│   │       ├── mod.rs
│   │       ├── packet_logger.rs
│   ├── lib.rs
│
```

### 2. **Descrição dos Módulos e Arquivos**

#### **1. Raw Sockets**
Esse módulo será responsável por capturar pacotes em nível bruto, permitindo acessar diretamente os dados da rede. Ele terá suporte para diferentes tipos de pacotes (TCP, UDP, ICMP).

- **Arquivos**:
  - **mod.rs**: Ponto de entrada do módulo. Importa e expõe as funções dos arquivos `tcp.rs`, `udp.rs`, e `icmp.rs`.
  - **tcp.rs**: Funções para capturar pacotes TCP usando raw sockets.
  - **udp.rs**: Funções para capturar pacotes UDP.
  - **icmp.rs**: Captura pacotes ICMP, útil para monitoramento de ping ou outros diagnósticos.

```rust
// src/listener/raw_sockets/mod.rs
pub mod tcp;
pub mod udp;
pub mod icmp;
```

- **tcp.rs**:
```rust
use std::net::{IpAddr, TcpListener, TcpStream};

pub fn capture_tcp_packets(ip: IpAddr, port: u16) {
    let listener = TcpListener::bind((ip, port)).expect("Failed to bind TCP listener");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_tcp_connection(stream),
            Err(e) => eprintln!("Error in TCP connection: {}", e),
        }
    }
}

fn handle_tcp_connection(mut stream: TcpStream) {
    // Manipula os pacotes recebidos via TCP
}
```

#### **2. Socket Listener**
Este módulo gerencia os listeners para TCP e UDP. Ele escuta em portas específicas e delega a captura de pacotes para funções específicas.

- **Arquivos**:
  - **mod.rs**: Ponto de entrada do módulo. Importa `tcp_listener.rs` e `udp_listener.rs`.
  - **tcp_listener.rs**: Funções para escutar conexões TCP em portas específicas.
  - **udp_listener.rs**: Funções para escutar conexões UDP.

```rust
// src/listener/socket_listener/mod.rs
pub mod tcp_listener;
pub mod udp_listener;
```

- **tcp_listener.rs**:
```rust
use std::net::TcpListener;

pub fn start_tcp_listener(addr: &str) {
    let listener = TcpListener::bind(addr).expect("Failed to bind to address");
    println!("Listening for TCP connections on {}", addr);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New TCP connection established");
                // Processar a conexão TCP
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
```

#### **3. Gerenciamento de Conexões**
Esse módulo lida com o gerenciamento de conexões ativas e timeouts. Ele verifica quais conexões estão ativas e cuida do encerramento de conexões inativas.

- **Arquivos**:
  - **mod.rs**: Ponto de entrada do módulo. Importa `timeout.rs` e `connection_manager.rs`.
  - **timeout.rs**: Implementa a lógica de timeout para conexões.
  - **connection_manager.rs**: Mantém o estado das conexões e gerencia seu ciclo de vida.

```rust
// src/listener/connection_management/mod.rs
pub mod timeout;
pub mod connection_manager;
```

- **timeout.rs**:
```rust
use std::time::Duration;

pub fn check_timeout(last_activity: Duration, timeout_limit: Duration) -> bool {
    last_activity > timeout_limit
}
```

#### **4. Log de Pacotes**
Este módulo é responsável por logar os pacotes recebidos, o que ajuda no monitoramento e na auditoria. Ele pode logar pacotes em arquivos ou sistemas de logging mais complexos.

- **Arquivos**:
  - **mod.rs**: Ponto de entrada do módulo. Importa `packet_logger.rs`.
  - **packet_logger.rs**: Implementa as funções de log para os pacotes capturados.

```rust
// src/listener/packet_logging/mod.rs
pub mod packet_logger;
```

- **packet_logger.rs**:
```rust
use std::fs::OpenOptions;
use std::io::Write;

pub fn log_packet(data: &[u8]) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("packet_logs.txt")
        .expect("Unable to open log file");

    writeln!(file, "{:?}", data).expect("Unable to write to log file");
}
```

### 3. **Como fica o Diretório no Projeto Cargo**

Aqui está uma visualização mais detalhada do diretório `src` com base nos módulos e arquivos descritos:

```plaintext
proxy_project/
│
├── Cargo.toml
└── src/
    ├── main.rs             # Arquivo principal que inicializa o proxy.
    │
    ├── listener/           # Diretório para a camada de entrada.
    │   ├── raw_sockets/    # Módulo de captura de pacotes brutos.
    │   │   ├── mod.rs      # Ponto de entrada para o módulo de raw sockets.
    │   │   ├── tcp.rs      # Captura de pacotes TCP.
    │   │   ├── udp.rs      # Captura de pacotes UDP.
    │   │   └── icmp.rs     # Captura de pacotes ICMP.
    │   │
    │   ├── socket_listener/# Módulo de listeners de sockets.
    │   │   ├── mod.rs      # Ponto de entrada para o módulo de listeners.
    │   │   ├── tcp_listener.rs  # Listener para conexões TCP.
    │   │   └── udp_listener.rs  # Listener para conexões UDP.
    │   │
    │   ├── connection_management/ # Módulo de gerenciamento de conexões.
    │   │   ├── mod.rs       # Ponto de entrada para o gerenciamento de conexões.
    │   │   ├── timeout.rs   # Implementação de timeouts para conexões.
    │   │   └── connection_manager.rs  # Gerenciamento do ciclo de vida das conexões.
    │   │
    │   └── packet_logging/  # Módulo de logging de pacotes.
    │       ├── mod.rs       # Ponto de entrada para o módulo de logging.
    │       └── packet_logger.rs  # Funções para log de pacotes.
    │
    └── lib.rs               # Biblioteca geral do projeto.
```

## 4. **Proxymos passos**

- Criar uma DNS própria, para cada pacote os IPs serão pesquisados numa database, caso não encontrados, serão pesquisados numa API e salvos, o IP e de onde é.

- Proteger o proxy com uma senha.

- Criar um reconhecimento de padrões binários, bloqueando malwares e usando códigos menores para representar padrões maiores, semelhante a compressão zip.
