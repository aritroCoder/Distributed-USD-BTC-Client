
# Distributed USD/BTC Client

This is Rust based aggregator and client that collects real time price of BTC in USD from the binance websocket API using multiple parallel clients, and computes the average to show in the terminal.


## Run Locally

Clone the project

```bash
  git clone https://github.com/aritroCoder/Distributed-USD-BTC-Client
```

Go to the project directory

```bash
  cd Distributed-USD-BTC-Client
```

Run with a single command (make sure Rust is installed and device port 8080 is free)

```bash
bash run.sh
```

## Working

This project as a parent-child process structure where the parent is the aggregator, and child is the client. The aggregator, when run spawns five instances of client processes, which are synchronized to start at the same tick of clock(upto microsecond accuracy). The client processes collect data from the binance socket independently 10 times, computes average, and sends the data to the server using a TCP stream opened at port 8080. Server, upon recieving the data, computes average of all the averages and displays on the terminal.
