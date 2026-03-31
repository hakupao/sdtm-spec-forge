# DataSet Specification Creator - 最终规格（恢复版）

> 恢复说明
>
> 原始 `docs/requirements/04_final_specification.md` 未能从 git 历史中直接找回。
> 当前版本根据以下本地资料重建：
> - `docs/requirements/00_project_overview.md`
> - `docs/requirements/01_questions_round1.md`
> - `docs/requirements/02_questions_round2.md`
> - `docs/requirements/DISCUSSION_LOG.md`
> - `.claude/projects/C--Local-iTMS-DataSet-Specification-Creator/memory/project_sdtm_spec_creator.md`
>
> 因此这是恢复版定稿，不是原文件的逐字副本。

## 1. 项目目标

开发一个工具，从已完成标准化的 SDTM 数据集和配套元数据中，自动生成面向日本客户交付的 Excel 仕様書。

该工具的定位为：

- 作为 Pinnacle 21 的过渡方案
- 帮助客户理解 SDTM 数据集的整体结构、变量含义和来源
- 让客户能够从规格书反向想象 EDC 原始数据的样貌
- 在宏观和微观两个层面同时描述数据

## 2. 目标用户与场景

- 临床数据标准化团队
- 向日本客户纳品 SDTM deliverables 的项目团队
- 需要 review、说明和 handoff 数据结构的 PM / DM / Programmer / Reviewer

## 3. 输入

工具输入包括：

- `SDTM_Data_Set/` 下的 SDTM CSV 文件
  - 当前确认域：`DM`, `CM`, `DS`, `MI`, `PR`, `RS`, `SS`, `TU`
- `SDTM_Master/SDTMIG_v3.4.xlsx`
- `SDTM_Master/SDTM Terminology.xlsx`
- `<STUDY_ID>_OperationConf.xlsx`

其中 `OperationConf.xlsx` 的以下 sheet 对生成规格书尤其关键：

- `Mapping`
- `DomainsSetting`
- `Process`
- `Files`
- `CodeList`

## 4. 输出形式

输出格式确定为：

- Excel 工作簿
- 多 Tab 构成
- 用于客户交付、说明、review 和 handoff

## 5. 语言策略

- 文档整体以日语为主
- Sheet 名使用简洁英语
- SDTMIG 官方元数据保留英语
- 最终输出允许日英混合，但优先保证日本客户可读性

## 6. 标准与范围

- 目标标准：`SDTMIG v3.4`
- 当前重点是“规格书生成”，不是“正式标准校验”
- 不以替代 Pinnacle 21 为目标

当前阶段不考虑：

- Web UI
- XPT 输入
- define.xml 生成
- 多 Study 批处理
- 与 Pinnacle 21 的直接联动格式

## 7. 规格书核心内容

规格书必须同时覆盖：

- 宏观层面
  - 每个 domain 的总体作用
  - 数据集结构
  - 排序键
  - 记录数、变量数
  - 域内分类层级
- 微观层面
  - 每个变量的元数据
  - 变量在当前 study 中的实际意义
  - 数据样本值
  - codelist 对应关系
  - 原始 EDC 字段到 SDTM 变量的追溯关系

## 8. Tab 构成

确认采用“树状图 + 表格”并存的方式，满足不同阅读习惯。

最终 Tab 方向如下：

1. `数据集一览`
   - 全域概要
   - 域名、记录数、结构说明、排序键、变量数、分类层级等
2. `域结构概览`
   - 每个域的 CAT / SCAT / CAT1 / TESTCD / TRT / DECOD 等层级结构
   - 采用文字树状表现
3. `域结构表`
   - 每行一个叶节点路径
   - 用表格方式展开结构
4. `变量一览（每域 1 个 Tab）`
   - 各域变量详细元数据
5. `代码列表一览`
   - 数据中提取的唯一值及 CDISC CT 对照
6. `数据来源追溯`
   - 原始 EDC 字段到 SDTM 变量的关系
7. `备注`
   - 特殊处理、补充说明

## 9. 数据集一览 Tab

当前确认列如下：

- `No.`
- `Domain`
- `Dataset Name`
- `数据概要说明`
- `Structure`
- `Keys`
- `变量数`
- `记录数`
- `分类层级`
- `备注`

说明：

- `数据概要说明` 用日语描述该域的业务含义
- `分类层级` 用于概述该域主要树状维度

