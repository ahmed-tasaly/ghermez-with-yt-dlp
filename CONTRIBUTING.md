# Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

- If you have suggestions for adding or removing projects, feel free to [open an issue](https://github.com/IamRezaMousavi/ghermez/issues/new) to discuss it, or directly create a pull request after you edit the _README.md_ file with necessary changes.
- Please make sure you check your spelling and grammar.
- Create individual PR for each suggestion.
- Please also read through the [Code Of Conduct](https://github.com/IamRezaMousavi/ghermez/blob/main/CODE_OF_CONDUCT.md) before posting your first idea as well.

## Roadmap

See the [open issues](https://github.com/IamRezaMousavi/ghermez/issues) for a list of proposed features (and known issues).

## Creating A Pull Request

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Prerequisites

You need to install `git`, `Python` (>=3.7) and `Rust`

### Installation

1. Clone the repo

   ```sh
   git clone https://github.com/<your_username>/<Project-Name>.git
   ```

2. Create a virtual environment and activate it

   ```sh
   python -m venv venv
   . ./venv/bin/activate
   ```

3. Install requirement packages

   ```sh
   pip install PyQt5 requests maturin patchelf
   ```

4. Build `ghermez` package

   ```sh
   maturin develop
   ```

5. Run the app from test dir

   ```sh
   python test/test.py
   ```
