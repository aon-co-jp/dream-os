# فلسفة التطوير وقواعد بيئة العمل (DreamOS / dream-os)

> **ملاحظة**: هذه ترجمة مختصرة للحالة الراهنة. يبقى سجل HANDOFF
> التاريخي الكامل متاحاً، للإيجاز، باليابانية فقط في
> [CLAUDE.md](CLAUDE.md) — راجعه هناك لتفاصيل كل جلسة على حدة.

قرص العمل: `F:\runo`. يتّبع هذا القسم ممارسة اعتماد القسم المقابل من
`CLAUDE.md` الخاص بمشروع
[`open-raid-z`](https://github.com/aon-co-jp/open-raid-z) كمرجع.
مستودع GitHub:
[aon-co-jp/dream-os](https://github.com/aon-co-jp/dream-os).

**بداية المفهوم: 2026-08-06** (أُنشئ مستودع GitHub الفارغ نفسه بالفعل
في 2026-07-01، نفس نمط `open-directx`).

## دور هذا المشروع (مرحلة المفهوم، إثباتات مفهوم فردية فقط كأكواد)

يهدف **DreamOS** إلى **نواة هجينة تجمع بين Linux ونظام TRON** تُوحّد
معظم أنظمة التشغيل الموجودة في العالم — من المفترض أن يعمل PowerShell
الخاص بـ Windows وتطبيقات macOS/iPhone وتطبيقات Android كلٌّ منها
بتوافقية محفوظة على نواة واحدة. تشمل الأهداف المستقبلية دعم
Sony PlayStation 5/6 وNintendo Switch 2.

## البحث التقني (2026-08-06، ملخص صادق)

لم يعثر بحث في Google/GitHub باليابانية والإنجليزية على **أي تنفيذ
قائم** يحقق "نواة هجينة Linux×TRON تشغّل تطبيقات كل المنصات بشكل
أصلي" كما كان متصوَّراً في الأصل. أهم اكتشاف: **لا توجد حتى عام 2026
طريقة عملية لتشغيل تطبيقات iOS بشكل أصلي على عتاد ليس من إنتاج Apple**
(تحظر Apple نفسها تجميع JIT والمحاكاة الافتراضية للتطبيقات من طرف
ثالث على مستوى النظام بأكمله). تبيّن أن نهج `crosvm` الخاص بـ ChromeOS
(أساس VMM مشترك لتشغيل Linux عبر Crostini وAndroid عبر ARCVM) هو
السابقة الأكثر رسوخاً وواقعية. يُعدّ seL4 مع Genode/Neptune OS
("نواة دقيقة + شخصية نظام تشغيل") مرجعاً آخر. للتفاصيل، بما فيها
تحليل أصلي للحواسيب المركزية (IBM z/OS / Hercules / AWS Mainframe
Modernization)، راجع قسم "技術調査結果" في [CLAUDE.md](CLAUDE.md).

## المنصات المستهدفة (خارطة الطريق، حتى 2026-08-17)

**المرحلة 1 (العتاد متوفر، البدء الآن)**: Windows (جهاز بمعالج رسوم
NVIDIA GT730) وAndroid (عدة هواتف).

**المرحلة 2 وما بعدها (مؤجَّلة، رهناً بالترخيص المستقبلي)**: macOS/
iPhone/**iPad** (أُضيف iPad في 2026-08-17)، توزيعات Linux/UNIX
المتنوعة، Sony PlayStation 5/6، Nintendo Switch 2/**3** (أُضيف
Switch 3 في 2026-08-17)، و**توافق حواسيب IBM المركزية (z/OS)**.

**إفصاح صادق**: يتطلب دعم macOS/iPhone/iPad وPS5/6 وSwitch 2/3 حزم
SDK غير عامة واتفاقيات عدم إفصاح وعتاد تطوير من كل مالك منصة — لا شيء
من ذلك متوفر حالياً في هذه المنظومة. **لم تُعلن Nintendo رسمياً عن
Switch 3 حتى 2026-08-17** — ذكره هنا مجرد عنصر نائب لـ"إن/عندما
يُعلَن"، وليس مبنياً على معلومات منتج حقيقية. أما بالنسبة لحواسيب IBM
المركزية: محاكي مجموعة التعليمات Hercules مفتوح المصدر وقابل للدمج
تقنياً، لكن تشغيل z/OS الحقيقي يتطلب ترخيصاً رسمياً من IBM (غير
متوفر). تبيّن أن "الحاسوب المركزي بنسخة AWS" هو خدمة ترحيل تطبيقات
(AWS Mainframe Modernization / AWS Transform for mainframe)، وليس
طبقة توافق نظام تشغيل.

## إثباتات المفهوم المنفَّذة (تم التحقق منها على عتاد حقيقي، Windows
GT730 + أندرويد Adreno 619)

- **`crates/dream-os-kernel`** (يعيد استخدام `opencuda-vulkan` من
  `open-cuda`): أساس تنفيذ Vulkan مشترك بين Windows/Android (نواة
  `vector_add`)، تحكم في الطاقة على غرار أنظمة التعدين
  (`MiningPowerProfile` + نواة `sha256d_mine` لـ SHA256 المزدوج، تم
  التحقق من نتائج GPU مقابل مرجع المعالج على المنصتين)، تحسين تركيبي
  مستوحى من آلة SBM الخاصة بتوشيبا (نواة `sbm_ising`، خوارزمية SB
  البالستية، يتقارب GPU/CPU إلى نفس تشكيلة السبينات)، تطبيق ملموس
  لمسألة الحقيبة باستخدام `sbm_ising` (اختيار المهام،
  `task_scheduler.rs`)، جسر ترجمة DXBC→SPIR-V من `open-directx`
  (`directx_bridge.rs`)، وجسر انتباه فلاش مدمج من `open-cuda`
  (`flash_attention_bridge.rs`، بحجم تجريبي 8×16).
- **`crates/dream-os-raid-bridge`** (يعيد استخدام `RaidZVdev` +
  `zfs_accel_hlsl` من `open-raid-z`): جسر تسريع تكافؤ RAID6/Z2، تم
  التحقق من تسريع GPU حقيقي (بلا رجوع للمعالج)، دورة كتابة/قراءة عبر
  8 شرائح، وشفاء ذاتي بعد إتلاف قرص واحد محاكياً، بالإضافة إلى عرض
  توضيحي يعمل بأمر واحد.
- **`crates/dream-os-wire`** (يعيد استخدام `SecureChannel` من
  `open-web-server-wire` وكذلك `redundant_transmission`/
  `transaction_log` من `RS-SmartTCP`): تسليم نتائج مشفَّر بـ AEAD
  ومقاوم لإعادة الإرسال والتلاعب لمفهوم World Laboratory (حوسبة
  موزعة تطوعية)، بالإضافة إلى استمرارية عبر `aruaru-db` (متوافق مع
  ACID)، تم التحقق منها مقابل نسخة حقيقية من `aruaru-server`.

## الموقع داخل المنظومة

مرتبط بـ [`open-directx`](https://github.com/aon-co-jp/open-directx)
(طبقة توافق DirectX عبر المنصات، حالياً المشروع ذو الأولوية القصوى في
المنظومة)، و[`open-cuda`](https://github.com/aon-co-jp/open-cuda)
(حساب GPU متوافق مع CUDA، خلفية Vulkan حقيقية)، و
[`aruaru-llm`](https://github.com/aon-co-jp/aruaru-llm) (استدلال
نماذج اللغة الكبيرة عبر نويات Vulkan من `open-cuda`). طريقة التكامل
الدقيقة لم تُحسَم بعد — يُنمَّى `dream-os` عمداً بخطوات صغيرة قابلة
للتحقق بشكل مستقل بينما يبقى إكمال `open-directx` الأولوية.

## الحالة الراهنة

إثباتات المفهوم أعلاه تم التحقق منها على عتاد حقيقي، لكن DreamOS نفسه
(النواة الموحَّدة، منسِّق World Laboratory) لا يزال في مرحلة
المفهوم/التصميم. راجع [CLAUDE.md](CLAUDE.md) للاطلاع على سجل HANDOFF
التاريخي الكامل.

---

لغات أخرى: [日本語 (الأصل، مع تاريخ HANDOFF كامل)](CLAUDE.md) ·
[English](CLAUDE-English.md) · [Deutsch](CLAUDE-German.md) ·
[Italiano](CLAUDE-Italian.md) · [Français](CLAUDE-French.md) ·
[Русский](CLAUDE-Russian.md) · [Українська](CLAUDE-Ukrainian.md) ·
[עברית](CLAUDE-Hebrew.md) · [فارسی](CLAUDE-Persian.md)
