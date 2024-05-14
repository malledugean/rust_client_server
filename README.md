<p align="center">
  <a href="" rel="noopener">
 <img src="https://i.imgur.com/On9HPQN.jpeg" alt="Project Client Server in Rust logo" width="300" height="300"></a>
</p>

<h3 align="center">Project Client Server in Rust</h3>

<div align="center">

[![Status](https://img.shields.io/badge/status-active-success.svg)]()
[![GitHub Issues](https://img.shields.io/badge/issues-0%20open-red)](https://github.com/malledugean/rust_safe_math/issues)
[![GitHub Pull Requests](https://img.shields.io/badge/pull%20requests-0%20pull-yellow)](https://github.com/malledugean/rust_safe_math/pulls)
[![License](https://img.shields.io/badge/license-CC--BY--4.0-blue) ](#license)

</div>

---

<p align="center"> A Rust library for simplified safe math operations.
    <br> 
</p>

## 📝 Table of Contents

-   [About](#about)
-   [Getting Started](#getting_started)
-   [Deployment](#deployment)
-   [Usage](#usage)
-   [Built Using](#built_using)
-   [License](#license)
-   [Authors](#authors)
-   [Acknowledgments](#acknowledgement)

## 🧐 About <a name = "about"></a>

This project provides a basic sample application demonstrating client-server communication in Rust. It includes a simple server program and a corresponding client program that can send and receive messages.

## 🏁 Getting Started <a name = "getting_started"></a>

This guide will help you set up and run the client-server sample application.

### Prerequisites

-   Rust compiler: Download and install Rust from the official website https://www.rust-lang.org/tools/install.
-   Basic understanding of Rust: Familiarity with core Rust concepts like functions, variables, and data types is recommended.

### Installing

1. Clone this repository:

```
Bash
git clone https://your_github_url/client-server.git
```

Use code with caution.

2. Navigate to the project directory:

```
cd client-server
```

Run 'cargo build' to compile both the client and server programs.

## 🔧 Running the tests <a name = "tests"></a>

The project includes unit tests for both the client and server code. To run the tests, execute:

```
Bash
cargo test
```

## 🎈 Usage <a name="usage"></a>

1. Run the server:

Open a terminal window and navigate to the project directory. Run the following command to start the server:

```
Bash
cargo run --bin server
```

This will start the server program, which will listen for incoming connections on a specific port (default: 8080).

2. Run the client:

Open another terminal window and navigate to the project directory. Run the following command to start the client program:

```
Bash
cargo run --bin client
```

This will start the client program, which will attempt to connect to the server running on the specified port. You can then type messages in the client terminal, and they will be sent to the server and displayed on the server console.

## 🚀 Deployment <a name = "deployment"></a>

This is a sample application intended for educational purposes. For production deployments, consider using more robust frameworks and libraries designed for client-server communication in Rust.

## ⛏️ Built Using <a name = "built_using"></a>

-   [Rust](https://www.rust-lang.org) - Rust

## 📜 License: <a name = "license"></a>

This project is licensed under the license Creative Commons Attribution 4.0 International (CC-BY-4.0).

## ✍️ Authors <a name = "authors"></a>

-   [@malledugean](https://github.com/malledugean) - Idea & Initial work

## 🎉 Acknowledgements <a name = "acknowledgement"></a>

-   NearX Rust learning
