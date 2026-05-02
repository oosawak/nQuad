# WASM デバッグガイド

現在、Lineboy WASM がブラウザで正しくロードされていません。
デバッグページを使用して、問題を特定してください。

## デバッグページを開く

```
http://localhost:8000/debug_test.html
```

## ブラウザの開発者コンソールで確認すること

1. **WASM バイナリは見つかるか？**
   - Network タブで `/lineboy/lineboy_bg.wasm` を確認
   - Status が 200 で、Content-Type が `application/wasm` か確認

2. **JavaScript ファイルは見つかるか？**
   - Network タブで `/lineboy/lineboy.js` を確認
   - Status が 200 で、サイズが約4.8KB か確認

3. **デバッグページの出力をスクリーンショット**
   - エラーが出ているステップを確認

## よくある問題と解決策

### Problem 1: WASM ファイルが 404

**症状**: Network タブで `/lineboy_bg.wasm` が赤表示

**解決策**:
```bash
# WASM ファイルが web/ にあるか確認
ls -lh /home/oosawak/Workspace/Nantaraquad/web/lineboy/

# ない場合は、ビルドして再度コピー
cd /home/oosawak/Workspace/Nantaraquad/games/lineboy
./build.sh
```

### Problem 2: "Cannot read properties of undefined (reading 'init')"

**症状**: `lineboy.js` の 4 行目でエラー

**原因**: `await wasmModule.default()` が実行されていない、または失敗している

**確認**:
- デバッグページで「Step 3: Calling default()...」がパスするか確認
- 失敗する場合は、Error メッセージをメモ

### Problem 3: Module imported but init is not a function

**症状**: `lineboy.js` はロードされるが、関数がない

**原因**: wasm-pack のビルドが不完全か、古いファイルが キャッシュされている

**解決策**:
```bash
# ブラウザキャッシュをクリア
# Chrome: Ctrl+Shift+Delete → Cached images and files を削除

# または、強制更新
# Chrome: Ctrl+Shift+R
```

### Problem 4: CORS error

**症状**: CORS policy error

**解決策**:
```bash
# server.js が Access-Control-Allow-Origin ヘッダを設定しているか確認
grep "Access-Control" /home/oosawak/Workspace/Nantaraquad/server.js

# これが出ていればOK:
# 'Access-Control-Allow-Origin': '*'
```

## デバッグページの結果をシェアする

デバッグページの出力をスクリーンショットまたはテキストで共有してください。
そこから問題を特定できます。

