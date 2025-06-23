# Agario 3D - Bevy ゲーム設計図

## 1. プロジェクト構造

```
src/
├── main.rs                 # エントリーポイント
├── game/
│   ├── mod.rs             # ゲームモジュール
│   ├── entities/          # エンティティ定義
│   │   ├── player.rs      # プレイヤー球体
│   │   ├── food.rs        # 食べ物
│   │   └── enemy.rs       # 敵AI
│   ├── systems/           # ゲームシステム
│   │   ├── movement.rs    # 移動処理
│   │   ├── collision.rs   # 衝突検出
│   │   ├── growth.rs      # 成長システム
│   │   └── spawning.rs    # スポーンシステム
│   └── components/        # コンポーネント定義
│       ├── physics.rs     # 物理コンポーネント
│       ├── size.rs        # サイズ管理
│       └── health.rs      # 生存状態
├── rendering/
│   ├── materials.rs       # マテリアル定義
│   ├── camera.rs          # カメラコントローラー
│   └── lighting.rs        # ライティング設定
├── network/               # ネットワーキング（将来実装）
│   ├── client.rs
│   └── server.rs
└── utils/
    ├── math.rs            # 数学ユーティリティ
    └── config.rs          # 設定管理
```

## 2. 核心コンポーネント設計

### 基本コンポーネント
```rust
// プレイヤー球体
#[derive(Component)]
struct Player {
    mass: f32,
    speed: f32,
    max_speed: f32,
}

// 食べ物
#[derive(Component)]
struct Food {
    nutrition: f32,
    respawn_timer: f32,
}

// 敵AI
#[derive(Component)]
struct Enemy {
    target: Option<Entity>,
    aggression: f32,
}

// サイズ管理
#[derive(Component)]
struct Size {
    radius: f32,
    mass: f32,
}

// 物理プロパティ
#[derive(Component)]
struct Physics {
    velocity: Vec3,
    acceleration: Vec3,
}
```

## 3. ゲームメカニクス

### 移動システム
- マウス/タッチ入力でプレイヤー移動
- 質量に応じた移動速度調整
- 慣性とドラッグ効果

### 成長システム
- 食べ物を食べると質量増加
- 質量に応じて球体サイズ拡大
- 大きくなるほど移動速度減少

### 衝突検出
- 球体同士の衝突判定
- 大きい球体が小さい球体を吸収
- 質量差による勝敗判定

## 4. 3D 視覚設計

### カメラシステム
- プレイヤー追従カメラ
- 球体サイズに応じたズーム調整
- スムーズなカメラ移動

### レンダリング
- PBR マテリアル使用
- 球体の光沢表現
- パーティクルエフェクト（吸収時）

### ライティング
- 環境光とディレクショナルライト
- 球体の立体感を強調
- 色による識別性向上

## 5. 実装フェーズ

### Phase 1: 基本機能
1. プレイヤー球体の移動
2. 食べ物の配置と消費
3. 基本的な成長システム

### Phase 2: ゲームプレイ
1. 敵AIの実装
2. 衝突検出と吸収システム
3. スコアとランキング

### Phase 3: 視覚強化
1. パーティクルエフェクト
2. UI実装
3. サウンドエフェクト

### Phase 4: マルチプレイヤー
1. ネットワーキング実装
2. サーバー・クライアント分離
3. リアルタイム同期

## 6. 技術要件

### Bevy プラグイン
- `bevy_rapier3d` - 物理エンジン
- `bevy_renet` - ネットワーキング
- `bevy_kira_audio` - オーディオ
- `bevy_egui` - UI

### パフォーマンス考慮
- LOD（Level of Detail）システム
- 視野外オブジェクトのカリング
- 効率的な衝突検出（空間分割）

この設計図を基に段階的に実装を進めることで、スケーラブルな Agario 3D ゲームを構築できます。