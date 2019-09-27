use typed_igo::conjugation::*;

pub fn get_suffixes(kind: ConjugationKind, form: ConjugationForm) -> &'static [&'static str] {
    use typed_igo::conjugation::ConjugationForm::*;
    use typed_igo::conjugation::ConjugationKind::*;
    match (kind, form) {
        (KahenKuru, ImperativeI) => &["こい"],
        (KahenKuru, Basic) => &["くる"],
        (KahenKuru, NegativeU) => &["こよ"],
        (KahenKuru, Continuous) => &["き"],
        (KahenKuru, ConditionalContraction1) => &["くりゃ"],
        (KahenKuru, SpecialAttributive) => &["くん"],
        (KahenKuru, SpecialAttributive2) => &["く"],
        (KahenKuru, Conditional) => &["くれ"],
        (KahenKuru, Negative) => &["こ"],
        (KahenKuru, ImperativeYo) => &["こよ"],
        (KahenKuruKanji, SpecialAttributive) => &["来ん"],
        (KahenKuruKanji, NegativeU) => &["来よ"],
        (KahenKuruKanji, Negative) => &["来"],
        (KahenKuruKanji, Conditional) => &["来れ"],
        (KahenKuruKanji, ImperativeYo) => &["来よ"],
        (KahenKuruKanji, Basic) => &["来る"],
        (KahenKuruKanji, ConditionalContraction1) => &["来りゃ"],
        (KahenKuruKanji, SpecialAttributive2) => &["来"],
        (KahenKuruKanji, ImperativeI) => &["来い"],
        (KahenKuruKanji, Continuous) => &["来"],
        (SahenSuru, ConditionalContraction1) => &["すりゃ"],
        (SahenSuru, SpecialAttributive) => &["すん"],
        (SahenSuru, SpecialAttributive2) => &["す"],
        (SahenSuru, NegativeNu) => &["せ"],
        (SahenSuru, NegativeU) => &["しよ", "しょ"],
        (SahenSuru, OldBasic) => &["す"],
        (SahenSuru, Negative) => &["し"],
        (SahenSuru, Conditional) => &["すれ"],
        (SahenSuru, ImperativeYo) => &["せよ"],
        (SahenSuru, NegativeReru) => &["さ"],
        (SahenSuru, ImperativeRo) => &["しろ"],
        (SahenSuru, Continuous) => &["し"],
        (SahenSuru, Basic) => &["する"],
        (SahenSuru, ImperativeI) => &["せい"],
        (SahenSuruConnected, Negative) => &["し"],
        (SahenSuruConnected, ConditionalContraction1) => &["すりゃ"],
        (SahenSuruConnected, Conditional) => &["すれ"],
        (SahenSuruConnected, Basic) => &["する"],
        (SahenSuruConnected, OldBasic) => &["す"],
        (SahenSuruConnected, NegativeU) => &["しよ", "しょ"],
        (SahenSuruConnected, ImperativeYo) => &["せよ"],
        (SahenSuruConnected, ImperativeRo) => &["しろ"],
        (SahenSuruConnected, NegativeReru) => &["せ"],
        (SahenZuruConnected, Conditional) => &["ずれ"],
        (SahenZuruConnected, NegativeU) => &["ぜよ"],
        (SahenZuruConnected, OldBasic) => &["ず"],
        (SahenZuruConnected, ConditionalContraction1) => &["ずりゃ"],
        (SahenZuruConnected, ImperativeYo) => &["ぜよ"],
        (SahenZuruConnected, Basic) => &["ずる"],
        (SahenZuruConnected, Negative) => &["ぜ"],
        (Ichidan, Negative) => &[""],
        (Ichidan, Continuous) => &[""],
        (Ichidan, NegativeU) => &["よ"],
        (Ichidan, ConditionalContraction1) => &["りゃ"],
        (Ichidan, SpecialAttributive) => &["ん"],
        (Ichidan, ImperativeYo) => &["よ"],
        (Ichidan, Basic) => &["る"],
        (Ichidan, Conditional) => &["れ"],
        (Ichidan, ImperativeRo) => &["ろ"],
        (IchidanYameru, Basic) => &["る"],
        (GodanKaIonbin, Conditional) => &["け"],
        (GodanKaIonbin, Negative) => &["か"],
        (GodanKaIonbin, ConditionalContraction1) => &["きゃ"],
        (GodanKaIonbin, NegativeU) => &["こ"],
        (GodanKaIonbin, ContinuousTa) => &["い"],
        (GodanKaIonbin, Continuous) => &["き"],
        (GodanKaIonbin, ImperativeE) => &["け"],
        (GodanKaIonbin, Basic) => &["く"],
        (GodanKaSokuonbin, ImperativeE) => &["け"],
        (GodanKaSokuonbin, Negative) => &["か"],
        (GodanKaSokuonbin, NegativeU) => &["こ"],
        (GodanKaSokuonbin, Conditional) => &["け"],
        (GodanKaSokuonbin, Continuous) => &["き"],
        (GodanKaSokuonbin, ContinuousTa) => &["っ"],
        (GodanKaSokuonbin, ConditionalContraction1) => &["きゃ"],
        (GodanKaSokuonbin, Basic) => &["く"],
        (GodanKaSokuonbinYuku, Conditional) => &["け"],
        (GodanKaSokuonbinYuku, ImperativeE) => &["け"],
        (GodanKaSokuonbinYuku, ConditionalContraction1) => &["きゃ"],
        (GodanKaSokuonbinYuku, Basic) => &["く"],
        (GodanKaSokuonbinYuku, Negative) => &["か"],
        (GodanKaSokuonbinYuku, NegativeU) => &["こ"],
        (GodanKaSokuonbinYuku, Continuous) => &["き"],
        (GodanGa, Basic) => &["ぐ"],
        (GodanGa, NegativeU) => &["ご"],
        (GodanGa, ContinuousTa) => &["い"],
        (GodanGa, Conditional) => &["げ"],
        (GodanGa, Continuous) => &["ぎ"],
        (GodanGa, ImperativeE) => &["げ"],
        (GodanGa, ConditionalContraction1) => &["ぎゃ"],
        (GodanGa, Negative) => &["が"],
        (GodanSa, ImperativeE) => &["せ"],
        (GodanSa, Negative) => &["さ"],
        (GodanSa, NegativeU) => &["そ"],
        (GodanSa, Basic) => &["す"],
        (GodanSa, Conditional) => &["せ"],
        (GodanSa, ConditionalContraction1) => &["しゃ"],
        (GodanSa, Continuous) => &["し"],
        (GodanTa, ConditionalContraction1) => &["ちゃ"],
        (GodanTa, Continuous) => &["ち"],
        (GodanTa, NegativeU) => &["と"],
        (GodanTa, Negative) => &["た"],
        (GodanTa, Basic) => &["つ"],
        (GodanTa, ContinuousTa) => &["っ"],
        (GodanTa, Conditional) => &["て"],
        (GodanTa, ImperativeE) => &["て"],
        (GodanNa, Continuous) => &["に"],
        (GodanNa, ConditionalContraction1) => &["にゃ"],
        (GodanNa, NegativeU) => &["の"],
        (GodanNa, ImperativeE) => &["ね"],
        (GodanNa, ContinuousTa) => &["ん"],
        (GodanNa, Conditional) => &["ね"],
        (GodanNa, Basic) => &["ぬ"],
        (GodanNa, Negative) => &["な"],
        (GodanBa, Basic) => &["ぶ"],
        (GodanBa, NegativeU) => &["ぼ"],
        (GodanBa, ConditionalContraction1) => &["びゃ"],
        (GodanBa, ImperativeE) => &["べ"],
        (GodanBa, Negative) => &["ば"],
        (GodanBa, Conditional) => &["べ"],
        (GodanBa, ContinuousTa) => &["ん"],
        (GodanBa, Continuous) => &["び"],
        (GodanMa, Conditional) => &["め"],
        (GodanMa, Basic) => &["む"],
        (GodanMa, NegativeU) => &["も"],
        (GodanMa, Continuous) => &["み"],
        (GodanMa, ContinuousTa) => &["ん"],
        (GodanMa, Negative) => &["ま"],
        (GodanMa, ImperativeE) => &["め"],
        (GodanMa, ConditionalContraction1) => &["みゃ"],
        (GodanRa, SpecialAttributive2) => &[""],
        (GodanRa, NegativeU) => &["ろ"],
        (GodanRa, ImperativeE) => &["れ"],
        (GodanRa, ConditionalContraction1) => &["りゃ"],
        (GodanRa, Negative) => &["ら"],
        (GodanRa, Conditional) => &["れ"],
        (GodanRa, Continuous) => &["り"],
        (GodanRa, SpecialAttributive) => &["ん"],
        (GodanRa, Basic) => &["る"],
        (GodanRa, NegativeSpecial) => &["ん"],
        (GodanRa, ContinuousTa) => &["っ"],
        (GodanRaAru, Continuous) => &["り"],
        (GodanRaAru, Negative) => &["ら"],
        (GodanRaAru, ContinuousTa) => &["っ"],
        (GodanRaAru, ConditionalContraction1) => &["りゃ"],
        (GodanRaAru, Conditional) => &["れ"],
        (GodanRaAru, SpecialAttributive) => &["ん"],
        (GodanRaAru, ImperativeE) => &["れ"],
        (GodanRaAru, NegativeU) => &["ろ"],
        (GodanRaAru, Basic) => &["る"],
        (GodanRaSpecial, Negative) => &["ら"],
        (GodanRaSpecial, Basic) => &["る"],
        (GodanRaSpecial, NegativeSpecial) => &["ん"],
        (GodanRaSpecial, Conditional) => &["れ"],
        (GodanRaSpecial, Continuous) => &["い"],
        (GodanRaSpecial, ImperativeI) => &["い"],
        (GodanRaSpecial, ConditionalContraction1) => &["りゃ"],
        (GodanRaSpecial, ContinuousTa) => &["っ"],
        (GodanRaSpecial, ImperativeE) => &["れ"],
        (GodanRaSpecial, NegativeU) => &["ろ"],
        (GodanWaUonbin, Basic) => &["う"],
        (GodanWaUonbin, ContinuousTa) => &["う"],
        (GodanWaUonbin, Continuous) => &["い"],
        (GodanWaUonbin, Conditional) => &["え"],
        (GodanWaUonbin, ImperativeE) => &["え"],
        (GodanWaUonbin, Negative) => &["わ"],
        (GodanWaUonbin, NegativeU) => &["お"],
        (GodanWaSokuonbin, NegativeU) => &["お"],
        (GodanWaSokuonbin, Continuous) => &["い"],
        (GodanWaSokuonbin, Conditional) => &["え"],
        (GodanWaSokuonbin, ImperativeE) => &["え"],
        (GodanWaSokuonbin, Basic) => &["う"],
        (GodanWaSokuonbin, ContinuousTa) => &["っ"],
        (GodanWaSokuonbin, Negative) => &["わ"],
        (YodanKa, ImperativeE) => &["け"],
        (YodanKa, Negative) => &["か"],
        (YodanKa, Continuous) => &["き"],
        (YodanKa, Basic) => &["く"],
        (YodanKa, Conditional) => &["け"],
        (YodanGa, ImperativeE) => &["げ"],
        (YodanGa, Conditional) => &["げ"],
        (YodanGa, Negative) => &["が"],
        (YodanGa, Basic) => &["ぐ"],
        (YodanGa, Continuous) => &["ぎ"],
        (YodanSa, Continuous) => &["し"],
        (YodanSa, Negative) => &["さ"],
        (YodanSa, Basic) => &["す"],
        (YodanSa, ImperativeE) => &["せ"],
        (YodanSa, Conditional) => &["せ"],
        (YodanTa, Negative) => &["た"],
        (YodanTa, ImperativeE) => &["て"],
        (YodanTa, Basic) => &["つ"],
        (YodanTa, Continuous) => &["ち"],
        (YodanTa, Conditional) => &["て"],
        (YodanBa, Continuous) => &["び"],
        (YodanBa, Negative) => &["ば"],
        (YodanBa, Conditional) => &["べ"],
        (YodanBa, Basic) => &["ぶ"],
        (YodanBa, ImperativeE) => &["べ"],
        (YodanMa, ImperativeE) => &["め"],
        (YodanMa, Continuous) => &["み"],
        (YodanMa, Basic) => &["む"],
        (YodanMa, Negative) => &["ま"],
        (YodanMa, Conditional) => &["め"],
        (YodanRa, Continuous) => &["り"],
        (YodanRa, ImperativeE) => &["れ"],
        (YodanRa, Negative) => &["ら"],
        (YodanRa, Basic) => &["る"],
        (YodanRa, Conditional) => &["れ"],
        (YodanHa, ImperativeE) => &["へ"],
        (YodanHa, Negative) => &["は"],
        (YodanHa, Continuous) => &["ひ"],
        (YodanHa, Conditional) => &["へ"],
        (YodanHa, Basic) => &["ふ"],
        (Rahen, Continuous) => &["り"],
        (Rahen, AttributiveConjunction) => &["る"],
        (Rahen, Basic) => &["り"],
        (Rahen, Conditional) => &["れ"],
        (Rahen, Negative) => &["ら"],
        (Rahen, ImperativeE) => &["れ"],
        (KaminiDa, ModernBasic) => &["ず"],
        (KaminiDa, Basic) => &["づ"],
        (KaminiDa, Negative) => &["ぢ"],
        (KaminiDa, Continuous) => &["ぢ"],
        (KaminiDa, AttributiveConjunction) => &["づる", "ずる"],
        (KaminiDa, Conditional) => &["づれ", "ずれ"],
        (KaminiDa, ImperativeYo) => &["ぢよ"],
        (KaminiHa, AttributiveConjunction) => &["ふる"],
        (KaminiHa, Conditional) => &["ふれ"],
        (KaminiHa, ImperativeYo) => &["ひよ"],
        (KaminiHa, Negative) => &["ひ"],
        (KaminiHa, Basic) => &["ふ"],
        (KaminiHa, Continuous) => &["ひ"],
        (ShimoniA, Continuous) => &["え"],
        (ShimoniA, Conditional) => &["うれ"],
        (ShimoniA, Negative) => &["え"],
        (ShimoniA, ImperativeYo) => &["えよ"],
        (ShimoniA, Basic) => &["う"],
        (ShimoniA, AttributiveConjunction) => &["うる"],
        (ShimoniKa, AttributiveConjunction) => &["くる"],
        (ShimoniKa, Conditional) => &["くれ"],
        (ShimoniKa, ImperativeYo) => &["けよ"],
        (ShimoniKa, Continuous) => &["け"],
        (ShimoniKa, Negative) => &["け"],
        (ShimoniKa, Basic) => &["く"],
        (ShimoniGa, Conditional) => &["ぐれ"],
        (ShimoniGa, Negative) => &["げ"],
        (ShimoniGa, Basic) => &["ぐ"],
        (ShimoniGa, AttributiveConjunction) => &["ぐる"],
        (ShimoniGa, ImperativeYo) => &["げよ"],
        (ShimoniGa, Continuous) => &["げ"],
        (ShimoniSa, Negative) => &["せ"],
        (ShimoniSa, Basic) => &["す"],
        (ShimoniSa, Conditional) => &["すれ"],
        (ShimoniSa, Continuous) => &["せ"],
        (ShimoniSa, AttributiveConjunction) => &["する"],
        (ShimoniSa, ImperativeYo) => &["せよ"],
        (ShimoniZa, ImperativeYo) => &["ぜよ"],
        (ShimoniZa, Basic) => &["ず"],
        (ShimoniZa, Negative) => &["ぜ"],
        (ShimoniZa, Continuous) => &["ぜ"],
        (ShimoniZa, AttributiveConjunction) => &["ずる"],
        (ShimoniZa, Conditional) => &["ずれ"],
        (ShimoniTa, Basic) => &["つ"],
        (ShimoniTa, Negative) => &["て"],
        (ShimoniTa, ImperativeYo) => &["てよ"],
        (ShimoniTa, AttributiveConjunction) => &["つる"],
        (ShimoniTa, Conditional) => &["つれ"],
        (ShimoniTa, Continuous) => &["て"],
        (ShimoniDa, Negative) => &["で"],
        (ShimoniDa, Conditional) => &["づれ"],
        (ShimoniDa, AttributiveConjunction) => &["づる"],
        (ShimoniDa, Continuous) => &["で"],
        (ShimoniDa, ImperativeYo) => &["でよ"],
        (ShimoniDa, Basic) => &["づ"],
        (ShimoniNa, Basic) => &["ぬ"],
        (ShimoniNa, ImperativeYo) => &["ねよ"],
        (ShimoniNa, AttributiveConjunction) => &["ぬる"],
        (ShimoniNa, Conditional) => &["ぬれ"],
        (ShimoniNa, Continuous) => &["ね"],
        (ShimoniNa, Negative) => &["ね"],
        (ShimoniHa, AttributiveConjunction) => &["ふる"],
        (ShimoniHa, Negative) => &["へ"],
        (ShimoniHa, ImperativeYo) => &["へよ"],
        (ShimoniHa, Conditional) => &["ふれ"],
        (ShimoniHa, Basic) => &["ふ"],
        (ShimoniHa, Continuous) => &["へ"],
        (ShimoniBa, AttributiveConjunction) => &["ぶる"],
        (ShimoniBa, Negative) => &["べ"],
        (ShimoniBa, Continuous) => &["べ"],
        (ShimoniBa, Conditional) => &["ぶれ"],
        (ShimoniBa, ImperativeYo) => &["べよ"],
        (ShimoniBa, Basic) => &["ぶ"],
        (ShimoniMa, Continuous) => &["め"],
        (ShimoniMa, ImperativeYo) => &["めよ"],
        (ShimoniMa, Negative) => &["め"],
        (ShimoniMa, Basic) => &["む"],
        (ShimoniMa, AttributiveConjunction) => &["むる"],
        (ShimoniMa, Conditional) => &["むれ"],
        (ShimoniYa, Conditional) => &["ゆれ"],
        (ShimoniYa, AttributiveConjunction) => &["ゆる"],
        (ShimoniYa, Continuous) => &["え"],
        (ShimoniYa, Negative) => &["え"],
        (ShimoniYa, ImperativeYo) => &["えよ"],
        (ShimoniYa, Basic) => &["ゆ"],
        (ShimoniRa, Conditional) => &["るれ"],
        (ShimoniRa, ImperativeYo) => &["れよ"],
        (ShimoniRa, Negative) => &["れ"],
        (ShimoniRa, AttributiveConjunction) => &["るる"],
        (ShimoniRa, Basic) => &["る"],
        (ShimoniRa, Continuous) => &["れ"],
        (ShimoniWa, AttributiveConjunction) => &["うる"],
        (ShimoniWa, Negative) => &["ゑ"],
        (ShimoniWa, Conditional) => &["うれ"],
        (ShimoniWa, Continuous) => &["ゑ"],
        (ShimoniWa, ImperativeYo) => &["ゑよ"],
        (ShimoniWa, Basic) => &["う"],
        (ShimoniU, Conditional) => &["得れ"],
        (ShimoniU, Basic) => &["得"],
        (ShimoniU, Continuous) => &["得"],
        (ShimoniU, AttributiveConjunction) => &["得る"],
        (ShimoniU, ImperativeYo) => &["得よ"],
        (ShimoniU, Negative) => &["得"],
        (ShimoniU, NegativeU) => &["得よ"],
        (IchidanKureru, ImperativeE) => &["れ"],
        (IchidanKureru, ConditionalContraction1) => &["れりゃ"],
        (IchidanKureru, ImperativeYo) => &["れよ"],
        (IchidanKureru, Continuous) => &["れ"],
        (IchidanKureru, NegativeU) => &["れよ"],
        (IchidanKureru, Basic) => &["れる"],
        (IchidanKureru, Conditional) => &["れれ"],
        (IchidanKureru, ImperativeRo) => &["れろ"],
        (IchidanKureru, NegativeSpecial) => &["ん"],
        (IchidanKureru, Negative) => &["れ"],
        (IchidanRu, Conditional) => &["れ"],
        (IchidanRu, Basic) => &["る"],
        (IchidanRu, ImperativeRo) => &["ろ"],
        (IchidanRu, ConditionalContraction1) => &["りゃ"],
        (AdjectiveAUO, Conditional) => &["けれ"],
        (AdjectiveAUO, NegativeNu) => &["から"],
        (AdjectiveAUO, Basic) => &["い"],
        (AdjectiveAUO, ImperativeE) => &["かれ"],
        (AdjectiveAUO, OldBasic) => &["し"],
        (AdjectiveAUO, ConditionalContraction1) => &["けりゃ"],
        (AdjectiveAUO, ContinuousTa) => &["かっ"],
        (AdjectiveAUO, NegativeU) => &["かろ"],
        (AdjectiveAUO, AttributiveConjunction) => &["き"],
        (AdjectiveAUO, ConditionalContraction2) => &["きゃ"],
        (AdjectiveAUO, Garu) => &[""],
        (AdjectiveAUO, ContinuousTe) => &["く", "くっ"],
        (AdjectiveAUO, ContinuousGozai) => &["う", "ぅ"],
        (AdjectiveI, ConditionalContraction1) => &["けりゃ"],
        (AdjectiveI, ConditionalContraction2) => &["きゃ"],
        (AdjectiveI, ContinuousTa) => &["かっ"],
        (AdjectiveI, OldBasic) => &[""],
        (AdjectiveI, Garu) => &[""],
        (AdjectiveI, NegativeNu) => &["から"],
        (AdjectiveI, ContinuousTe) => &["く", "くっ"],
        (AdjectiveI, Conditional) => &["けれ"],
        (AdjectiveI, Basic) => &["い"],
        (AdjectiveI, ContinuousGozai) => &["ゅう", "ゅぅ"],
        (AdjectiveI, AttributiveConjunction) => &["き"],
        (AdjectiveI, NegativeU) => &["かろ"],
        (AdjectiveI, ImperativeE) => &["かれ"],
        (SpecialNai, ConditionalContraction1) => &["なけりゃ"],
        (SpecialNai, Conditional) => &["なけれ"],
        (SpecialNai, OldBasic) => &["なし"],
        (SpecialNai, ContinuousTa) => &["なかっ", "なかつ"],
        (SpecialNai, ContinuousGozai) => &["のう"],
        (SpecialNai, NegativeU) => &["なかろ"],
        (SpecialNai, ContinuousDe) => &["ない"],
        (SpecialNai, Garu) => &["な"],
        (SpecialNai, ContinuousTe) => &["なく", "なくっ"],
        (SpecialNai, NegativeNu) => &["なから"],
        (SpecialNai, ConditionalContraction2) => &["なきゃ"],
        (SpecialNai, OnbinBasic) => &["ねえ", "ねぇ"],
        (SpecialNai, Basic) => &["ない"],
        (SpecialNai, ImperativeE) => &["なかれ"],
        (SpecialNai, AttributiveConjunction) => &["なき"],
        (SpecialTai, OnbinBasic) => &["てえ", "てぇ"],
        (SpecialTai, OldBasic) => &["たし"],
        (SpecialTai, NegativeNu) => &["たから"],
        (SpecialTai, NegativeU) => &["たかろ"],
        (SpecialTai, Conditional) => &["たけれ"],
        (SpecialTai, AttributiveConjunction) => &["たき"],
        (SpecialTai, ConditionalContraction2) => &["たきゃ"],
        (SpecialTai, ConditionalContraction1) => &["たけりゃ"],
        (SpecialTai, Garu) => &["た"],
        (SpecialTai, Basic) => &["たい"],
        (SpecialTai, ContinuousTa) => &["たかっ", "たかつ"],
        (SpecialTai, ContinuousTe) => &["たく", "たくっ"],
        (SpecialTai, ContinuousGozai) => &["とう"],
        (SpecialTa, Conditional) => &["ら"],
        (SpecialTa, Basic) => &[""],
        (SpecialTa, Negative) => &["ろ"],
        (SpecialDa, ContinuousTa) => &["だっ"],
        (SpecialDa, ImperativeE) => &["なれ"],
        (SpecialDa, Continuous) => &["で"],
        (SpecialDa, Conditional) => &["なら"],
        (SpecialDa, AttributiveConjunction) => &["な"],
        (SpecialDa, Negative) => &["だろ", "だら"],
        (SpecialDa, Basic) => &["だ"],
        (SpecialDesu, Continuous) => &["し"],
        (SpecialDesu, Basic) => &["す"],
        (SpecialDesu, Negative) => &["しょ"],
        (SpecialJa, Continuous) => &["っ"],
        (SpecialJa, Basic) => &[""],
        (SpecialJa, Negative) => &["ろ"],
        (SpecialMasu, Negative) => &["せ"],
        (SpecialMasu, NegativeU) => &["しょ"],
        (SpecialMasu, Conditional) => &["すれ"],
        (SpecialMasu, ImperativeE) => &["せ"],
        (SpecialMasu, Continuous) => &["し"],
        (SpecialMasu, Basic) => &["す"],
        (SpecialMasu, ImperativeI) => &["し"],
        (SpecialNu, ContinuousNi) => &["ず"],
        (SpecialNu, Conditional) => &["ね", "ざれ", "ずん"],
        (SpecialNu, Basic) => &["ぬ"],
        (SpecialNu, Continuous) => &["ざり"],
        (SpecialNu, AttributiveConjunction) => &["ざる"],
        (SpecialNu, OldBasic) => &["ず"],
        (OldBeshi, Negative) => &["から"],
        (OldBeshi, AttributiveConjunction) => &["き"],
        (OldBeshi, Conditional) => &["けれ"],
        (OldBeshi, Basic) => &["し"],
        (OldBeshi, Continuous) => &["く"],
        (OldGotoshi, Continuous) => &["く"],
        (OldGotoshi, Basic) => &["し"],
        (OldGotoshi, AttributiveConjunction) => &["き"],
        (OldNari, Negative) => &["ら"],
        (OldNari, Conditional) => &["れ"],
        (OldNari, ImperativeE) => &["れ"],
        (OldNari, Basic) => &["り"],
        (OldNari, AttributiveConjunction) => &["る"],
        (OldMaji, Conditional) => &["けれ"],
        (OldMaji, Continuous) => &["く"],
        (OldMaji, Basic) => &[""],
        (OldMaji, AttributiveConjunction) => &["き"],
        (OldShimu, ImperativeYo) => &["めよ"],
        (OldShimu, Basic) => &["む"],
        (OldShimu, Continuous) => &["め"],
        (OldShimu, Negative) => &["め"],
        (OldShimu, AttributiveConjunction) => &["むる"],
        (OldShimu, Conditional) => &["むれ"],
        (OldShimu, ImperativeE) => &["むれ"],
        (OldKi, ImperativeE) => &["しか"],
        (OldKi, Basic) => &["き"],
        (OldKi, AttributiveConjunction) => &["し"],
        (OldKeri, Basic) => &["り"],
        (OldKeri, AttributiveConjunction) => &["る"],
        (OldRi, AttributiveConjunction) => &["る"],
        (OldRi, Basic) => &["り"],
        (OldRu, ImperativeYo) => &["れよ"],
        (OldRu, Basic) => &["る"],
        (OldRu, Negative) => &["れ"],
        (OldRu, Conditional) => &["るれ"],
        (OldRu, ImperativeE) => &["るれ"],
        (OldRu, AttributiveConjunction) => &["るる"],
        (OldRu, Continuous) => &["れ"],
        (NoConjugation, Basic) => &[""],
        (AdjectiveII, BasicSokuonbin) => &["いっ", "いいっ"],
        (AdjectiveII, Basic) => &["い"],
        (SpecialDosu, Basic) => &["す"],
        (IchidanEru, Conditional) => &["れ"],
        (IchidanEru, Basic) => &["る"],
        (SpecialYa, Continuous) => &["っ"],
        (SpecialYa, Negative) => &["ろ"],
        (SpecialYa, Basic) => &[""],
        (ConjugationKind::None, ConjugationForm::None) => &[],
        _ => panic!("cannot get suffix of undefined conjugation"),
    }
}

pub fn convert(
    orig: &str,
    kind: ConjugationKind,
    from: ConjugationForm,
    to: ConjugationForm,
) -> String {
    let from_suffixes = get_suffixes(kind, from);
    let from_suffix = from_suffixes
        .iter()
        .find(|&suffix| orig.ends_with(suffix))
        .expect("failed to find suffix.");
    let orig_base = &orig[0..orig.len() - from_suffix.len()];

    // TODO: make a good way to select appropriate candidate?
    //       currently use first one in any ways.
    let to_suffix = get_suffixes(kind, to)
        .get(0)
        .expect("cannot get conjugation suffix.");

    format!("{}{}", orig_base, to_suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            convert(
                "こ",
                ConjugationKind::KahenKuru,
                ConjugationForm::Negative,
                ConjugationForm::Basic,
            ),
            "くる"
        );

        assert_eq!(
            convert(
                "する",
                ConjugationKind::SahenSuru,
                ConjugationForm::Basic,
                ConjugationForm::Continuous,
            ),
            "し"
        )
    }
}
