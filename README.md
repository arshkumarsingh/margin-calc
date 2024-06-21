

# Margin Calc

Margin Calc is a Rust-based application that provides a graphical user interface (GUI) for fetching and displaying margin information for options trading using the Kite Trade API.

## Features

- Fetch and display option symbols.
- Retrieve and display margin information for selected option symbols.
- Automatic refresh of symbols every 60 seconds.
- Error handling and display for API requests.

## Requirements

- Rust (latest stable version)
- Cargo (latest stable version)
- Kite Trade API credentials (API key and access token)

## Installation

1. **Clone the repository:**
   ```sh
   git clone https://github.com/arshkumarsingh/margin-calc
   cd margin-calc
   ```

2. **Build the project:**
   ```sh
   cargo build
   ```

## Usage

1. **Run the application:**
   ```sh
   cargo run
   ```

2. **Enter your Kite Trade API credentials:**
   - API Key
   - Access Token

3. **Refresh symbols and fetch margin information:**
   - Click "Refresh Symbols" to load available option symbols.
   - Select a symbol from the dropdown.
   - Click "Get Margin" to fetch and display margin information for the selected symbol.

## Project Structure

- **src/api.rs**: Contains functions for making API requests to the Kite Trade API.
- **src/gui.rs**: Implements the GUI using `eframe` and `egui` for interaction with the user.
- **src/main.rs**: Entry point for the application, setting up and running the GUI.

## Dependencies

- `reqwest`: For making HTTP requests.
- `serde`: For deserializing JSON responses.
- `eframe` and `egui`: For building the graphical user interface.
- `tokio`: For asynchronous runtime.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any changes.

## License

This project is licensed under the MIT License.

## Contact

For any questions or issues, please open an issue on the [GitHub repository](https://github.com/yourusername/margin-calc/issues).

---
