灵感来源于 [gravity_sim](https://github.com/kavan010/gravity_sim)

项目结构：
```
rust_gravity_3d/
├── Cargo.toml
├── src/
│   ├── main.rs              # 主程序入口
│   ├── graphics/            # 图形相关模块
│   │   ├── mod.rs           # 图形模块入口
│   │   ├── shader.rs        # 着色器管理
│   │   ├── camera.rs        # 相机控制
│   │   └── renderer.rs      # 渲染器
│   ├── physics/             # 物理相关模块
│   │   ├── mod.rs           # 物理模块入口
│   │   ├── object.rs        # 物体定义
│   │   ├── gravity.rs       # 引力计算
│   │   └── collision.rs     # 碰撞检测
│   ├── ui/                  # UI相关模块
│   │   ├── mod.rs           # UI模块入口
│   │   └── input.rs         # 输入处理
│   ├── utils/               # 工具模块
│   │   ├── mod.rs           # 工具模块入口
│   │   ├── geometry.rs      # 几何计算
│   │   └── grid.rs          # 网格生成
│   └── constants.rs         # 常量定义
```