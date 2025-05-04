# qline

`qline` は, 標準入力またはファイルからのテキストをメール返信スタイルで引用（`>` を各行の先頭に付与）するシンプルな CUI ツールです.

## 特長

- 各行の先頭に `>` を付けて引用形式に変換
- パイプとの併用が可能

## Usage

```sh
echo "Hello,\nThis is a test." | qline
```

出力:

```text
> Hello,
> This is a test.
```

またはファイルから:

```sh
qline input.txt
```

## License

MIT License

## Author

TAKAHASHI Satoshi <hikobae@gmail.com>
