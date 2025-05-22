# rust-simple-counter-cli

A simple Rust command-line application that starts counting from a specified number and prints the incremented value at regular intervals.

---

## Features

* **Custom Start**: Begin counting from any integer value.
* **Adjustable Interval**: Pause between increments with a user-defined sleep duration (in seconds).
* **Lightweight**: Minimal dependencies, using [`clap`](https://crates.io/crates/clap) for argument parsing.

---

## Prerequisites

* [Rust](https://www.rust-lang.org/tools/install) (1.56 or later)
* Cargo (comes bundled with Rust)

---

## Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/your-username/rusty-counter.git
   cd rusty-counter
   ```
2. Build the executable:

   ```sh
   cargo build --release
   ```
3. (Optional) Install globally:

   ```sh
   cargo install --path .
   ```

After installation, the `rusty-counter` binary will be available in your `PATH`.

---

## Usage

```sh
rusty-counter --count <START> [--sleep <SECONDS>]
```

* `--count`, `-c` **(required)**: The starting integer value for the counter.
* `--sleep`, `-s` **(optional)**: Number of seconds to wait between increments. Defaults to `1` second.

### Examples

1. **Count from 5 with default sleep (1s):**

   ```sh
   rusty-counter -c 5
   ```

   **Output:**

   ```
   Counter starts from 5
   counter => 5
   counter => 6
   counter => 7
   ...
   ```

2. **Count from 10 with a 2-second interval:**

   ```sh
   rusty-counter --count 10 --sleep 2
   ```

   **Output:**

   ```
   Counter starts from 10
   counter => 10
   counter => 11
   # (waits 2 seconds)
   counter => 12
   ...
   ```

### Error Handling

If the provided arguments cannot be parsed as integers, the app will print an error message and exit with code `1`:

```sh
rusty-counter -c not_a_number
# Error parsing argument, Please enter a valid number!
```

---

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/YourFeature`)
3. Commit your changes (`git commit -m "Add some feature"`)
4. Push to the branch (`git push origin feature/YourFeature`)
5. Open a Pull Request

---

## License

This project is licensed under the [MIT License](LICENSE).
