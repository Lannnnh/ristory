# ristory

<p align="center">
  <a href="https://crates.io/crates/ristory"
    ><img
      src="https://img.shields.io/crates/v/ristory?style=flat-square"
      alt="Crates.io version"
  /></a>
</p>

<p align="center">
<em> a tool for finding zsh history command 😀 </em>
</p>

# Install
## 1. 安装 Rust
安装 Rust 可以参考文档：https://rustwiki.org/zh-CN/book/ch01-01-installation.html ，如果开发环境是 Linux 或 macOS 可以直接执行
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
安装成功后会出现
```bash
Rust is installed now. Great!
```
验证 Rust 环境已经成功安装，在 Shell 中输入
```bash
$ rustc --version
```
看到最新发布的稳定版本的版本号、提交哈希值和提交日期，如下所示格式：
```bash
rustc x.y.z (abcabcabc yyyy-mm-dd)
```
就说明 Rust 环境安装成功了。

## 2. 安装 ristory
直接使用 Rust 包管理工具（Cargo）安装，执行
```bash
$ cargo install ristory
```

## 3. 将 ristory 作为 zsh 插件
将当前仓库下，shell/ristory.zsh 脚本添加到 .zshrc 文件中
```bash
autoload -U add-zsh-hook

_ristory() {
	emulate -L zsh
	zle -I

	echoti rmkx
	output=$(ristory 3>&1 1>&2 2>&3)
	echoti smkx

	if [[ -n $output ]] ; then
		LBUFFER=$output
	fi

	zle reset-prompt
}

zle -N _ristory_widget _ristory

if [[ -z $RISTORY_NOBIND ]]; then
	bindkey '^h' _ristory_widget
fi
```
其中，bindkey 为绑定 ristory 的 zsh 快捷键，可以修改为任何你喜欢的按键；

# 使用帮助
ristory 目前支持多关键词搜索，通过 '&' 来分割关键词，比如输入 "a&b"，就会在 zsh_history 中搜索出所有包含 a 和 b 关键词的命令行。
