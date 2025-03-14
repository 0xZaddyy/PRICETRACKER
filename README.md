# Bitcoin Price Tracker

A simple Rust application that fetches and displays the latest Bitcoin price in USD using the CoinGecko API.

## 🚀 Features
- Fetches the latest Bitcoin price every 10 seconds
- Uses asynchronous requests with `reqwest`
- Parses JSON responses with `serde`
- Lightweight and efficient

## 🛠️ Technologies Used
- Rust
- Reqwest (for HTTP requests)
- Serde (for JSON parsing)
- Tokio (for async runtime)

## 📦 Installation
1. **Clone the repository**
   ```sh
   git clone https://github.com/0xZaddyy/PRICETRACKER.git
   cd PRICETRACKER
   ```

2. **Ensure Rust is installed**
   Install Rust via [rustup](https://rustup.rs/):
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **Build the project**
   ```sh
   cargo build
   ```

4. **Run the tracker**
   ```sh
   cargo run
   ```

## 📡 API Source
This app uses the [CoinGecko API](https://www.coingecko.com/en/api/documentation) to fetch Bitcoin price data.

## ⚠️ Disclaimer
This project is for educational purposes only. Do not use it for financial decisions.

## 📜 License
This project is open-source and available under the MIT License.

