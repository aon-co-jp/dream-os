# פילוסופיית פיתוח וכללי סביבה (DreamOS / dream-os)

> **הערה**: זהו תרגום מתומצת של המצב הנוכחי. יומן ה-HANDOFF ההיסטורי
> המלא נשאר זמין, מטעמי תמציתיות, רק ביפנית בקובץ
> [CLAUDE.md](CLAUDE.md) — יש לעיין שם לפרטים על כל סשן בנפרד.

כונן העבודה: `F:\runo`. סעיף זה פועל לפי הנוהג של אימוץ הסעיף המקביל
מתוך `CLAUDE.md` של הפרויקט
[`open-raid-z`](https://github.com/aon-co-jp/open-raid-z) כמקור
לחיקוי. מאגר GitHub:
[aon-co-jp/dream-os](https://github.com/aon-co-jp/dream-os).

**תחילת הקונספט: 2026-08-06** (מאגר ה-GitHub הריק עצמו נוצר כבר
ב-2026-07-01, אותו דפוס כמו `open-directx`).

## תפקידו של פרויקט זה (שלב קונספטואלי, רק PoC בודדים בקוד)

**DreamOS** שואף ל**גרעין היברידי של Linux ו-TRON OS** שיאחד את רוב
מערכות ההפעלה הקיימות בעולם — המטרה הסופית היא ש-PowerShell של
Windows, אפליקציות macOS/iPhone ואפליקציות Android יפעלו, כל אחת
תוך שמירה על תאימות, על גבי אותו הגרעין. יעדים עתידיים כוללים תמיכה
ב-Sony PlayStation 5/6 וב-Nintendo Switch 2.

## מחקר טכני (2026-08-06, סיכום כן)

חיפוש ב-Google/GitHub ביפנית ובאנגלית **לא מצא שום מימוש קיים**
המגשים "גרעין היברידי Linux×TRON עם הרצה מקורית (native) של
אפליקציות מכל הפלטפורמות" כפי שתוכנן במקור. הממצא המרכזי: **נכון
ל-2026 אין דרך מעשית להריץ אפליקציות iOS באופן מקורי על חומרה שאינה
של Apple** (Apple עצמה חוסמת ברמת המערכת קומפילציית JIT ווירטואליזציה
לאפליקציות צד שלישי). הגישה של `crosvm` ב-ChromeOS (בסיס VMM משותף
ל-Linux דרך Crostini ול-Android דרך ARCVM) התבררה כדוגמה המבוססת
והמעשית ביותר. seL4 + Genode/Neptune OS ("מיקרו-גרעין + אישיות מערכת
הפעלה") הוא נקודת ייחוס נוספת. לפרטים, כולל ניתוח ייעודי על מחשבי-
העל (מיינפריים) (IBM z/OS / Hercules / AWS Mainframe Modernization),
ראו את הסעיף "技術調査結果" בקובץ [CLAUDE.md](CLAUDE.md).

## פלטפורמות יעד (מפת דרכים, נכון ל-2026-08-17)

**שלב 1 (חומרה קיימת, מתחילים עכשיו)**: Windows (מחשב עם NVIDIA
GT730) ו-Android (מספר סמארטפונים).

**שלב 2 ואילך (נדחה, תלוי ברישיונות עתידיים)**: macOS/iPhone/
**iPad** (iPad נוסף ב-2026-08-17), הפצות Linux/UNIX שונות, Sony
PlayStation 5/6, Nintendo Switch 2/**3** (Switch 3 נוספה
ב-2026-08-17), וכן **תאימות למחשבי-על (מיינפריים) של IBM (z/OS)**.

**גילוי כן**: תמיכה ב-macOS/iPhone/iPad, ב-PS5/6 וב-Switch 2/3
מחייבת SDK לא-ציבוריים, הסכמי סודיות (NDA) וחומרת פיתוח מבעלי
הפלטפורמות בהתאמה — אף אחד מאלה אינו קיים כרגע במערכת האקולוגית
הזו. **נכון ל-2026-08-17, Nintendo לא הכריזה רשמית על Switch 3** —
האזכור כאן הוא רק שומר מקום עבור "אם/כאשר תוכרז", ואינו מבוסס על
מידע אמיתי על המוצר. באשר למחשבי-על של IBM: אמולטור סט הפקודות
Hercules הוא קוד פתוח וניתן לשילוב טכנית, אך הרצת z/OS אמיתית
מחייבת רישיון רשמי מ-IBM (שאינו קיים). "מיינפריים גרסת AWS"
התברר כשירות הגירת אפליקציות (AWS Mainframe Modernization / AWS
Transform for mainframe), ולא שכבת תאימות של מערכת הפעלה.

## PoC שמומשו (מאומתים על חומרה אמיתית, Windows GT730 + Android
Adreno 619)

- **`crates/dream-os-kernel`** (עושה שימוש חוזר ב-`opencuda-vulkan`
  של `open-cuda`): בסיס הרצה משותף של Vulkan עבור Windows/Android
  (גרעין `vector_add`), בקרת ביצועים בסגנון mining-OS
  (`MiningPowerProfile` + גרעין `sha256d_mine` ל-double-SHA256,
  תוצאות ה-GPU אומתו בשתי הפלטפורמות מול הפניה על CPU), אופטימיזציה
  קומבינטורית בהשראת ה-SBM של Toshiba (גרעין `sbm_ising`, אלגוריתם
  SB בליסטי, GPU ו-CPU מתכנסים לאותה קונפיגורציית ספינים), יישום
  קונקרטי של `sbm_ising` לבעיית התרמיל (בחירת משימות,
  `task_scheduler.rs`), גשר DXBC→SPIR-V עבור `open-directx`
  (`directx_bridge.rs`), וכן גשר ל-flash-attention המאוחדת (fused)
  של `open-cuda` (`flash_attention_bridge.rs`, בגודל צעצוע 8×16).
- **`crates/dream-os-raid-bridge`** (עושה שימוש חוזר ב-`RaidZVdev`
  ו-`zfs_accel_hlsl` של `open-raid-z`): גשר להאצת זוגיות (parity)
  RAID6/Z2, שמאמת האצת GPU אמיתית (ללא נפילה חזרה ל-CPU), סבב הלוך-
  ושוב (roundtrip) על פני 8 פסים (stripes) ותיקון-עצמי לאחר השחתה
  מדומה של דיסק, בתוספת הדגמה שניתן להריץ בפקודה אחת.
- **`crates/dream-os-wire`** (עושה שימוש חוזר ב-`SecureChannel` של
  `open-web-server-wire` וב-`redundant_transmission`/
  `transaction_log` של `RS-SmartTCP`): העברת תוצאות מוצפנת AEAD,
  עמידה בפני replay וזיוף, עבור הקונספט של World Laboratory
  (מחשוב מבוזר התנדבותי), בתוספת התמדה (persistence) תואמת ACID
  של `aruaru-db`, שאומתה מול מופע אמיתי של `aruaru-server`.

## מיקום במערכת האקולוגית

קשר ל-[`open-directx`](https://github.com/aon-co-jp/open-directx)
(שכבת תאימות DirectX חוצת-פלטפורמות, כרגע בעדיפות הגבוהה ביותר
במערכת האקולוגית), [`open-cuda`](https://github.com/aon-co-jp/open-cuda)
(חישוב GPU תואם CUDA, backend Vulkan אמיתי) ו-
[`aruaru-llm`](https://github.com/aon-co-jp/aruaru-llm) (הסקת LLM
דרך גרעיני ה-Vulkan של `open-cuda`). אופן האינטגרציה המדויק עדיין
פתוח — `dream-os` מפותח במכוון בצעדים קטנים וניתנים לאימות באופן
עצמאי, בעוד השלמת `open-directx` נותרת בעדיפות.

## מצב נוכחי

ה-PoC שלעיל אומתו על חומרה אמיתית, אך DreamOS עצמו (הגרעין המאוחד,
מתאם ה-World Laboratory) עדיין נמצא בשלב הקונספט/העיצוב. ליומן
ה-HANDOFF ההיסטורי המלא ראו [CLAUDE.md](CLAUDE.md).

---

שפות נוספות: [日本語 (המקור, עם היסטוריית HANDOFF מלאה)](CLAUDE.md) ·
[English](CLAUDE-English.md) · [Deutsch](CLAUDE-German.md) ·
[Italiano](CLAUDE-Italian.md) · [Français](CLAUDE-French.md) ·
[Русский](CLAUDE-Russian.md) · [Українська](CLAUDE-Ukrainian.md) ·
[فارسی](CLAUDE-Persian.md) · [العربية](CLAUDE-Arabic.md)
