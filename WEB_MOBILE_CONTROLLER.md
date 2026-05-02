# Web モバイルコントローラー UI ガイド

## 概要

Nantaraquad Web アプリケーションには、モバイルデバイス用の仮想コントローラーUIが統合されています。
スマートフォンやタブレットからゲームをプレイできるようになっています。

## コントローラー構成

### 1. D-PAD（十字キー）
- **上 (▲)**: 上移動 / Arrow Up
- **下 (▼)**: 下移動 / Arrow Down
- **左 (◄)**: 左移動 / Arrow Left
- **右 (►)**: 右移動 / Arrow Right

### 2. ABCD ボタン
- **A ボタン** (赤): Z キー / ジャンプ・決定
- **B ボタン** (黄): X キー / キャンセル
- **X ボタン** (緑): X キー / サブアクション
- **Y ボタン** (青): Y キー / スペシャルアクション

### 3. Shoulder ボタン
- **L ボタン**: Q キー / 左ショルダー
- **R ボタン**: E キー / 右ショルダー

## 技術仕様

### 自動検出
コントローラーUIはモバイルデバイスで自動的に表示されます：

```javascript
function isMobileDevice() {
    return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent)
        || window.matchMedia("(max-width: 600px)").matches;
}
```

### イベント処理
- **タッチイベント** (`touchstart`, `touchend`): モバイル用
- **マウスイベント** (`mousedown`, `mouseup`): デスクトップ用（開発時）
- **キーボードシミュレーション**: 仮想キープレスを生成

### キーマッピング
```javascript
const keyMap = {
    'up': 'ArrowUp',
    'down': 'ArrowDown',
    'left': 'ArrowLeft',
    'right': 'ArrowRight',
    'z': 'z',      // A ボタン
    'x': 'x',      // B ボタン
    'y': 'y',      // Y ボタン
    'l': 'q',      // L ボタン
    'r': 'e'       // R ボタン
};
```

## 使用方法

### HTML マークアップ
```html
<div class="mobile-controller" id="mobileController">
    <div class="controller-layout">
        <!-- D-PAD -->
        <div class="button-group">
            <label>D-PAD</label>
            <div class="dpad">
                <button class="dpad-up" data-key="up">▲</button>
                <button class="dpad-down" data-key="down">▼</button>
                <button class="dpad-left" data-key="left">◄</button>
                <button class="dpad-right" data-key="right">►</button>
            </div>
        </div>
        
        <!-- ABCD ボタン -->
        <div class="button-group">
            <label>ABCD</label>
            <div class="abcd-buttons">
                <button class="button-abcd button-x" data-key="x">X</button>
                <button class="button-abcd button-y" data-key="y">Y</button>
                <button class="button-abcd button-a" data-key="z">A</button>
                <button class="button-abcd button-b" data-key="x">B</button>
            </div>
        </div>
        
        <!-- L/R ボタン -->
        <div class="button-group">
            <label>Shoulder</label>
            <div class="lr-buttons">
                <button class="button-lr" data-key="l">L</button>
                <button class="button-lr" data-key="r">R</button>
            </div>
        </div>
    </div>
</div>
```

### CSS スタイリング

#### D-PAD
```css
.dpad {
    position: relative;
    width: 100px;
    height: 100px;
}

.dpad button {
    position: absolute;
    width: 40px;
    height: 40px;
    background-color: #444;
    border: 2px solid #666;
}

.dpad button.active {
    background-color: #00ff00;
    border-color: #00ff00;
}
```

#### ABCD ボタン
```css
.button-abcd {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    font-weight: bold;
}

.button-a { background-color: #ff4444; }  /* 赤 */
.button-b { background-color: #ffff44; }  /* 黄 */
.button-x { background-color: #44ff44; }  /* 緑 */
.button-y { background-color: #4444ff; }  /* 青 */

.button-abcd.active {
    box-shadow: 0 0 10px currentColor;
    transform: scale(1.1);
}
```

#### Shoulder ボタン
```css
.button-lr {
    width: 50px;
    height: 30px;
    background-color: #444;
    border: 2px solid #666;
}

.button-lr.active {
    background-color: #00ff00;
    border-color: #00ff00;
}
```

### JavaScript インタラクション

