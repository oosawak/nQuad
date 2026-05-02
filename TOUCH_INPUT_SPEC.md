# Nantaraquad タッチ入力仕様書

## 現状

### 1. デスクトップゲーム（Native Binaries）
- **マウス入力**: 完全対応
  - `is_mouse_button_down(MouseButton::Left)` で左クリック検出
  - `mouse_position()` で座標取得
  - Lineboy/Cubeboy では直接マウス入力を使用

- **タッチ入力**: macroquad フレームワーク経由で利用可能
  - `touches()` - タッチポイント一覧取得
  - `touch_position(index)` - タッチ座標取得
  - **現在は使用されていない**（マウス入力のみ）

### 2. WASM ゲーム（ブラウザ）

#### A. 直接タッチ操作
- **スクリーン上でのタッチ**: 実装されていない
  - macroquad WASM は理論的にはタッチ対応
  - しかし、ゲーム内では使用されていない
  - `touches()` / `touch_position()` は使用していない

#### B. 仮想コントローラー経由（新規実装）
```
タッチコントローラーボタン → KeyboardEvent生成 → ゲーム入力システム → btn()/btnp()
```

- **D-PAD**: 十字キー（Arrow Keys）
- **ABCD ボタン**: Z/X/Y キー
- **L/R ボタン**: Q/E キー

**制限**: キーボード入力ベースのため、マウス位置は利用できない

---

## 入力フロー比較

### 現在のデスクトップフロー
```
マウス左クリック
  ↓
is_mouse_button_down(Left)
  ↓
mouse_position() で座標取得
  ↓
ゲーム内で直接 (x, y) をピクセル描画
```

### 現在の WASM フロー
```
仮想コントローラーボタンをタッチ
  ↓
JavaScript KeyboardEvent生成
  ↓
btn(Key::Up/Down/Left/Right) など
  ↓
ゲーム内でキー入力に対応した処理
```

**問題点**: 
- マウス座標が渡されないため、キャンバス描画（Lineboy の例）に対応できない
- タッチドラッグが検出されない

---

## 技術仕様

### macroquad タッチ API
```rust
use macroquad::prelude::*;

// タッチ入力取得
let touches = touches();  // Vec<Touch>
for touch in touches {
    let (x, y) = touch.position;
    println!("Touch at ({}, {})", x, y);
}

// または個別取得
if let Some(pos) = touch_position(0) {
    println!("Touch 0 at {:?}", pos);
}

// タッチイベント検出
match touch_phase {
    TouchPhase::Started => { /* 新しくタッチ開始 */ },
    TouchPhase::Stationary => { /* タッチ継続中 */ },
    TouchPhase::Moved => { /* タッチ位置移動 */ },
    TouchPhase::Ended => { /* タッチ終了 */ },
    TouchPhase::Cancelled => { /* タッチキャンセル */ },
}
```

### WASM タッチキャプチャ（JavaScript）
```javascript
// Canvas タッチイベント
canvas.addEventListener('touchstart', (e) => {
    e.preventDefault();
    const touch = e.touches[0];
    const x = touch.clientX - canvas.getBoundingClientRect().left;
    const y = touch.clientY - canvas.getBoundingClientRect().top;
    console.log(`Touch start at (${x}, ${y})`);
});

canvas.addEventListener('touchmove', (e) => {
    e.preventDefault();
    const touch = e.touches[0];
    // ...
});

canvas.addEventListener('touchend', (e) => {
    console.log('Touch ended');
});
```

---

## 現在の制限と解決方法

### 問題 1: WASM でマウス座標が渡されない

**原因**: 
- 仮想コントローラーはキーボード入力をシミュレートするだけ
- マウス座標情報は含まれていない

**解決方法**:
1. **オプション A**: Canvas にタッチハンドラーを追加し、macroquad に座標を渡す
2. **オプション B**: JavaScript でタッチ座標を取得し、SharedMemory/MessageChannel でゲームに伝達
3. **オプション C**: 描画が必要なゲーム向けに特別なタッチハンドラーを実装

### 問題 2: マルチタッチ対応なし

