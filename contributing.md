# Contributing

You need to install `Python` (>=3.7) and `Rust`

then follow this steps:

1. Create a virtual environment and activate it

   ```sh
   python -m venv venv
   . ./venv/bin/activate
   ```

2. Install requirement packages

   ```sh
   pip install PyQt5 requests maturin patchelf
   ```

3. Build `ghermez` package

   ```sh
   maturin develop
   ```

4. Run the app from test dir

   ```sh
   python test/test.py
   ```
