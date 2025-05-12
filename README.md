# material_keeper

> !!!目前仅属于 chixiaoshu 学习 Rust 以及自行使用。

一个用于管理物料信息的 Rust 项目，支持添加、查询和更新物料数据。

## 项目结构

- `material_core/`：核心功能模块
- `material_lib/`：公共库，定义了 `Material` 结构体等
- `material_gui/`：图形用户界面模块（待开发）
- `material_scan/`：扫描功能模块（待开发）

## 使用方法

1. 克隆仓库：

   ```bash
   git clone https://github.com/chixiaoshu/material_keeper.git
   ````

2. 进入目录并构建项目：

   ```bash
   cd material_keeper
   cargo build
   ```

3. 运行示例：

   ```bash
   cargo run --bin material_core
   ```

## License

本项目采用 MIT 许可证。详情请见 [LICENSE](./LICENSE) 文件。

---

本项目由 chixiaoshu 编写，仅供学习与个人使用。如有建议欢迎提 issue。

---
