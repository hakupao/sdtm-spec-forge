# DataSet Specification Creator - 项目概要

## 背景
- 已有SDTM标准格式的dataset，需要给日本客户纳品，需制作帮助客户理解数据集的仕様書
- Pinnacle 21是最终目标，但学习和使用成本较高，因此自建工具作为过渡方案
- 现有开源工具中，没有能够直接从SDTM数据自动生成日文仕様書的方案

## 已确定的需求

| 项目 | 决定 |
|------|------|
| 输出格式 | Excel（多Tab构成） |
| 语言 | 日英混合（SDTMIG官方信息用英语，其余用日语） |
| SDTM版本 | SDTMIG v3.4 |
| Codelist来源 | 从数据中自动提取 |
| 技术栈 | Rust |
| 输入数据 | SDTM_Data_Set/ 下的CSV文件（DM, CM, DS, MI, PR, RS, SS, TU） |

## 输出Tab初步方案
1. **数据集一览** - 全域的概要一览
2. **变量一览（按域分Tab）** - 各域的变量元数据
3. **代码列表一览** - 从数据中提取的Controlled Terminology
4. **备注** - 特殊处理及注意事项

## Notes
See the main README.md for setup and usage instructions.
