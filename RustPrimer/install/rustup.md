# Rust bǎnběn guǎnlǐ gōngjù: Rustup rustup shì rust guānfāng de bǎnběn guǎnlǐ gōngjù. Yīng dàng zuòwéi ānzhuāng Rust de shǒuxuǎn. Xiàngmù zhǔyè shì: <Https://Github.Com/rust-lang-nursery/rustup.Rs> ## Features * guǎnlǐ ānzhuāng duō gè guānfāng bǎnběn de Rust èrjìnzhì chéngxù. * Pèizhì jīyú mùlù de Rust gōngjù liàn. * Ānzhuāng hé gēngxīn láizì Rust de fǎ bù tōngdào: Nightly, beta hé stable. * Jiēshōu láizì fābù tōngdào gēngxīn de tōngzhī. * Cóng guānfāng ānzhuāng lìshǐ bǎnběn de nightly gōngjù liàn. * Tōngguò zhǐdìng stable bǎnběn lái ānzhuāng. * Ānzhuāng éwài de std yòng yú jiāochā biānyì. * Ānzhuāng zì dìngyì de gōngjù liàn. * Dúlì měi gè ānzhuāng de Cargo metadata. * Jiào yàn xiàzài de hash zhí. * Jiào yàn qiānmíng (rúguǒ GPG cúnzài). * Duàn diǎn xù chuán. * Zhǐ yīlài bash, curl hé chángjiàn unix gōngjù. * Zhīchí Linux, OS X, Windows(via MSYS2). ## Ānzhuāng ### Windows zài [rustup de zhǔyè](http://Www.Rustup.Rs) xiàzài bìng yùnxíng [rustup-init.Exe](https://Win.Rustup.Rs/), bìng ànzhào tíshì xuǎnzé xuǎnxiàng. ``` Welcome to Rust! This will download and install the official compiler for the Rust programming language, and its package manager, Cargo. It will add the cargo, rustc, rustup and other commands to Cargo's bin directory, located at: C:\Users\Liqueur Librazy\.Cargo\bin This path will then be added to your PATH environment variable by modifying the HKEY_CURRENT_USER/Environment/PATH registry key. You can uninstall at any time with rustup self uninstall and these changes will be reverted. Current installation options: Default host triple: X86_64-pc-windows-msvc default toolchain: Stable modify PATH variable: Yes 1) Proceed with installation (default) 2) Customize installation 3) Cancel installation ``` sān gè xuǎnxiàng fēnbié shì 1) kāishǐ ānzhuāng (mòrèn xuǎnxiàng) 2) zì dìngyì ānzhuāng 3) qǔxiāo qízhōng zì dìngyì ānzhuāng kěyǐ gēnggǎi mòrèn jiàgòu yǔ gōngjù liàn, shìfǒu tiānjiā PATH. Lìrú xiǎng yào xuǎnzé nightly gōngjù liàn kěyǐ jìnxíng yǐxià zì dìngyì
Show more
1,402 / 5,000
Translation results
Translation result
# Rust version management tool: rustup

rustup is the official version management tool for rust. Should be the first choice for installing Rust.

The project home page is: <https://github.com/rust-lang-nursery/rustup.rs>

## Features

* Manages installation of multiple official Rust binaries.
* Configure a directory based Rust toolchain.
* Install and update from Rust's release channels: nightly, beta and stable.
* Receive notifications from release channel updates.
* Install the historical version of the nightly toolchain from the official.
* Install by specifying the stable version.
* Install extra std for cross compilation.
* Install a custom toolchain.
* Cargo metadata per installation independently.
* Check the downloaded hash value.
* Verify signature (if GPG exists).
* http.
* Only depends on bash, curl and common unix tools.
* Supports Linux, OS X, Windows (via MSYS2).

## Install

### Windows

Download and run [rustup-init.exe](https://win.rustup.rs/) on [rustup's homepage](http://www.rustup.rs), and follow the prompts to select options.

```
Welcome to Rust!

This will download and install the official compiler for the Rust programming
language, and its package manager, Cargo.

It will add the cargo, rustc, rustup and other commands to Cargo's bin
directory, located at:

   C:\Users\Liqueur Librazy\.cargo\bin

This path will then be added to your PATH environment variable by modifying the
HKEY_CURRENT_USER/Environment/PATH registry key.

You can uninstall at any time with rustup self uninstall and these changes will
be reversed.

Current installation options:

    default host triple: x86_64-pc-windows-msvc
      default toolchain: stable
   modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
```

The three options are

1) Start the installation (default option)
2) Custom installation
3) cancel