#### ボタン初期化
```javascript
function initMobileController() {
    const controller = document.getElementById('mobileController');
    const buttons = controller.querySelectorAll('button[data-key]');
    
    buttons.forEach(button => {
        const key = button.dataset.key;
        
        button.addEventListener('touchstart', (e) => {
            e.preventDefault();
            button.classList.add('active');
            simulateKeyDown(keyMap[key]);
        });
        
        button.addEventListener('touchend', (e) => {
            e.preventDefault();
            button.classList.remove('active');
            simulateKeyUp(keyMap[key]);
        });
    });
}
```

#### キーシミュレーション
```javascript
function simulateKeyDown(key) {
    const event = new KeyboardEvent('keydown', {
        key: key,
        code: getKeyCode(key),
        keyCode: getKeyCodeNum(key),
        bubbles: true
    });
    document.dispatchEvent(event);
}

function simulateKeyUp(key) {
    const event = new KeyboardEvent('keyup', {
        key: key,
        code: getKeyCode(key),
        keyCode: getKeyCodeNum(key),
        bubbles: true
    });
    document.dispatchEvent(event);
}
```

#### ゲーム開始時の表示
```javascript
window.startLineboy = async () => {
    document.getElementById('status').textContent = '⏳ Loading Lineboy...';
    try {
        await loadWasm('/target/wasm32-unknown-unknown/release/examples/lineboy.wasm');
        window.showMobileController(); // コントローラー表示
    } catch (e) {
        console.error(e);
    }
};
```

## レスポンシブ設計

### デスクトップ
- コントローラーUIは固定位置（下部）に表示
- マウスイベントで操作可能（開発・テスト用）

### モバイル
- 画面下部に表示
- フルスクリーン表示時には非表示（オプション）
- タッチイベントで完全対応

```css
@media (max-width: 600px) {
    .mobile-controller {
        position: relative;
        margin-top: 20px;
        background: rgba(0, 0, 0, 0.9);
        border-top: 2px solid #00ff00;
    }
}
```

## 今後の改善予定

### HIGH優先度
- [ ] フルスクリーン時のコントローラー非表示/表示トグル
- [ ] START/SELECT ボタン追加
- [ ] 画面向き（ポートレート/ランドスケープ）対応

### MEDIUM優先度
- [ ] ジョイスティック（アナログスティック）実装
- [ ] 長押し判定（btnr 相当）
- [ ] 振動フィードバック（Haptic API）

### LOW優先度
- [ ] カスタマイズ可能なボタンレイアウト
- [ ] キー設定のローカルストレージ保存
- [ ] ダークモード対応

## トラブルシューティング

### Q: コントローラーが表示されない

**A:** デバイス判定を確認してください。ブラウザコンソール（F12）で以下を実行：

```javascript
console.log(isMobileDevice()); // true なら表示されるはず
document.getElementById('mobileController').style.display = 'block';
```

### Q: ボタンが反応しない

**A:** キー映像がゲーム側で認識されているか確認：

```javascript
document.addEventListener('keydown', (e) => {
    console.log('Key pressed:', e.key, e.keyCode);
});
```

### Q: タッチがデスクトップでも動作する

**A:** これは仕様です。開発時のテストに役立ちます。
本番環境では `isMobileDevice()` で自動的にモバイルのみに制限されます。

## API 統合

### Pyxel互換 API との連携

```rust
// Rust 側（ゲーム実装）
use nantaraquad::api::*;

fn main() {
    // キーボード入力（Web コントローラーがキーイベント生成）
    if btn(Key::Up) {
        player_y -= 2;
    }
    
    if btnp(Key::Z) { // A ボタン
        fire();
    }
}
```

Web コントローラーが生成するキーイベント → キーボード入力システム → Pyxel API → ゲームロジック

## デモ・テスト方法

### 1. モバイルブラウザでテスト
- iPhone Safari、Android Chrome で web/index.html を開く
- ゲームを起動するとコントローラーが表示される
- タッチでボタン操作

### 2. デスクトップでシミュレーション
- Chrome DevTools → Device Toolbar → Responsive
- マウスで各ボタンをクリック

### 3. コンソール検証
```javascript
// ボタンが機能しているか確認
document.addEventListener('keydown', (e) => console.log('Key:', e.key));
```

