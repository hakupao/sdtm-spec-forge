# 讨论记录

## 2026-03-24 - 初次讨论

### 用户已确认的需求
- Pinnacle 21为最终目标，当前自建工具作为过渡方案
- Excel（多Tab）形式输出仕様書
- 日英混合（SDTMIG官方信息 = 英语，其余 = 日语）
- SDTMIG v3.4标准
- Codelist从数据中提取
- 技术栈: 待定（初步想法Rust，但愿意听取建议）
- 输入: CSV格式的SDTM数据集（8个域）

## 2026-03-24 - 第1轮回答汇总 & 数据分析

### 用户回答要点
- Q1: Tab构成OK，另有SDTM_ENSEMBLE文件夹含映射规则
- Q2: 暂时够用
- Q3: 选C方案（树状图Tab + 表格展开Tab，混合）
- Q4: 变量一览列OK
- Q5: 已提供SDTM_Master文件夹 → B方案（使用现有元数据文件）
- Q6: 选B方案（含CDISC CT对应关系）
- Q7: 表头配色、列宽自动调整、冻结窗格、缩进文字
- Q8: 暂不考虑扩展性
- Q9: 暂不考虑实现方式，先确认需求

### 数据分析发现
- SDTMIG_v3.4.xlsx：完整的元数据（Variables + Datasets sheets）
- SDTM Terminology.xlsx：完整的CDISC CT（2022-09-30版）
- ENSEMBLE_OperationConf.xlsx：映射规则，含日文DEFINITION（极有价值）
- 4个非标准变量：CMONGO, CMTRT1, DSYNFLG, RSCAT1
- 所有8个域在SDTMIG中都有完整覆盖

## 2026-03-24 - 第2轮回答汇总

### 用户回答要点
- Q10: ENSEMBLE_OperationConf.xlsx的DEFINITION列必须活用，帮助客户理解EDC原始含义
- Q11: 非标准变量用浅黄色背景标记，Core标为Custom → 同意
- Q12: 最终选择Rust（用户将来想做GUI工具，Python图形化体验不好，Rust+Tauri方向）
- Q13: 只展示非空字段，空字段不出现在仕様書中

## 2026-03-25 - 开始编码

### 技术决定
- 技术栈: Rust
- 依赖: rust_xlsxwriter（写Excel）、calamine（读Excel）、csv、clap、serde、toml、chrono、indexmap
- 项目结构: sdtm-spec-creator/（Cargo项目）
- 配置方式: TOML配置文件（config.toml）+ CLI参数（可覆盖）

### 实现进度
- [x] CSV读取器
- [x] SDTMIG元数据读取器
- [x] CDISC Terminology读取器
- [x] 映射配置读取器
- [x] 树状结构分析器（CAT → SCAT → TRT/DECOD/TESTCD）
- [x] 变量统计分析器（只保留非空字段）
- [x] Codelist匹配器
- [x] Excel生成器（全17个Tab）
- [x] TOML配置文件支持

### 用户反馈修正记录
1. 所有sheet不需要目盛線 → `set_screen_gridlines(false)`
2. Domain Tab名直接用域名（CM, DM...），不加_Variables后缀
3. 层级顺序修正: CAT → SCAT → TRT/DECOD/CAT1/TESTCD（不是TESTCD在最前面）
4. Overview的D列（英文Description）删除
5. Cover的B7:C10加表格线
6. DatasetList的D列删除，K列改名Sort Key
7. CAT1非标准字段，不出现在Excel中
8. DomainTable列名修正
9. Domain工作表删除サンプル値列
10. Codelist工作表：C列多匹配用分号分隔Codelist Code，D列改名Value，删除E列和G列
11. Domain工作表删除Origin列（全部为空）
12. 文字多的单元格用多行显示（text_wrap）
