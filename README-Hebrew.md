# DreamOS (dream-os)

*שפות נוספות*: [日本語 (המקור)](README.md) ·
[English](README-English.md) · [Deutsch](README-German.md) ·
[Italiano](README-Italian.md) · [Français](README-French.md) ·
[Русский](README-Russian.md) · [Українська](README-Ukrainian.md) ·
[فارسی](README-Persian.md) · [العربية](README-Arabic.md)

> 📌 **עדכון אחרון (2026-08-08)**: `dream-os-raid-bridge` (גשר
> RAID6/Z2 של open-raid-z) קיבל הדגמה הניתנת להרצה
> (`examples/raid6_bridge_benchmark.rs`), כך שניתן לבדוק אותה
> בפקודה אחת (`cargo run -p dream-os-raid-bridge --example
> raid6_bridge_benchmark --release`) בלי לקרוא את קוד המקור של
> הבדיקות. אומת על חומרת NVIDIA GT730 אמיתית: מאיץ הזוגיות שזוהה
> הוא אכן `Gpu` (ללא נפילה חזרה ל-CPU), סבב הלוך-ושוב של כתיבה/קריאה
> על פני 8 פסים תואם, והגשר מתקן את עצמו לאחר השחתה ישירה של דיסק
> (שני דיסקי הזוגיות של RAID6 משחזרים אותו). נבדק גם
> `dream-os-kernel/src/lib.rs` לאיתור סימוני
> TODO/unimplemented/`allow(dead_code)` — לא נמצא אף אחד, ולכן לא
> נדרש תיקון (גילוי כן, ללא טענה לבדיקה ממצה).

> 📌 **עדכון אחרון (2026-08-07)**: נוספה אפליקציית Android חדשה
> (`android/`) המדגימה PoC של אינטגרציה בין open-cuda (פתיחת התקן
> Vulkan) ל-open-directx (תרגום DXBC→SPIR-V), שאומתה על חומרה
> אמיתית (Windows GT730 + Android Adreno 619). תוכנית האינטגרציה
> המתוכננת עם Toshiba SBM/DeepSeek נותרת דחויה — עדיין לא זוהתה
> מטרת אופטימיזציה קונקרטית לכל מאגר, ולכן לא בוצע מימוש ספקולטיבי
> (ראו [CLAUDE.md](CLAUDE.md) לדיון הכן). כמו כן תוקנה שגיאת תיעוד
> קודמת: ל-`sbm_ising` כבר יש נתיב dispatch של GPU דרך
> `opencuda_core::GpuDevice`, שאושר באמצעות `cargo test`.

**פרויקט זה נמצא בשלב קונספטואלי, אך החל מ-2026-08-06 הוא כולל
מספר proof-of-concept שאומתו על חומרה אמיתית.** קובץ
`README-Hebrew.md` זה — יחד עם [`README.md`](README.md) (יפנית,
השפה העיקרית של מערכת אקולוגית זו), [`CLAUDE.md`](CLAUDE.md),
[`PORTING.md`](PORTING.md) וכן
[world-lab/docs/world-laboratory-design.md](https://github.com/aon-co-jp/world-lab/blob/main/docs/world-laboratory-design.md)
— מתעד את המטרה, ההיקף, המיקום במערכת האקולוגית ומצב המימוש
הנוכחי של פרויקט זה.

## מטרה

**גרעין היברידי של Linux ו-TRON OS**, השואף לאחד את רוב מערכות
ההפעלה הקיימות בעולם. המטרה הסופית היא ש-PowerShell של Windows,
אפליקציות macOS/iPhone ואפליקציות Android יפעלו, כל אחת תוך שמירה
על תאימות, על גבי גרעין אחד. יעדים עתידיים כוללים גם תמיכה ב-**Sony
PlayStation 5 / PlayStation 6 וב-Nintendo Switch 2** (כיוון שנקבע
בהוראת המשתמש מ-2026-08-06).

## מחקר טכני (בוצע ב-2026-08-06, סיכום כן)

חיפוש ב-Google/GitHub ביפנית ובאנגלית **לא מצא שום מימוש קיים**
המגשים "גרעין היברידי Linux×TRON עם הרצה מקורית (native) של
אפליקציות מכל הפלטפורמות" כפי שתוכנן במקור. הממצא המרכזי: **נכון
ל-2026 אין דרך מעשית להריץ אפליקציות iOS באופן מקורי על חומרה שאינה
של Apple** (Apple עצמה חוסמת ברמת מערכת ההפעלה קומפילציית JIT
ווירטואליזציה לאפליקציות צד שלישי). מאידך, הגישה של ChromeOS —
אירוח מספר מערכות הפעלה אורחות (Linux דרך Crostini, Android דרך
ARCVM) על בסיס VMM משותף (`crosvm`) — התבררה כדוגמה המבוססת
והמעשית ביותר. לפירוט מלא לפי טכנולוגיית רכיב, השוואת בגרות
והצעה לכיוון ארכיטקטוני מציאותי, ראו את הסעיף "技術調査結果"
(תוצאות המחקר הטכני) בקובץ [`CLAUDE.md`](CLAUDE.md).

## פלטפורמות יעד (מפת דרכים, עודכנה ב-2026-08-06 על בסיס החומרה
הזמינה בפועל)

