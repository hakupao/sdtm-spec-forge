# 第1轮问答

## 补充背景（已确认）

这些SDTM数据集是从EDC导出的医疗原始数据（rawdata），经过理解原始仕様書并根据SDTMIG映射制作而成。
仕様書的目的不仅是罗列元数据，而是要：
- 帮助客户**理解数据的整体结构和逻辑**
- 让客户能够**想象到数据原貌**（EDC中的数据长什么样）
- 体现域内的**树状分类结构**（CAT → SCAT → 更深层的分类逻辑）
- **宏观和微观层面**都有描述

### 数据中已确认的层级结构示例

**RS域**（树状结构非常典型）：
```
RSTESTCD=CLINRESP
  └─ RSCAT=CLINICAL UICCV8
       ├─ RSSCAT=T_CATEGORY
       ├─ RSSCAT=N_CATEGORY
       └─ RSSCAT=M_CATEGORY
  └─ RSCAT=PROTOCOL DEFINED RESPONSE CRITERIA
       └─ RSSCAT=CURABILITY → RSCAT1=Memorial Sloan Kettering Regression Schema
```

**CM域**：CAT=POST TREATMENT → SCAT=CHEMOTHERAPY MEDICATION / TARGETED MEDICATION

**PR域**：CAT = ENDOSCOPY / IMAGING / SURGERY / PHYSICAL EXAMINATION

---

请在每个问题的「回答:」处填写你的想法。

---

## Q1: Excel Tab构成

考虑到「宏观+微观」的需求，我提议以下Tab构成：

| Tab | 内容 | 层次 |
|-----|------|------|
| **数据集一览** | 全域概要（域名、记录数、结构说明等） | 宏观 |
| **域结构概览** | 各域内的CAT/SCAT树状分类关系图（文字表现） | 宏观→中观 |
| **XX变量一览**（每域1个Tab） | 各域的变量详细元数据 | 微观 |
| **代码列表一览** | 从数据中提取的Codelist | 微观 |
| **备注** | 特殊处理、映射说明等 | 补充 |

关键新增：**域结构概览Tab**，用来展示每个域内的逻辑分类树。
这个构成你觉得如何？有没有想增减的Tab？

**回答:**
我觉得没有问题，我其实还有一个SDTM_ENSEMBLE文件夹，里面放了从原始数据SDTM_ENSEMBLE\studySpecific\ENSEMBLE\01_RawData，经过脚本程序，读取SDTM_ENSEMBLE\studySpecific\ENSEMBLE\ENSEMBLE_OperationConf.xlsx的映射规则文件，然后一步步生成数据的过程。

---

## Q2: 数据集一览Tab的列

| 列名 | 说明 | 示例 |
|------|------|------|
| No. | 序号 | 1 |
| Domain | 域代码 | RS |
| Dataset Name | 数据集名称（英文） | Disease Response and Clin Classification |
| 数据概要说明 | 该域数据的概要描述（日文） | 腫瘍評価・臨床分類に関するデータ |
| Structure | 数据结构 | One record per subject per test per visit |
| Keys | 排序键 | STUDYID, USUBJID, RSTESTCD, VISITNUM |
| 变量数 | 变量数 | 47 |
| 记录数 | 记录数 | 350 |
| 分类层级 | 该域的主要分类维度 | TESTCD → CAT → SCAT → CAT1 |
| 备注 | 补充说明 | （手动填写用） |

新增了「数据概要说明」和「分类层级」列。你觉得这些列够用吗？有需要增减的吗？

**回答:**
暂时够用，如果后续有需要再调整。

---

## Q3: 域结构概览Tab的展现方式

这是本工具的核心差异化功能。有以下几种方案：

**A方案 - 缩进文字树**（每个域一个区块）
```
【RS - Disease Response and Clin Classification】

RSTESTCD=CLINRESP (Clinical Response)
  ├─ RSCAT=CLINICAL UICCV8
  │    ├─ RSSCAT=T_CATEGORY
  │    ├─ RSSCAT=N_CATEGORY
  │    └─ RSSCAT=M_CATEGORY
  └─ RSCAT=PROTOCOL DEFINED RESPONSE CRITERIA
       └─ RSSCAT=CURABILITY
            └─ RSCAT1=Memorial Sloan Kettering Regression Schema
```

