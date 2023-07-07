# sakusei 作成 🦀

Sakusei (Saku) is a versatile alternative for 'touch' written in Rust that allows you to quickly create new files and boilerplates for popular programming languages.

## Prerequisites
Before you can compile and use Sakusei, you need to have Cargo installed on your system. Cargo is the package manager and build system for Rust. You can install it by following the instructions at [Rust's official website](https://www.rust-lang.org/tools/install).

## Installation
To install Sakusei globally on your system, you can use Cargo's `install` command with the `--locked` flag to ensure a consistent version and the `--git` flag to specify the repository:

```shell
cargo install --locked --git https://github.com/fromgodd/sakusei saku
```
This command will download the source code from the given GitHub repository and build Sakusei, making it available as a global command-line tool.


## Usage
Once you have installed Sakusei, you can start using it to create new files and boilerplate code in a breeze! Here's how you can use Sakusei like a pro programmer!

### Creating a New File
To create a new file, simply open your terminal or command prompt and type the saku command followed by the name of the file you want to create. For example, let's say you want to create a file called myfile.txt. Here's what you need to do:
```shell
saku myfile.txt
```

And just like that, Sakusei will create a new file called myfile.txt in the current directory. You can replace myfile.txt with any name you like.

## Creating Boilerplate Code
Sakusei also provides the ability to create boilerplate code for popular programming languages. This can save you time and effort when starting a new project. To create boilerplate code, you need to specify the programming language using the --language or -l option, followed by the name of the file you want to create. Additionally, Sakusei provides the ability to generate .ignore files for Git and other purposes. To generate an .ignore file, you need to provide the desired arguments. For more information on generating .ignore files and the required arguments, please refer to the documentation.
Also you can use the --help flag

```shell
saku --help
```

Remember to update the content of the `.ignore` file generation section once you have specific instructions and arguments for that feature.


## License
This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).