**שלב 1 (חומרה קיימת, מתחילים עכשיו)**: Windows (מחשב עם NVIDIA
GT730) ו-Android (מספר סמארטפונים).

**שלב 2 ואילך (נדחה, תלוי ברישיונות/אישורים עתידיים)**: macOS/
iPhone/**iPad** (נוסף ב-2026-08-17), הפצות Linux/UNIX שונות, Sony
PlayStation 5/6, Nintendo Switch 2/**3** (נוספה ב-2026-08-17), וכן
**תאימות למחשבי-על (מיינפריים) של IBM (z/OS)** (אמולטור סט
הפקודות Hercules הוא קוד פתוח, אך הרצת מערכת ההפעלה האמיתית
מחייבת רישיון רשמי מ-IBM). "מיינפריים גרסת AWS" התברר כשירות
הגירת אפליקציות (AWS Mainframe Modernization וכדומה), ולא שכבת
תאימות של מערכת הפעלה — פרטים בקובץ [`CLAUDE.md`](CLAUDE.md).

**גילוי כן**: תמיכה ב-macOS/iPhone/iPad, ב-PS5/6 וב-Switch 2/3
מחייבת SDK לא-ציבוריים, הסכמי סודיות (NDA) וחומרת פיתוח מבעלי
הפלטפורמות בהתאמה, שכולם נעדרים כרגע לחלוטין ממערכת האקולוגית
הזו. **נכון ל-2026-08-17, Nintendo לא הכריזה רשמית על Switch 3 —
האזכור כאן הוא רק שומר מקום עבור "אם/כאשר תוכרז ותושק", ואינו
מבוסס על מידע אמיתי על המוצר** (מנוסח בבירור כדי להימנע מטענות
מוגזמות). בהתאם להחלטת המשתמש מ-2026-08-06, פלטפורמות אלה נדחות
בהנחת קבלת רישיון בעתיד — העבודה מתחילה כעת בפלטפורמות שעבורן קיימת
בפועל חומרה (פרטים בסעיף "スコープの絞り込み" בקובץ
[`CLAUDE.md`](CLAUDE.md)).

## PoC שמומשו (2026-08-06, אומתו על חומרה אמיתית)

`crates/dream-os-kernel` (עושה שימוש חוזר ב-`opencuda-vulkan` של
`open-cuda`):

- **בסיס הרצה משותף של Vulkan עבור Windows/Android**: הגרעין
  `vector_add` הורץ בפועל ואומת הן על מחשב Windows (NVIDIA GT730)
  והן על סמארטפון Android אמיתי (Moto G53Y 5G, Adreno 619).
- **בקרת ביצועים בסגנון mining-OS**: `MiningPowerProfile` (הגבלת
  duty-cycle בצד התוכנה) + גרעין `sha256d_mine` (double-SHA256).
  תוצאות ה-GPU אומתו בית-בית מול הפניה על CPU (חבילת `sha2`) בשתי
  הפלטפורמות. נוסף עיצוב פיצול לאצוות כדי לעקוף פסק זמן (TDR) של
  מנהל התקן GPU נייד שהתגלה בהתקן Android אמיתי.
- **אופטימיזציה קומבינטורית מסוג quantum-annealing בהשראת
  Simulated Bifurcation Machine (SBM) של Toshiba**: גרעין
  `sbm_ising` (אלגוריתם SB בליסטי, מזעור מודל Ising). אומת על חומרת
  Windows אמיתית שההרצה על GPU מתכנסת בדיוק לאותה קונפיגורציית
  ספינים כמו מימוש ההפניה על CPU.

`crates/dream-os-wire` (עושה שימוש חוזר ב-`SecureChannel` של
`open-web-server-wire`):

- **הגנה על העברת התוצאות עבור הקונספט של World Laboratory**
  (מחשוב מבוזר התנדבותי בסגנון BOINC): הצפנת AEAD + הגנה מפני
  התקפות replay דרך `WorkerChannel`/`CoordinatorChannel`, המזהים
  ודוחים תוצאות worker מזויפות או חוזרות. אומת בשלושה תרחישי בדיקה
  קריפטוגרפיים אמיתיים (סבב הלוך-ושוב רגיל, דחיית replay, דחיית
  זיוף). לעיצוב המלא ראו
  [world-lab/docs/world-laboratory-design.md](https://github.com/aon-co-jp/world-lab/blob/main/docs/world-laboratory-design.md).

## מיקום במערכת האקולוגית

חלק ממערכת האקולוגית `aon-co-jp`, ובפרט מקושר אל (אופן האינטגרציה
המדויק עדיין פתוח — פרטים בקובץ [`CLAUDE.md`](CLAUDE.md)):

- [`open-directx`](https://github.com/aon-co-jp/open-directx) —
  שכבת תאימות DirectX חוצת-פלטפורמות המאפשרת לאפליקציות/משחקי
  DirectX קיימים, שהיו זמינים בעבר רק ל-Windows, לפעול על Linux,
  Android, ובעתיד macOS/PlayStation. **כיום הפרויקט הנרדף בעדיפות
  הגבוהה ביותר במערכת האקולוגית מבחינת השלמתו.**
- [`open-cuda`](https://github.com/aon-co-jp/open-cuda) — בסיס
  חישוב GPU תואם CUDA (backend Vulkan אמיתי).
- [`aruaru-llm`](https://github.com/aon-co-jp/aruaru-llm) — הסקת
  LLM, המשתמשת בגרעיני ה-Vulkan של `open-cuda`.

## מצב נוכחי

ה-PoC שלעיל אומתו על חומרה אמיתית, אך DreamOS עצמו (הגרעין המאוחד,
המתאם) עדיין נמצא בשלב הקונספט/העיצוב. השלמת open-directx נותרת
העדיפות הגבוהה ביותר, בעוד `dream-os` גדל לצדו בצעדים קטנים
הניתנים לאימות באופן עצמאי (פרטים בסעיף HANDOFF בקובץ
[`CLAUDE.md`](CLAUDE.md)).

## פרויקטים קשורים

- [aon-co-jp/open-directx](https://github.com/aon-co-jp/open-directx)
- [aon-co-jp/open-cuda](https://github.com/aon-co-jp/open-cuda)
- [aon-co-jp/aruaru-llm](https://github.com/aon-co-jp/aruaru-llm)
- [aon-co-jp/RUNO](https://github.com/aon-co-jp/RUNO) — האינדקס-העל
  של מערכת האקולוגית כולה

## License

טרם הוחלט (שלב קונספטואלי).
