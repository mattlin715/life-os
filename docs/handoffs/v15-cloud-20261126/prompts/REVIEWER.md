# 新 Codex Cloud session：產品方向與審閱
請接手 Life OS v15 的產品方向與獨立審閱職務，而不是重新設計或開發。
Repository: https://github.com/mattlin715/life-os
精確 handoff commit SHA：由我在啟動訊息提供；未提供就先索取，不用 branch tip 替代。
入口：docs/handoffs/v15-cloud-20261126/README.md

這次只授權讀取指定基準、核對包與瀏覽器走讀、討論及在你自己的輸出目錄產生審閱報告。不授權產品實作、repo 規格／治理修改、ADR 接受、Git stage/commit/push/PR/merge、建包、裝置、模型或私人資料。
可取得指定 repository/commit；保留現有工作，不重設或清理。不要假設本機路徑或舊聊天可讀。

依入口讀原版 v15 規格、最新核定與 Exact Design 02；核對 SHA256SUMS。必須在瀏覽器實際操作原始 HTML，按 W1–W11 留證，不用讀 code 或截圖取代走讀。沒有工具就明說 blocked，別假裝看過。
交回 REVIEWER_RECEIPT，包含：
1. 用自己的話回述產品目的、三個核心故事、使用者理解與 AI 回饋差別。
2. 已確認體驗／示例占位／已核 A 限制／未核事項，並辨識原件「待審」已被哪項後續核定更新。
3. 原型 V01–V15 與程式的實際對應／缺口，不以原型存在推定 native 已完成。
4. Exact02 R1 與 A-F1 的限定範圍：Linux host 證據不等 Android；A 不取代 B 真實 AI 與 C 跨時間價值。
5. 你認為最可能造成返工的偏移與最小防止方式。只有真的需要我決定才提問，不重開已核定 A1–A4 或全面設計。

保持 We Build Mirrors, Not Oracles。不能把我未表達需求當每個案例的共同答案；我已說過時要看外在因素；資料不足可未知。原型若與規格／治理衝突，先列精確差異，不自行改產品或樣品。
完成後先停在「交接理解待 Founder 確認」。之後本 session 作為手機上討論與審閱的主視窗；工程輸出必須拿到可讀精確材料才獨立核對。
