<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD041 -->
<p align="center">
  <img src="./resources/ghermez-with-text.png" />
</p>
<h1 align="center">Ghermez Download Manager</h1>
<p align="center">
  A Fast and Safe Download Manager build with ❤️
  <br/>
  <br/>
  <a href="https://github.com/IamRezaMousavi/ghermez"><strong>Explore the docs »</strong></a>
  <br/>
  <br/>
  <a href="https://github.com/IamRezaMousavi/ghermez/releases">View Release</a>
  .
  <a href="https://github.com/IamRezaMousavi/ghermez/issues">Report Bug</a>
  .
  <a href="https://github.com/IamRezaMousavi/ghermez/issues">Request Feature</a>
</p>

[![Latest stable release](https://img.shields.io/github/v/release/IamRezaMousavi/ghermez)](https://github.com/IamRezaMousavi/ghermez/releases) [![Total downloads](https://img.shields.io/github/downloads/IamRezaMousavi/ghermez/total)](https://github.com/IamRezaMousavi/ghermez) [![GitHub license](https://img.shields.io/github/license/IamRezaMousavi/ghermez)](https://github.com/IamRezaMousavi/ghermez/blob/master/LICENSE) [![GitHub last commit (branch)](https://img.shields.io/github/last-commit/IamRezaMousavi/ghermez/master)](https://github.com/IamRezaMousavi/ghermez/commits/master) [![GitHub contributors](https://img.shields.io/github/contributors/IamRezaMousavi/ghermez)](https://github.com/IamRezaMousavi/ghermez/graphs/contributors) [![GitHub commit activity (branch)](https://img.shields.io/github/commit-activity/y/IamRezaMousavi/ghermez)](https://github.com/IamRezaMousavi/ghermez/commits/master) [![AUR version](https://img.shields.io/aur/version/ghermez)](https://aur.archlinux.org/packages/ghermez)

## Table Of Contents

- [About the Project](#about-the-project)
- [Installation](#installation)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [Acknowledgements](#acknowledgements)

## About The Project

|GNU/Linux|Mac OSX|Windows|  
|:---:|:---:|:---:|
|![persepolis](./resources/screenshots/persepolis.png)|![mac](./resources/screenshots/mac.png)|![windows](./resources/screenshots/windows.png)|

Ghermez is a download manager & a GUI for [Aria2](https://github.com/aria2/aria2). It's written in `Python` and `Rust`. Ghermez is a sample of free and open source software. It's developed for GNU/Linux distributions, BSDs, MacOS, and Microsoft Windows.

### **Features**

- Multi-segment downloading
- Scheduling downloads
- Download queuing
- and many more!

## Installation

There are a few installation options for Ghermez. If you would like another installation option, please open an issue for it.

### Windows

Download the installer .exe file from [latest release](https://github.com/IamRezaMousavi/ghermez/releases/latest).

### Linux

#### Arch Linux

Download the .pkg.tar.zst package from [latest release](https://github.com/IamRezaMousavi/ghermez/releases/latest) and run:

```sh
sudo pacman -S <file-name>.pkg.tar.zst
```

Or download [PKGBUILD](./archlinux/PKGBUILD) file in `archlinux` directory and run:

```sh
makepkg -si
```

#### Debian-base Linux

Download the .deb package from [latest release](https://github.com/IamRezaMousavi/ghermez/releases/latest) and run:

```sh
sudo dpkg -i <file-name>.deb
```

### Source Code

**Run below commands with `sudo`**

- First you have to install [python](https://www.python.org/downloads/) and [rust](https://www.rust-lang.org/tools/install)

- Install requirement packages

```sh
pip install -U -r ./requirements.txt
```

- Run setup script

```sh
python setup.py install
```

## Roadmap

See the [open issues](https://github.com/ShaanCoding/ReadME-Generator/issues) for a list of proposed features (and known issues).

## Contributing

Information describing how to contribute can be found in the file [CONTRIBUTING.md](./CONTRIBUTING.md)

## Acknowledgements

- [persepolis](https://github.com/persepolisdm/persepolis)
- [aria2-ws-rs](https://github.com/ComfyFluffy/aria2-ws-rs)
- [maturin](https://github.com/PyO3/maturin)

---
_Is there any bug? Report it in [issue tracker](https://github.com/IamRezaMousavi/ghermez/issues) or correct it by yourself._
