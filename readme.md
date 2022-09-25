# 23wards

東京23区をランダムで表示するRust/Yewのサンプルアプリ

## 目的

- Rustの学習
- Yewの試用

## 開発環境立ち上げ

1. `config.example.toml`を参考に`.cargo/config.toml`に`MAPS_API_KEY`を追加する

2. Tailwind CSSのbuild/変更監視

    ```shell
    npx tailwindcss -i ./styles.scss -o ./styles.css --watch
    ```

3. Yewのbuild/変更監視

    ```shell
    trunk serve
    ```

## デプロイ

1. Tailwind CSSのbuild

    ```shell
    npx tailwindcss -o styles.css --minify
    ```

1. Yewのbuild

    ```shell
    trunk build
    ```
