# char-cli

可以用来将 Unicode 字符转换为 Unicode 转义序列
---

<!-- PROJECT SHIELDS -->

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]

## 目录

- [使用方法](#使用方法)
    - [安装](#安装)
    - [帮助](#帮助)
    - [输出字符数值](#输出字符数值)
    - [输出转义序列](#输出转义序列)
    - [解析转义序列](#解析转义序列)
    - [输出ASCII编码表](#输出ASCII编码表)
- [作者](#作者)

### 使用方法

###### 安装

将安装至 **$HOME/.cargo/bin** 目录下，请将该目录添加至 **$PATH** 变量中

``` shell
cargo install char-cli
```

###### 帮助

``` shell
$ char-cli --help
Usage: char-cli [OPTIONS]

Options:
  -e, --escape    Convert unicode string to unicode escape sequences
  -u, --unescape  Convert unicode escape sequences to unicode string
  -a, --ascii     Output ascii table
  -h, --help      Print help
  -V, --version   Print version


```

###### 输出字符数值

``` shell
$ echo "abcdˍ©ƒ®¥你好中国🍎🍊" | char-cli
a      00000000000000000000000001100001 0x00000061 0o00000000141
b      00000000000000000000000001100010 0x00000062 0o00000000142
c      00000000000000000000000001100011 0x00000063 0o00000000143
d      00000000000000000000000001100100 0x00000064 0o00000000144
ˍ      00000000000000000000001011001101 0x000002CD 0o00000001315
©      00000000000000000000000010101001 0x000000A9 0o00000000251
ƒ      00000000000000000000000110010010 0x00000192 0o00000000622
®      00000000000000000000000010101110 0x000000AE 0o00000000256
¥      00000000000000000000000010100101 0x000000A5 0o00000000245
你     00000000000000000100111101100000 0x00004F60 0o00000047540
好     00000000000000000101100101111101 0x0000597D 0o00000054575
中     00000000000000000100111000101101 0x00004E2D 0o00000047055
国     00000000000000000101011011111101 0x000056FD 0o00000053375
🍎     00000000000000011111001101001110 0x0001F34E 0o00000371516
🍊     00000000000000011111001101001010 0x0001F34A 0o00000371512
```

###### 输出转义序列

``` shell
$ echo "abcd你好中国" | char-cli -e 
\u0061\u0062\u0063\u0064\u4F60\u597D\u4E2D\u56FD
```

###### 解析转义序列

``` shell
$ echo "\u0061\u0062\u0063\u0064\u4F60\u597D\u4E2D\u56FD\U0001F34E\U0001F34A\U0001F34A" | char-cli -u
abcd你好中国🍎🍊🍊 
```

###### 输出ASCII编码表

```shell
$ char-cli -a
0000 0x00 NUL   0001 0x01 SOH   0002 0x02 STX   0003 0x03 ETX   
0004 0x04 EOT   0005 0x05 ENQ   0006 0x06 ACK   0007 0x07 BEL   
0008 0x08 BS    0009 0x09 HT    0010 0x0a LF    0011 0x0b VT    
0012 0x0c FF    0013 0x0d CR    0014 0x0e SO    0015 0x0f SI    
0016 0x10 DLE   0017 0x11 DC1   0018 0x12 DC2   0019 0x13 DC3   
0020 0x14 DC4   0021 0x15 NAK   0022 0x16 SYN   0023 0x17 ETB   
0024 0x18 CAN   0025 0x19 EM    0026 0x1a SUB   0027 0x1b ESC   
0028 0x1c FS    0029 0x1d GS    0030 0x1e RS    0031 0x1f US    
0032 0x20       0033 0x21 !     0034 0x22 "     0035 0x23 # 
...
```

### 作者

darkiiiiiice@gmail.com

### 版权说明

该项目签署了MIT 授权许可，详情请参阅 [LICENSE](https://github.com/darkiiiiiice/char-cli/blob/master/LICENSE)


<!-- links -->

[your-project-path]:darkiiiiiice/char-cli

[contributors-shield]: https://img.shields.io/github/contributors/darkiiiiiice/char-cli.svg?style=flat-square

[contributors-url]: https://github.com/darkiiiiiice/char-cli/graphs/contributors

[forks-shield]: https://img.shields.io/github/forks/darkiiiiiice/char-cli.svg?style=flat-square

[forks-url]: https://github.com/darkiiiiiice/char-cli/network/members

[stars-shield]: https://img.shields.io/github/stars/darkiiiiiice/char-cli.svg?style=flat-square

[stars-url]: https://github.com/darkiiiiiice/char-cli/stargazers

[issues-shield]: https://img.shields.io/github/issues/darkiiiiiice/char-cli.svg?style=flat-square

[issues-url]: https://img.shields.io/github/issues/darkiiiiiice/char-cli.svg

[license-shield]: https://img.shields.io/github/license/darkiiiiiice/char-cli.svg?style=flat-square

[license-url]: https://github.com/darkiiiiiice/char-cli/blob/master/LICENSE.txt

[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=flat-square&logo=linkedin&colorB=555

[linkedin-url]: https://linkedin.com/in/shaojintian



