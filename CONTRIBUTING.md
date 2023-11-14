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

### Develop

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
   pip install PyQt5 maturin requests setproctitle psutil youtube_dl
   ```

   (only for windows)

   ```sh
   pip install pypiwin32
   ```

4. Build `ghermez` package

   ```sh
   maturin develop
   ```

5. Run the app from test dir

   ```sh
   python test/test.py
   ```

## Build

### step 1: Preparing

#### 1-1 clone or download [Ghermez](https://github.com/IamRezaMousavi/ghermez)

You can download project form our github page or using git clients. I recommend [git for windows](https://git-scm.com/download/win).

You can download the stable version Source code from [release page](https://github.com/IamRezaMousavi/ghermez/releases) or last git version from the [master branch](https://github.com/IamRezaMousavi/ghermez/archive/master.zip). After downloading or cloning, extract and enter ghermez path.

#### 1-2 python and rust

ghermez has been written in rust and python on [persepolis](https://github.com/persepolisdm/persepolis). so we need python3 to build it, after freezing and building the package there are no more need to python and its library. Download latest [python3](https://www.python.org/downloads/) and install it.

remember the destination directory and enable the “**Add Python 3.7 to PATH**” option. (only for windows)

for build ghermez package, you need rust toolchain. Download latest [rust](https://www.rust-lang.org/tools/install) and install it. after building the package there are no more need to rust and its directories.

#### 1-3 dependencies & libraries

Install these libraries with pip from terminal.

```sh
pip install pyqt5 maturin requests setproctitle psutil youtube_dl pyinstaller
```

(only for windows)

```sh
pip install pypiwin32
```

For build ghermez package run this:

```sh
maturin develop
```

Ghermez is gui for [Aria2](https://aria2.github.io/) so we need it, you can download latest or specific version according to your system from [Aria2 release page](https://github.com/aria2/aria2/releases/)

Ghermez uses [ffmpeg](https://www.ffmpeg.org/) for mixing videos(more info.). [Download ffmpeg](https://ffmpeg.zeranoe.com/builds/).

#### 1-4 windows SDK (only for windows)

Download and install the [Windows Software Development Kit (SDK)](https://developer.microsoft.com/en-us/windows/downloads/windows-10-sdk) for Windows 10. we need it for Application Certification Kit API.

### step 2: test and run

Move aria2c.exe and ffmpeg.exe to the test folder next to the test.py according to your system architecture

Open terminal and Enter cloned ghermez directory with cd command. run ghermez as test with this command.

```sh
python test/test.py
```

now ghermez should run as a python script. If you get some error you may had mistake. Open an issue [here](https://github.com/IamRezaMousavi/ghermez/issues), We will help you :)

### step 3: build and freeze

Now let's build ghermez!

Build `ghermez` package:

```sh
maturin develop --release
```

run terminal and enter ghermez folder so build ghermez by pyinstaller with this command:

```sh
pyinstaller "./persepolis/persepolis_download_manager.py"  -w -F -i ./resources/ghermez.ico -n "GhermezDownloadManager" --version-file version.py
```

(for windows)

```sh
pyinstaller '.\persepolis\persepolis_download_manager.py' -p "C:\Program Files (x86)\Windows Kits\10\Redist\ucrt\DLLs\x64" -p C:\python35\Lib\site-packages\PyQt5\Qt\bin\ -w -F -i ./resources/ghermez.ico -n "GhermezDownloadManager" --version-file version.py
```

If you changed windows SDK (step 1-4) and python (step 1-2) installation directory you should change -p(path)

`-w` means it is a windowed app, not a console one.

`-F` Create a one-file bundled executable.

`-i` ghermez icon.

`-n` name of bundled executable.

`--version-file` add ghermez version resource from version.py to the exe.

If you get error messages, you made mistake. Open an issue [here](https://github.com/IamRezaMousavi/ghermez/issues), We will help you :)

- After this, you have bundled executable file in dist folder, Move ffmpeg.exe and aria2c.exe next to the Ghermez Download Manager.exe. you can run it and test it, it works perfectly

### step 4: create package installer (for windows)

You have executable ghermez and you can put it everywhere (next to the and ffmpeg.exe and aria2c.exe) but we going to create a installer for windows.

- Download and install [Inno Setup](http://www.jrsoftware.org/isdl.php)
- you can create your installation or use our standard one, I put theme in this repository for both 32 and 64 architecture (`.iss files`). you should edit `[Files]` section and LicenseFile, InfoAfterFile, OutputBaseFilename, SetupIconFile, UninstallDisplayIcon according to your directory name, also I put license, ghermez readme, after installation text and icon in this repository.
- Build and compile installation if everything goes well, you have a ghermez installer.

Enjoy it. 😊

### step 4: create debian package (for debian-base linux)

You may want to have a deb file, so run:

- Install python package (`pip install stdeb`)
- build deb file with setuptools (`python setup.py --command-packages=stdeb.command bdist_deb`)