**B方案 - 表格展开**（每行一个叶节点路径）

| Domain | TESTCD | TEST | CAT | SCAT | CAT1 | 记录数 |
|--------|--------|------|-----|------|------|--------|
| RS | CLINRESP | Clinical Response | CLINICAL UICCV8 | T_CATEGORY | | 50 |
| RS | CLINRESP | Clinical Response | CLINICAL UICCV8 | N_CATEGORY | | 50 |

**C方案 - A+B混合**（树状图放在一个Tab，表格展开放在另一个Tab）

你倾向哪种方案？或者有其他想法？

**回答:**
C方案比较好，树状图和表格可以互相印证，满足不同用户的阅读习惯。

---

## Q4: 变量一览Tab的列

| 列名 | 说明 | 数据来源 |
|------|------|----------|
| No. | 序号 | 自动生成 |
| Variable Name | 变量名 | CSV表头 |
| Variable Label | 变量标签 | SDTMIG v3.4元数据 |
| Type | 数据类型（Char/Num） | 从数据推定 |
| Length | 最大长度 | 从数据计算 |
| Core | Req/Exp/Perm | SDTMIG v3.4元数据 |
| Origin | 数据来源 | SDTMIG v3.4元数据 |
| Role | 变量角色 | SDTMIG v3.4元数据 |
| Codelist | 关联Codelist名 | SDTMIG v3.4元数据 + 数据检测 |
| Format | 显示格式 | 从数据推定 |
| 数据说明 | 该变量在本study中的具体含义/用途（日文） | 空栏，手动补充用 |
| 样本值 | 数据中的典型值（前几个唯一值） | 从数据提取 |
| 备注 | 补充说明 | 空栏，手动补充用 |

新增了「数据说明」列，方便你手动补充EDC原始含义。你觉得这些列够用吗？

**回答:**
可以的

---

## Q5: SDTMIG v3.4元数据的获取方式

变量的Label、Core、Role等官方元数据是必须的。方案如下：

- **A方案**: 从CDISC官方SDTMIG v3.4 PDF手动整理元数据为JSON/CSV，内置到工具中
- **B方案**: 你已有SDTMIG v3.4的元数据文件（Excel等），作为工具输入
- **C方案**: 工具内硬编码SDTMIG v3.4主要域的元数据

你倾向哪种？有没有现成的元数据文件？

**回答:**
我把我手头的有关 SDTM的资料都放到 SDTM_Master 文件夹了。
你读取一下，然后可以继续向我提问

---

## Q6: 代码列表的输出粒度

- **A方案**: 域名 + 变量名 + 全部唯一值列表
- **B方案**: A方案 + CDISC CT对应关系
- **C方案**: 仅唯一值列表（不含Codelist名称）

你倾向哪种？

**回答:**
B方案比较好，可以提供更全面的信息。

---

## Q7: Excel格式和样式

- 表头配色（蓝色背景+白色文字等）？
- 列宽自动调整？
- 冻结窗格（固定表头行）？
- 树状结构的缩进用合并单元格还是缩进文字？
- 其他格式要求？

**回答:**
表头配色、列宽自动调整、冻结窗格都是不错的功能。树状结构的缩进建议使用缩进文字，这样更便于阅读和理解。

---

## Q8: 将来的扩展性

- XPT文件输入支持？
- define.xml生成？
- 多Study批量处理？
- 与Pinnacle 21的衔接（例如输出P21兼容的Excel格式）？

**回答:**
目前暂时不考虑

---

## Q9: 工具执行方式

- CLI（命令行）是否足够？
- 是否需要TOML/YAML配置文件？
- 是否需要指定哪些域要处理（还是默认处理全部）？

**回答:**
暂时不考虑实现方法，先把需求确认好再说。