Among them, the custom installation can change the default architecture and tool chain, and whether to add PATH. For example, if you want to choose the nightly toolchain, you can make the following customizations

```
I'm going to ask you the value of each these installation options.
You may simply press the Enter key to leave unchanged.

Default host triple?


Default toolchain? (stable/beta/nightly)
nightly

Modify PATH variable? (y/n)

```

设置完毕后，选择 1 以开始安装。

### Linux & macOS

运行以下命令

```
curl https://sh.rustup.rs -sSf | sh
```

这个命令将会编译和安装 rustup, 安装过程中可能会提示你输入 sudo 的密码。 然后, 他会下载和安装 stable 版本的工具链, 当执行 rustc, rustdoc 和 cargo 时, 将会配置他为默认工具链。

`Unix` 上安装后工具链会被安装到 `$HOME/.cargo/bin` 目录。

`.cargo/bin` 目录会被添加到系统的 `$PATH` 环境变量,重新登录后即可使用 `rustc`，`cargo` 等命令。

## 卸载

```
rustup self uninstall
```

## 用法

安装后会得到一个 rustup 命令, 多使用命令自带的帮助提示, 可以快速定位你需要功能。

### 帮助

运行 ` rustup -h` 你将会得到如下提示:

```
❯ rustup -h
rustup 1.5.0 (92d0d1e9e 2017-06-24)
The Rust toolchain installer

USAGE:
    rustup.exe [FLAGS] [SUBCOMMAND]

FLAGS:
    -v, --verbose    Enable verbose output
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    show           Show the active and installed toolchains
    update         Update Rust toolchains and rustup
    default        Set the default toolchain
    toolchain      Modify or query the installed toolchains
    target         Modify a toolchain's supported targets
    component      Modify a toolchain's installed components
    override       Modify directory toolchain overrides
    run            Run a command with an environment configured for a given toolchain
    which          Display which binary will be run for a given command
    doc            Open the documentation for the current toolchain
    self           Modify the rustup installation
    set            Alter rustup settings
    completions    Generate completion scripts for your shell
    help           Prints this message or the help of the given subcommand(s)

DISCUSSION:
    rustup installs The Rust Programming Language from the official
    release channels, enabling you to easily switch between stable,
    beta, and nightly compilers and keep them updated. It makes
    cross-compiling simpler with binary builds of the standard library
    for common platforms.

    If you are new to Rust consider running `rustup doc --book` to
    learn Rust.

```

根据提示, 使用 `rust help <command>` 来查看子命令的帮助。

`rustup doc --book` 会打开英文版的 [The Rust Programming Language](https://doc.rust-lang.org/book/)。

### 常用命令

`rustup default <toolchain>` 配置默认工具链。

`rustup show` 显示当前安装的工具链信息。

`rustup update` 检查安装更新。

`rustup toolchain [SUBCOMMAND]` 配置工具链

> * `rustup toolchain install <toolchain>` 安装工具链。
> * `rustup toolchain uninstall <toolchain>` 卸载工具链。
> * `rustup toolchain link <toolchain-name> "<toolchain-path>"` 设置[自定义工具链](https://github.com/rust-lang-nursery/rustup.rs#working-with-custom-toolchains-and-local-builds)。
> 
> 其中标准的 `<toolchain>`具有如下的形式
> ```
> `<channel>[-<date>][-<host>]`
> <channel>       = stable|beta|nightly|<version>
> <date>          = YYYY-MM-DD
> <host>          = <target-triple>
> ```
> 如 `stable-x86_64-pc-windows-msvc` `nightly-2017-7-25` `1.18.0` 等都是合法的toolchain名称。

`rustup override [SUBCOMMAND]` 配置一个目录以及其子目录的默认工具链

> 使用 `--path <path>` 指定目录或在某个目录下运行以下命令
> 
> * `rustup override set <toolchain>` 设置该目录以及其子目录的默认工具链。
> * `rustup override unset` 取消目录以及其子目录的默认工具链。
> 
> 使用 `rustup override list` 查看已设置的默认工具链。

`rustup target [SUBCOMMAND]` 配置工具链的可用目标

> * `rustup target add <target>` 安装目标。
> * `rustup target remove <target>` 卸载目标。
> * `rustup target add --toolchain <toolchain> <target>` 为特定工具链安装目标。

`rustup component` 配置 rustup 安装的组件

> * `rustup component add <component>` 安装组件
> * `rustup component remove <component>` 卸载组件
> * `rustup component list` 列出可用组件
>
> 常用组件：
> * Rust 源代码 `rustup component add rust-src`
> * Rust Langular Server (RLS) `rustup component add rls`