**現状**: 
- 仮想コントローラーはボタン毎に個別のタッチイベント
- 複数のボタン同時押しはサポート

**今後**: 
- キャンバス直接タッチでのマルチタッチが必要な場合は別途実装

---

## 推奨される改善案

### SHORT TERM（推奨）

#### 1. Canvas タッチハンドラー追加
```javascript
// web/index.html に追加
canvas.addEventListener('touchstart', (e) => {
    const touch = e.touches[0];
    const rect = canvas.getBoundingClientRect();
    const x = (touch.clientX - rect.left) * (canvas.width / rect.width);
    const y = (touch.clientY - rect.top) * (canvas.height / rect.height);
    
    // macroquad に座標を伝達
    simulateMouseEvent('mousedown', x, y);
});

canvas.addEventListener('touchmove', (e) => {
    e.preventDefault();
    const touch = e.touches[0];
    const rect = canvas.getBoundingClientRect();
    const x = (touch.clientX - rect.left) * (canvas.width / rect.width);
    const y = (touch.clientY - rect.top) * (canvas.height / rect.height);
    simulateMouseEvent('mousemove', x, y);
});

canvas.addEventListener('touchend', (e) => {
    simulateMouseEvent('mouseup', 0, 0);
});

function simulateMouseEvent(type, x, y) {
    const event = new MouseEvent(type, {
        bubbles: true,
        cancelable: true,
        clientX: x,
        clientY: y
    });
    canvas.dispatchEvent(event);
}
```

**効果**: 
- Lineboy のようなドラッグ描画が WASM でも動作
- 既存のマウス入力コードがそのまま機能

#### 2. 仮想コントローラーの改善
```javascript
// 既存の仮想コントローラーを保持
// + ゲームが入力ベースではなく座標ベースの場合は Canvas タッチを使用
```

### MEDIUM TERM

#### 1. ゲームアプリのタイプ分け
- **キー入力型**: 仮想コントローラーのみ使用（Platformer など）
- **マウス座標型**: Canvas タッチ＋マウスイベント（描画アプリなど）

#### 2. フルスクリーン対応
```javascript
document.getElementById('canvas').requestFullscreen();
// フルスクリーン時に仮想コントローラーをオーバーレイ表示
```

### LONG TERM

#### 1. ジョイスティック（アナログ入力）
- 仮想コントローラーに D-PAD の代わりにアナログスティック
- または D-PAD + アナログスティック両立

#### 2. 複数タッチジェスチャー
- ピンチズーム
- スワイプ認識
- マルチタッチドラッグ

#### 3. デバイス物理入力
- ゲームパッド API サポート
- モーションセンサー

---

## ゲーム毎の仕様

### Lineboy
```
現状：
  - デスクトップ: マウスドラッグで描画
  - WASM: 仮想コントローラーのみ（描画できない）

改善後（Canvas タッチ導入）:
  - デスクトップ: マウスドラッグで描画
  - WASM: タッチドラッグで描画 + 仮想コントローラー
```

### Cubeboy
```
現状：
  - デスクトップ: 矢印キー + A/B キー
  - WASM: 仮想コントローラー（D-PAD + ABCD）

改善後：
  - デスクトップ: 同じ
  - WASM: 同じ（完全に機能する）

※ Cubeboy は既に完全対応
```

---

## 実装チェックリスト

- [ ] Canvas touchstart/touchmove/touchend ハンドラー追加
- [ ] マウスイベントへの変換ロジック
- [ ] DPI スケーリング対応（高 DPI デバイス）
- [ ] マルチタッチ判定（複数タッチ時の処理）
- [ ] 仮想コントローラーとの競合回避
- [ ] iOS Safari での動作確認
- [ ] Android Chrome での動作確認
- [ ] タッチラグ最小化（イベント遅延）

---

## 参考

### macroquad 公式ドキュメント
- タッチ API: `touches()`, `touch_position()`
- マウス API: `mouse_position()`, `is_mouse_button_down()`

### WASM Canvas タッチ
- Canvas MDN: https://developer.mozilla.org/en-US/docs/Web/API/Touch_events
- TouchEvent: `touches`, `targetTouches`, `changedTouches`

