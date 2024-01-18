# ReMnK Project

## Overview

ReMnK is an innovative monorepo project that comprises a NextJS frontend, a Tauri application, and a Rust Listener. This project is designed to capture and virtualize mouse and keyboard events as a controller, while providing a sleek user interface for configuring your setup.

## Components

### NextJS Frontend

- **Description**: The NextJS frontend serves as the user interface for the project. It displays the controller data received from the Rust Listener in a clear, visual format. (IN THE FUTURE) It also allows the user to configure their setup by mapping keys and buttons to specific actions.
- **Technologies used**: NextJS, React, Tailwind.

### Tauri Application

- **Description**: The Tauri application acts as a bridge between the frontend and the Rust Listener. It manages the IPC (Inter-Process Communication) and ensures secure and efficient communication between the components.
- **Technologies used**: Tauri, Rust, rdev.

### Rust Listener

- **Description**: The Rust Listener is responsible for capturing mouse and keyboard events at the OS level. It processes these events and sends them to the Tauri application.
- **Technologies used**: Rust.

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/en/download/)
- [Rust](https://www.rust-lang.org/learn/get-started)

Read more about [Tauri](https://tauri.app/v1/guides/getting-started/prerequisites).

### Installation

Clone the repository and navigate to the project directory:

```bash
git clone <repository-url>
cd <repository-name>
```

Install the dependencies for each component:

```bash
# For the Tauri application
cargo install tauri-cli

# For the NextJS frontend
cd nextjs
npm install

# For the Rust Listener
cd mnk
cargo build --release
```

### Running the Project

Run whole project: This will run all components in parallel.

```bash
cargo tauri dev
```

## Usage

Once all components are up and running, the NextJS frontend will visualize the controller data received from the Rust Listener. The Tauri application will manage the communication between the frontend and the Rust Listener, ensuring data integrity and security.

## Development

### Code Style

Please follow the coding standards and style guides provided for each technology (e.g., Prettier and ESLint for JavaScript, `rustfmt` for Rust).

### Testing

- For the NextJS frontend, run `npm run test`.
- For the Tauri application and Rust Listener, run `cargo test`.

### Building and Deployment

Follow the building and deployment instructions specific to each component:

- For the NextJS frontend, refer to NextJS deployment documentation.
- For the Tauri application, use `cargo tauri build`.
- For the Rust Listener, use `cargo build --release`.

## Contributing

Contributions are welcome! Please feel free to submit pull requests, open issues, or suggest improvements.

## License

This project is licensed under the MIT License - see the `LICENSE` file for details.

<!-- ## Acknowledgments

Special thanks to all the contributors and supporters of the ReMnK project. Your efforts and dedication are greatly appreciated. -->