## 10. 域结构展示

用户确认采用 C 方案：

- 一个 Tab 放树状图
- 一个 Tab 放表格展开

树状结构要求：

- 使用缩进文字，不使用合并单元格
- 结构顺序以实际 domain 逻辑为准
- 当前修正后的优先层级为：
  - `CAT -> SCAT -> TRT / DECOD / CAT1 / TESTCD`

典型示例：

- `RS`
  - `RSTESTCD=CLINRESP`
  - `RSCAT=CLINICAL UICCV8`
  - `RSSCAT=T_CATEGORY / N_CATEGORY / M_CATEGORY`
- `CM`
  - `CAT=POST TREATMENT`
  - `SCAT=CHEMOTHERAPY MEDICATION / TARGETED MEDICATION`
- `PR`
  - `CAT=ENDOSCOPY / IMAGING / SURGERY / PHYSICAL EXAMINATION`

## 11. 变量一览 Tab

当前确认列如下：

- `No.`
- `Variable Name`
- `Variable Label`
- `Type`
- `Length`
- `Core`
- `Role`
- `Codelist`
- `Format`
- `数据说明`
- `样本值`
- `备注`

补充规则：

- `数据说明` 默认从 `OperationConf.xlsx` 的 `Mapping.DEFINITION` 提取
- 只展示本次数据中非空的字段
- 若某列在当前文件中全部为空，则不在规格书中展示
- `Origin` 列不强制展示，若无实际信息可删除

## 12. EDC 到 SDTM 的追溯

这是规格书的核心差异化能力，必须保留。

原因：

- `Mapping.DEFINITION` 中保存了日文定义
- 这些定义是 EDC 原始含义到 SDTM 变量的桥梁
- 客户需要通过这些说明理解 SDTM 数据与原始数据的关系

例如：

- `RSSCAT=T_CATEGORY` -> `原発腫瘍 壁深達度`
- `RSSCAT=N_CATEGORY` -> `リンパ節転移`
- `RSSCAT=M_CATEGORY` -> `遠隔転移`

## 13. Codelist 输出策略

确认采用 B 方案：

- 域名
- 变量名
- 数据中实际唯一值
- CDISC CT 对应关系

`SDTM Terminology.xlsx` 作为术语标准输入源。

## 14. 非标准变量处理

当前已识别的非标准变量：

- `CMONGO`
- `CMTRT1`
- `DSYNFLG`
- `RSCAT1`

处理规则：

- 在变量一览中用浅黄色背景标记
- `Core` 列显示为 `Custom`
- 需要补充手工说明

其中：

- `CAT1` 非标准字段不应出现在结构展示的最终交付 Excel 中

## 15. Excel 样式要求

确认需要：

- 表头配色
- 列宽自动调整
- 冻结窗格
- 长文本换行显示
- 所有 sheet 不显示目盛线

补充的已确认细节：

- Domain Tab 名直接使用域名，如 `CM`, `DM`, `RS`
- 不添加 `_Variables` 后缀

## 16. 技术方案

最终技术决定为：

- 实现语言：Rust
- 工具形态：CLI
- 配置方式：`TOML` 配置文件 + CLI 参数

选择 Rust 的原因：

- 用户未来希望向 GUI 工具扩展
- 倾向 `Rust + Tauri` 路线
- 当前先完成 CLI，后续为 GUI 留技术基础

## 17. 当前确认结论

| 项目 | 结论 |
|------|------|
| 工具类型 | Rust CLI |
| 输出格式 | Excel 多 Tab 仕様书 |
| 语言 | 日语为主，英语辅助 |
| 标准版本 | SDTMIG v3.4 |
| 元数据来源 | `SDTMIG_v3.4.xlsx` |
| Terminology来源 | `SDTM Terminology.xlsx` |
| Study-specific来源 | `<STUDY_ID>_OperationConf.xlsx` |
| Codelist策略 | 数据提取 + CT 对照 |
| 变量展示规则 | 只展示非空字段 |
| 树状结构展示 | 树状图 + 表格双轨 |
| 非标准变量处理 | 标黄，`Core=Custom` |
| 产品定位 | Pinnacle 21 过渡方案 |

## 18. 关联文件

- `docs/requirements/00_project_overview.md`
- `docs/requirements/01_questions_round1.md`
- `docs/requirements/02_questions_round2.md`
- `docs/requirements/DISCUSSION_LOG.md`
