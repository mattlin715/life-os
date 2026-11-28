> 我接受精確工具候選 `0e8e464c2a6057ec557f315571f3e9e0c80f550f`，接受本次僅有 Linux workflow 程序中斷恢復證據、Windows canonical 與跨平台斷電耐久性未驗的限制。
>
> 授權工程在已核對的 `/workspace/r3-governance-source` 執行下列限定操作：
>
> **A｜取消：** 起始 R3 為 `android-pilot-byo-gemini-r1`、sequence 56。重新核對來源、工具、控制面 hashes 與單 writer 後，將本次實際核定記為獨立 disposition 證據，使用官方入口取消本輪 sprint。schema 仍未解、ADR-0013 仍 Proposed，不代表功能完成、PASS、產品方向否決或能力授權。
>
> **B｜封存：** 僅在 A 成功、sequence 57、原事件前綴與取消前 checkpoint 完整核對後，執行一次官方 archive／idle reset。成功只到 idle。
>
> **C｜必要恢復：** 僅限 A／B 本次產生的中斷紀錄。核對精確 operation SHA、前後映像與 current bytes 後，使用官方 forward recovery。未知內容、衝突、不完整 preparing 或來源漂移均保全停止。
>
> 不授權手改 state/events、解除 schema gate、接受 ADR、Git stage／commit／push、merge／release 或啟動 A-F1。
