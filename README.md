# smolgrep
A simplified recreation of the command-line utility grep written in Rust. 


## Download and run
### Download Rust
#### On Mac/Linux
Open a terminal and enter the following command:

`curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

If the install is successful, the following line will appear:

`Rust is installed now. Great!`


#### On Windows
On Windows, go to https://www.rust-lang.org/tools/install and follow the instructions for installing Rust. 
At some point in the installation, you’ll receive a message explaining that you’ll also need the C++ build tools for Visual Studio 2013 or later. 
The easiest way to acquire the build tools is to install Build Tools for Visual Studio 2019. 
When asked which workloads to install make sure "C++ build tools" is selected and that the Windows 10 SDK and the English language pack components are included.

### Download the utility
Clone this repo, extract it and get inside the directory.

Command srtucture:

`cargo run <PATTERN> <FILENAME>`

Example command:

`cargo run Rust poem.txt`

You can also set the enviroment variable CASE_INSENSITIVE to anything to enable search with case insensitivity.
