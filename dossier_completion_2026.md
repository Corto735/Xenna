# Xenna Paie — Dossier de complétion 2026

> Recherche sourcée pour amener **tous les pays à l'année courante (2026)**.
> Phase de **documentation préalable** : aucune migration ni code Rust n'est encore
> modifié. À valider avant implémentation.
>
> Généré le 19/06/2026. Principe cardinal du projet respecté : **aucun chiffre n'est
> inventé**. Les valeurs ci-dessous proviennent de sources officielles ou, à défaut,
> de relais professionnels citant le texte officiel — celles-ci sont marquées
> *« à confirmer sur source officielle »* avant mise en paie réelle.

## Légende des statuts

| Statut | Signification |
|--------|---------------|
| `À JOUR` | Données 2026 déjà correctes en base (ou taux stables portés par `date_fin NULL`). Rien à faire. |
| `MÀJ` | Valeurs indexées / taux à actualiser pour 2026 (fermer la période précédente, ouvrir 2026). |
| `RÉFORME` | Changement structurel (nouveau barème, nouveau régime, nouvelle devise) → travail Rust + SQL. |
| `LACUNE` | Valeur 2026 non encore publiée officiellement → lacune assumée, à re-sourcer. |

---

## Sommaire — matrice de couverture 2026 (41 entrées de l'enum `Pays`)

| Pays | Statut | Nature du chantier 2026 |
|------|--------|-------------------------|
| France (FR) | `MÀJ` | AGS 0,15 → **0,25 %**. SMIC/PMSS 2026 déjà OK. |
| France – FPT | `LACUNE` | Taux employeur **CNRACL** 2026 (hausse programmée) à sourcer. |
| Allemagne (DE) | `À JOUR` | Plafonds (BBG KV 5 812,50 / RV 8 450) et taux 2026 déjà en base. |
| Suisse (CH) | `MÀJ` | Barèmes **IS cantonaux 2026** (26 cantons, Rust). LPP : déjà à jour. |
| Luxembourg (LU) | `À JOUR` | SSM stable depuis mai 2025 ; taux IGSS stables. |
| Belgique (BE) | `MÀJ` | Barème **précompte professionnel 2026** (`be_pp.rs`, indexé). ONSS stable. |
| Italie (IT) | `RÉFORME` | IRPEF 2ᵉ tranche **35 → 33 %** (`it_irpef.rs`). Détrazioni à revoir. |
| Canada (CA) | `MÀJ` | YMPE 74 600 / YAMPE 85 000 / AE 68 900 + taux AE + impôt féd. 2026. |
| Québec (QC) | `MÀJ` | MGA 74 600 / RQAP 103 000 + taux / AE QC 1,30 % / impôt QC 2026. |
| Espagne (ES) | `MÀJ` | SMI 1 221 ×14 / base max 5 101,20 / MEI 0,9 % / cuota solidaridad. |
| Portugal (PT) | `MÀJ` | SMN 920 € / barème **IRS 2026** (Despacho 233-A/2026). |
| Royaume-Uni (UK) | `MÀJ` | Retard : manque **FY 2025/26 et 2026/27** (employeur NI 15 %, ST £5 000). |
| Japon (JP) | `MÀJ` | 協会けんぽ Tokyo 9,85 % + nouvelle 子育て支援金 0,23 % + 雇用 0,85/0,5 % (avr. 2026). |
| Chine (CN) | `MÀJ` | Assurance maladie employeur **4,5 → 6 %** (jan. 2026) + bases Pékin. |
| Corée (KR) | `MÀJ` | Pension **9 → 9,5 %** / santé 7,09 → 7,19 % / LTC 13,14 %. |
| Pays-Bas (NL) | `À JOUR` | 2026 (pilote) sourcé. Historique 2015-2021 = hors périmètre. |
| Australie (AU) | `MÀJ` | FY 2026-27 (1ᵉʳ juil. 2026) : 2ᵉ tranche **16 → 15 %** ; SG 12 % stable. |
| Nouvelle-Zélande (NZ) | `MÀJ` | FY 2026-27 : ACC 1,67 → **1,75 %**, KiwiSaver 3 → **3,5 %** (1ᵉʳ avr. 2026). |
| Pologne (PL) | `MÀJ` | Salaire min 4 806 zł + plafond ZUS 282 600 zł. PIT stable. |
| Andorre (AD) | `MÀJ` (mineure) | Salaire min 1 525,33 €. IRPF/CASS stables. |
| Monaco (MC) | `MÀJ` (mineure) | Base CAR 1 525 € + plafonds CCSS 2026 + SMIC 12,02 €. |
| Danemark (DK) | `RÉFORME` | Nouveau système **4 paliers** (mellemskat 7,5 %). |
| Finlande (FI) | `MÀJ` (mineure) | Chômage employeur **0,20 → 0,31 %** (salarié 0,89 % déjà OK). |
| Suède (SE) | `MÀJ` (mineure) | Skiktgräns 643 000 (vs 625 800). Taux stables. |
| Estonie (EE) | `RÉFORME` | Nouveau **julgeolekumaks 2 %** + abattement uniforme 700 €/mois. |
| Lettonie (LV) | `MÀJ` (mineure) | Salaire min 780 € + abattement 550 €. Taux stables. |
| Lituanie (LT) | `RÉFORME` | GPM progressif **3 paliers 20/25/32 %** + MMA 1 153 €. |
| Autriche (AT) | `MÀJ` | Höchstbeitragsgrundlage **6 930 €/mois** (vs 6 450) + barème indexé. |
| Tchéquie (CZ) | `MÀJ` (mineure) | Salaire min 22 400 Kč + base max 2 350 416 Kč. Taux stables. |
| Slovaquie (SK) | `MÀJ` | Assurance **santé 15 → 16 %**. Bases/salaire min 2026. |
| Hongrie (HU) | `À JOUR` | SZJA 15 % / szocho 13 % / TB 18,5 % stables. Minimálbér indexé. |
| Slovénie (SI) | `MÀJ` (mineure) | Paliers relevés + olajšava 5 551,93 € + salaire min 1 481,88 €. |
| Grèce (GR) | `RÉFORME` | Barème impôt 2026 (toutes tranches −2 pp sauf 9 %, logique âge/enfants). |
| Chypre (CY) | `RÉFORME` | Barème impôt 2026 (exonéré 19 500 → **22 000**, tranches 20/25/30/35). |
| Malte (MT) | `MÀJ` | 1ᵉʳ tranche exonérée 12 000 € + SSC seuils +~3 %. |
| Croatie (HR) | `MÀJ` (mineure) | Salaire min 1 050 € + osobni odbitak 600 €. Taux stables. |
| Irlande (IE) | `MÀJ` | PRSI 4,10 → **4,35 %** (1ᵉʳ oct. 2026) + bandes USC ajustées. |
| Roumanie (RO) | `MÀJ` (mineure) | Salaire min 4 050 → 4 325 RON (1ᵉʳ juil. 2026). Taux stables. |
| Bulgarie (BG) | `RÉFORME` | **Passage à l'euro** + retraite +2 pp + plafond assurable revalorisé. |

**Bilan** : 4 `À JOUR`, 2 `LACUNE`, 6 `RÉFORME`, le reste `MÀJ`. Aucun pays n'est
silencieusement faux pour 2026 (les régimes à taux stables sont correctement portés
par `date_fin NULL`) — mais 6 réformes structurelles et de nombreuses indexations
restent à intégrer.

---

## Détail par pays

> Champs : **base** = ce qui est en base aujourd'hui · **2026** = valeur officielle ·
> **source** · **cible** technique.

### France (FR) — `MÀJ`
**AGS (garantie des salaires)**
- base : `0,0015` patronal (0,15 %), `'2024-01-01' → NULL` — `0004_seed_cotisations.sql:117`.
- 2026 : **0,25 %** patronal, maintenu au 01/01/2026 (CA de l'AGS du 16/12/2025).
- source : Unédic/AGS ; relais URSSAF & service-public.gouv.fr (Service Public Entreprendre, A17906).
- cible : nouvelle migration fermant la période 2024 (insérer 0,20 % au 01/07/2024 puis 0,25 %) → `cotisation_taux` code `AGS`.
- *Reste OK* : SMIC 01/01/2026 (1 823,03 €) et 01/06/2026 (1 867,02 €), PMSS 4 005 €, PASS 48 060 €, réduction générale nouvelle formule, AGIRC-ARRCO figés 2024-2026 — déjà en base (`0006`, `0014`, `0047`, `0049`).

### France – Fonction Publique Territoriale (FPT) — `LACUNE`
- base : taux 2025 (`0015_fpt.sql`, `0024_fpt_2015.sql`).
- 2026 : taux employeur **CNRACL** en hausse (relèvement pluriannuel +1 pt/an programmé par décret pour combler le déficit, 2024→2028).
- source : à sourcer — décret CNRACL annuel / LFSS 2026 (Légifrance).
- cible : `plafond_reference`/`cotisation_taux` FPT + éventuellement `fpt_cotisations.rs`. **À re-sourcer** avant chiffrage (lacune assumée).

### Allemagne (DE) — `À JOUR`
- BBG KV/PV 5 812,50 €/mois (69 750/an), BBG RV/AV 8 450 €/mois : déjà en base (`0028`, `0029`).
- Taux 2026 : RV 18,6 %, AV 2,6 %, PV 3,6 % (sans enfant +0,6 %), KV 14,6 % + Zusatzbeitrag moyen 2,9 % : conformes.
- source : Sozialversicherungsrechengrößen-Verordnung 2026 (Bundeskabinett 08/10/2025) ; GKV-Spitzenverband ; Deutsche Rentenversicherung.
- cible : aucune. (Vérifier seulement le Zusatzbeitrag moyen 2,9 % si pas encore en base.)

### Suisse (CH) — `MÀJ`
- 1ᵉʳ/2ᵉ piliers (AVS 8,7 %, AI 1,4 %, APG 0,5 %, AC, LPP) : taux stables 2026 ; seuil d'entrée LPP 22 680 et déduction de coordination 26 460 déjà en base (`0022_ch_historique.sql`).
- 2026 : taux d'intérêt minimal LPP maintenu à **1,25 %** ; déduction de coordination 26 460 CHF, salaire coordonné max 64 260 CHF — inchangés.
- **À FAIRE** : barèmes **Impôt à la Source (IS) cantonaux 2026** — 26 cantons, tables annuelles, calcul dans `ch_is.rs`.
- source : OFAS (bsv.admin.ch) pour LPP ; administrations fiscales cantonales pour l'IS.
- cible : `ch_is.rs` (tables cantonales 2026). LPP : RAS.

### Luxembourg (LU) — `À JOUR`
- SSM non qualifié 2 703,74 € / qualifié 3 244,48 € (indice 968,04) : **inchangés** depuis le 01/05/2025, pas de hausse au 01/01/2026 (prochaine indexation attendue T3 2026).
- Taux IGSS (maladie+vieillesse 5,25 % sal / 8,10 % pat, chômage) : stables.
- source : CCSS/IGSS ; PwC Luxembourg « Paramètres sociaux au 1er janvier 2026 ».
- cible : aucune (surveiller l'indexation T3 2026).

### Belgique (BE) — `MÀJ`
- ONSS 13,07 % sal / 25,92 % pat de référence + réduction structurelle + bonus emploi : stables.
- 2026 : barème **précompte professionnel** indexé — tranches 26,75 / 42,80 / 48,15 / 53,50 %, quotité exemptée 10 900 €, fin d'une mesure transitoire pour personnes à charge.
- source : SPF Finances (barèmes PP au Moniteur belge) ; à confirmer sur le barème officiel 2026.
- cible : `be_pp.rs` (barème 2026) ; vérifier l'indexation du seuil du bonus emploi.

### Italie (IT) — `RÉFORME`
- base : `it_irpef.rs` branche `annee > 2023` = **23 % / 35 % / 43 %** (seuils 28 000 / 50 000).
- 2026 (Legge di Bilancio 2026) : 2ᵉ tranche **33 %** (28 000–50 000), stérilisée au-delà de 200 000 € de revenu ; 1ᵉʳ (23 %) et 3ᵉ (43 %) inchangées ; no-tax area 8 500 € ; premi di produttività 1 %, fringe benefits 1 000/2 000 €.
- source : MEF (mef.gov.it, « Principali misure della legge di bilancio 2026 ») ; à confirmer au texte de loi publié.
- cible : ajouter une branche `annee >= 2026` dans `irpef_annuel` (et ajuster `detrazione_lavdip` si déductions modifiées). Addizionale regionale : les régions conservent leurs barèmes 2025 jusqu'en 2028 sauf délibération → table `addizionale_regionale` portée telle quelle, **vérifier au cas par cas** les délibérations régionales 2026.

### Canada (CA) — `MÀJ`
- base : MGA 71 300 / MGAP2 81 900 / MAGA (AE) 65 700 / AE 1,64 % sal · 2,296 % pat (`date_fin NULL`, `0020`/`0021`).
- 2026 : **YMPE 74 600**, **YAMPE 85 000**, exemption de base 3 500, RPC 5,95 %, RPC2 4,00 % ; **AE MIE 68 900**, taux salarié **1,63 %** / employeur **2,28 %** (max 1 123,07 $).
- source : ARC (canada.ca, communiqué « maximum pensionable earnings 2026 ») ; Commission de l'AE (communiqué 09/2025).
- cible : fermer les `plafond_reference` 2025 et ouvrir 2026 (MGA, MGAP2, MAGA) ; `cotisation_taux` AE 2026 ; barème impôt fédéral + Ontario 2026 dans `ca_impot.rs` (tables T4032 — **à sourcer en détail**).

### Québec (QC) — `MÀJ`
- base : MGA 71 300, RQAP max 97 500 + 0,494/0,692 %, AE QC 1,31/1,834 % (`date_fin NULL`).
- 2026 : **MGA 74 600** (RRQ 6,40 % / RRQ2 4,00 %) ; **RQAP max 103 000**, taux **0,430 % sal / 0,602 % pat** ; **AE QC 1,30 % sal / 1,82 % pat** ; FSS stable.
- source : Revenu Québec, Retraite Québec, RAMQ (actualité 08/01/2026), Commission de l'AE.
- cible : `plafond_reference` + `cotisation_taux` QC 2026 ; barème impôt QC 2026 dans `qc_impot.rs` (TP-1015.3 — **à sourcer**).

### Espagne (ES) — `MÀJ`
- base : SMI/ES_BASE_MIN 1 184 €, ES_BASE_MAX 4 909,50 €, MEI 0,13 sal / 0,53 pat (`date_fin NULL`).
- 2026 : **SMI 1 221 €/mois ×14 (17 094 €/an)**, +3,1 % (RD approuvé 17/02/2026, rétroactif janvier) ; **base máxima 5 101,20 €/mois** ; **MEI 0,9 %** (0,15 sal / 0,75 pat) ; cuota de solidaridad sur l'excédent de la base max (tranche 5 101,21–5 611,32 € : +1,15 %, dont 0,19 sal / 0,96 pat).
- source : RDL 3/2026 (BOE 04/02/2026), Orden PJC/297/2026, RD du SMI 2026.
- cible : fermer 2025 / ouvrir 2026 sur `plafond_reference` (ES_BASE_MAX, ES_BASE_MIN/SMI) et `cotisation_taux` MEI ; ajouter la cuota de solidaridad (nouvelle cotisation ou logique `es_cotizaciones.rs`).

### Portugal (PT) — `MÀJ`
- base : SMN 870 € (`date_fin NULL`), barème IRS 2025 (`pt_irs.rs`).
- 2026 : **SMN 920 €/mois** (+50 €, OE 2026) ; tables **IRS 2026** (Despacho n.º 233-A/2026 du 06/01) — escalões +3,51 %, taux des 2ᵉ au 5ᵉ escalão −0,3 pp ; mínimo de existência 12 880 €.
- source : Governo (portugal.gov.pt), Despacho 233-A/2026, OE 2026.
- cible : `plafond_reference` SMN 2026 ; barème `pt_irs.rs` 2026 (TSU 11 %/23,75 % stable).

### Royaume-Uni (UK) — `MÀJ` (rattrapage de 2 exercices)
- base : seul **FY 2024/25** (depuis 06/04/2024) — NI employeur 13,8 %, secondary threshold £9 100 (`0051`/`0052`).
- 2026 : en vigueur au 19/06/2026 = **FY 2026/27** → NI salarié 8 % (12 570–50 270) / 2 % au-delà ; **NI employeur 15 %, secondary threshold £5 000** (réforme d'octobre 2024, déjà en vigueur depuis 06/04/2025) ; Income Tax PAYE gelé (PA 12 570, 20/40/45 % à 50 270/125 140) ; Employment Allowance £10 500.
- source : HMRC, « Rates and thresholds for employers 2026 to 2027 » ; Social Security (Contributions) Regs.
- cible : créer les périodes **FY 2025/26** (06/04/2025) et **FY 2026/27** (06/04/2026) dans `plafond_reference`/`cotisation_taux` UK ; vérifier les seuils dans `uk_cotisations.rs`.

### Japon (JP) — `MÀJ`
- base : taux d'avril 2024 (`0053`–`0055`, `0081`).
- 2026 (année fiscale dès le 01/04/2026) : 協会けんぽ Tokyo **santé 9,85 %**, **介護 1,62 %** ; **nouvelle 子ども・子育て支援金 0,23 %** (part. salarié/employeur, dès avril 2026) ; 厚生年金 18,3 % (fixe) ; **雇用保険 employeur 0,85 % / salarié 0,5 %**.
- source : 全国健康保険協会 (協会けんぽ), 厚生労働省.
- cible : `cotisation_taux` JP au 01/04/2026 + ajouter la cotisation 子育て支援金 ; vérifier `jp_cotisations.rs`.

### Chine (CN) — `MÀJ`
- base : Pékin, données 2024 (`0057`/`0058`), assurance maladie employeur 4,5 %.
- 2026 : **assurance maladie employeur rétablie de 4,5 % à 6 %** (et travailleurs indépendants 6,5 → 8 %) au 01/01/2026 ; bases Pékin (depuis 07/2025) plafond **35 811** / plancher **7 162** ¥/mois.
- source : 北京市医保局 / 北京市人社局 (bendibao relayant les avis officiels) ; à confirmer sur source gouvernementale.
- cible : `cotisation_taux` CN maladie 6 % au 01/01/2026 ; `plafond_reference` bases Pékin 2025-2026.

### Corée (KR) — `MÀJ`
- base : taux 2025 (`0068`, `0081`).
- 2026 : **국민연금 9 % → 9,5 %** (4,75 % chacun ; +0,5 pt/an jusqu'à 13 % en 2033) ; **건강보험 7,09 % → 7,19 %** (3,595 % chacun) ; **장기요양 12,95 % → 13,14 %** de la prime santé ; 고용보험 salarié 0,9 %.
- source : 국민연금공단 / 국민건강보험공단 / 고용노동부.
- cible : `cotisation_taux` KR 2026 (pension, santé, LTC) ; vérifier `kr_bulletin.rs`.

### Pays-Bas (NL) — `À JOUR`
- 2026 (box 1) : 35,75 % (≤ 38 883) / 37,56 % (→ 78 426) / 49,50 % ; AHK max 3 115 (dégressif 6,398 % dès 29 736) ; arbeidskorting, alleenstaande-ouderkorting 540, jonggehandicaptenkorting 923 — déjà sourcés (`0061`–`0062`, `0073`+, `nl_loonheffing.rs`).
- source : Belastingdienst, Belastingplan 2026.
- cible : aucune pour 2026. (Historique 2015-2021 = hors périmètre validé.)

### Australie (AU) — `MÀJ`
- base : FY 2025-26 (depuis 01/07/2025, `0064`/`0078`) — barème avec 2ᵉ tranche 16 %, Super Guarantee 12 %.
- 2026 : à compter du **01/07/2026 (FY 2026-27)**, 2ᵉ tranche **16 % → 15 %** (18 201–45 000) ; autres tranches inchangées (30/37/45) ; Medicare levy 2 %, seuil single 29 207 ; **Super Guarantee maintenu à 12 %**.
- source : ATO (« Individual income tax rates »), législation « cost-of-living tax cuts ».
- cible : créer la période FY 2026-27 (01/07/2026) dans `au_bulletin.rs`/tables — la baisse prend effet dans 12 jours.

### Nouvelle-Zélande (NZ) — `MÀJ`
- base : FY 2025-26 (depuis 01/04/2025, `0066`/`0079`).
- 2026 : **FY 2026-27 en vigueur depuis le 01/04/2026** — barème PAYE inchangé (10,5/17,5/30/33/39 %) ; **ACC earner levy 1,67 % → 1,75 %**, plafond 152 790 → **156 641 $** ; **KiwiSaver défaut 3 % → 3,5 %**.
- source : Inland Revenue (IRD), ACC.
- cible : `cotisation_taux`/tables NZ au 01/04/2026 (ACC + KiwiSaver) ; barème PAYE inchangé.

### Pologne (PL) — `MÀJ` (mineure)
- base : 2025 (`0067`/`0080`), `date_fin NULL`.
- 2026 : **płaca minimalna 4 806 zł** ; plafond annuel ZUS (30×) **282 600 zł** ; PIT inchangé (12/32 %, kwota wolna 30 000 zł, składka zdrowotna 9 %).
- source : Rozporządzenie płaca minimalna 2026 ; ZUS.
- cible : `plafond_reference` salaire min + plafond ZUS 2026. Taux : RAS.

### Andorre (AD) — `MÀJ` (mineure)
- base : depuis 2015, `date_fin NULL` (`0069`/`0077`).
- 2026 : **salari mínim 1 525,33 €/mois** ; CASS (salarié 6,5 %, total ~22 %) et IRPF (exempt 24 000 €, max 10 %) inchangés depuis 2015.
- source : Govern d'Andorra (CASS), BOPA.
- cible : `plafond_reference` salaire min 2026 si utilisé. Taux : RAS.

### Monaco (MC) — `MÀJ` (mineure)
- base : 2025 (`0070`/`0083`), `date_fin NULL`.
- 2026 : SMIC aligné France **12,02 €/h** (01/01/2026) ; chômage 6,40 % ; **base CAR 1 525 €** (01/10/2025–30/09/2026) ; plafonds CCSS 2026 à actualiser. Pas d'impôt sur le revenu.
- source : Caisses Sociales de Monaco (caisses-sociales.mc), CLEISS.
- cible : `plafond_reference` MC (plafond CCSS, base CAR) 2026.

### Danemark (DK) — `RÉFORME`
- base : 2025 (`0071`/`0082`) — système bundskat + topskat (2 niveaux).
- 2026 : passage à **4 niveaux** — bundskat (au-delà du personfradrag), **mellemskat 7,5 %** (> 641 200 kr), **topskat 7,5 %** (> 777 900 kr après AM-bidrag), **top-topskat 5,0 %** (> 2 592 700 kr) ; personfradrag **54 100 kr** ; AM-bidrag 8 % ; beskæftigelsesfradrag 12,75 % (plafond 63 300 kr).
- source : Skatteministeriet / skat.dk (satser og beløbsgrænser 2026).
- cible : refonte du calcul d'impôt dans `dk_bulletin.rs` (4 paliers) + `plafond_reference` seuils 2026.

### Finlande (FI) — `MÀJ` (mineure)
- base : `FI_TYOTTOMYYS` 2026 = salarié 0,89 % / **employeur 0,20 %** (`0072_fi_organismes.sql:49`).
- 2026 : chômage **employeur 0,31 %** (jusqu'à 2 509 500 € de masse salariale ; 1,23 % au-delà) — le salarié 0,89 % est déjà bon ; sairausvakuutus 1,91 % (sairaanhoito 1,10 % + päiväraha 0,88 %) ; TyEL employeur moyen 17,10 %.
- source : Työllisyysrahasto (confirmation des taux 2026) ; STM ; Veronmaksajat.
- cible : corriger `taux_patronal` 0,20 → 0,31 sur `FI_TYOTTOMYYS` ; vérifier sairausvakuutus.

### Suède (SE) — `MÀJ` (mineure)
- base : 2025 (`0085`), `date_fin NULL`.
- 2026 : arbetsgivaravgift 31,42 % (stable) ; **skiktgräns statlig skatt 643 000 kr** (vs 625 800) au-delà → +20 % ; grundavdrag 16 500–41 200 kr ; kommunalskatt moyenne 32,38 %.
- source : Skatteverket (« Belopp och procent inkomstår 2026 »).
- cible : `plafond_reference`/tables SE skiktgräns 2026. Taux : RAS.

### Estonie (EE) — `RÉFORME`
- base : 2025 (`0086`) — tulumaks 22 %, « tax staircase » (abattement dégressif).
- 2026 : tulumaks 22 % et sotsiaalmaks 33 % inchangés ; **nouveau julgeolekumaks 2 %** retenu sur le brut au 01/01/2026 (ne réduit pas l'assiette imposable) ; **abattement uniforme 700 €/mois** (fin de l'escalier) ; töötuskindlustus 1,6 % sal / 0,8 % pat.
- source : Maksu- ja Tolliamet (emta.ee, « Maksumuudatused 2026 »).
- cible : ajouter le julgeolekumaks 2 % (nouvelle cotisation) + abattement 700 €/mois dans `ee_bulletin.rs`.

### Lettonie (LV) — `MÀJ` (mineure)
- base : 2025 (`0087`).
- 2026 : IIN 25,5 % / 33 % et VSAOI 34,09 % (sal+pat) inchangés, plafond VSAOI 105 300 € maintenu ; **salaire min 780 €** ; **neapliekamais minimums 550 €/mois**.
- source : VID ; PwC Latvija (« 2026. gada špikeris »).
- cible : `plafond_reference` salaire min + abattement 2026. Taux : RAS.

### Lituanie (LT) — `RÉFORME`
- base : 2025 (`0088`) — GPM 20/32 %.
- 2026 : **nouveau GPM progressif 3 paliers** au 01/01/2026 — 20 % (≤ 83 237,40 €/an), 25 % (jusqu'à 138 729 €), **32 %** (au-delà) ; **MMA 1 153 €/mois** ; Sodra salarié ~19,5 % (dont PSD 6,98 %), employeur 1,77 %.
- source : VMI ; Sodra.
- cible : refonte du barème GPM (3 paliers) dans `lt_bulletin.rs` + salaire min 2026.

### Autriche (AT) — `MÀJ`
- base : 2025 (`0089`) — Höchstbeitragsgrundlage 6 450 €/mois (mentionnée dans le texte_loi).
- 2026 : **Höchstbeitragsgrundlage 6 930 €/mois** (Sonderzahlungen 13 860 €/an) ; Geringfügigkeitsgrenze 551,10 € (gelée) ; barème Lohnsteuer indexé (Aufwertungszahl ~1,073) ; SV salarié ~18,07 %.
- source : ÖGK / WKO (« SV-Werte 2026 ») ; BMF pour le barème.
- cible : `plafond_reference` Höchstbeitragsgrundlage 2026 + barème Lohnsteuer 2026 (`at_bulletin.rs`).

### Tchéquie (CZ) — `MÀJ` (mineure)
- base : 2025 (`0090`).
- 2026 : sociální 24,8 % pat / 6,5 % sal, zdravotní 9 % pat / 4,5 % sal inchangés ; **minimální mzda 22 400 Kč** ; **maximální vyměřovací základ 2 350 416 Kč** ; seuil de la tranche **23 % à 1 762 812 Kč/an** (mzda moyenne 48 967 Kč).
- source : ČSSZ ; VZP.
- cible : `plafond_reference` salaire min, base max, seuil 23 % 2026. Taux : RAS.

### Slovaquie (SK) — `MÀJ`
- base : 2025 (`0091`) — assurance santé totale 15 %.
- 2026 : **assurance santé 15 % → 16 %** ; vymeriavací základ et minimums revalorisés (mzda moyenne 2024 = 1 524 €) ; daň 19/25 % inchangé.
- source : Sociálna poisťovňa, Zdravotná poisťovňa ; à confirmer (répartition sal/pat de la hausse santé).
- cible : `cotisation_taux` SK santé 16 % 2026 + bases. **Confirmer la répartition** salarié/employeur avant chiffrage.

### Hongrie (HU) — `À JOUR`
- 2026 : SZJA 15 % (flat), szocho 13 %, TB 18,5 % — tous **stables** (portés par `date_fin NULL`, `0092`). Suppression du multiplicateur 112,5 % pour les indépendants (hors salariés). Minimálbér indexé (assiette plancher 30 % = 96 840 Ft).
- source : NAV.
- cible : aucune pour les salariés (mettre à jour le minimálbér si utilisé comme base).

### Slovénie (SI) — `MÀJ` (mineure)
- base : 2025 (`0093`).
- 2026 : stopnje dohodnine inchangées mais **paliers relevés** ; **splošna olajšava 5 551,93 €/an** (+291,93) ; **minimalna plača 1 481,88 €** ; prispevki (22,1 % sal / 16,1 % pat) stables.
- source : FURS (lestvica dohodnine 2026).
- cible : barème/seuils + olajšava 2026 (`si_bulletin.rs`) + salaire min.

### Grèce (GR) — `RÉFORME`
- base : 2025 (`0094`) — barème 9/22/28/36/44 %.
- 2026 : réforme majeure au 01/01/2026 — **toutes les tranches −2 pp sauf la 1ᵉʳ (9 %)** ; tranche 40 000–60 000 **44 % → 39 %** ; exonération < 25 ans (jusqu'à 20 000 €), taux réduit 9 % de 25 à 30 ans ; réductions liées au nombre d'enfants (jusqu'à 0 % à 4 enfants) ; EFKA : classes d'assurance ajustées +2,5 % (circulaire e-EFKA, dès 01/01/2026).
- source : ΑΑΔΕ / Υπουργείο Οικονομικών ; e-EFKA (εγκύκλιος 2026).
- cible : refonte du barème + logique âge/enfants dans `gr_bulletin.rs` ; `cotisation_taux`/plafonds EFKA 2026. **Sourcer le barème complet** (tranches exactes) au texte officiel.

### Chypre (CY) — `RÉFORME`
- base : 2025 (`0095`) — exonéré 19 500 €, tranches 20/25/30/35.
- 2026 (réforme votée déc. 2025) : **seuil exonéré 19 500 → 22 000 €** ; tranches 20 % (22–32 k), 25 % (32–42 k), 30 % (42–72 k), 35 % (> 72 k) ; assurance sociale 8,3 % sal / 8,3 % pat, plafond **62 868 €/an** ; GESY 2,65 % sal / 2,90 % pat ; salaire min 1 088 €.
- source : Τμήμα Φορολογίας (Cyprus Tax Dept) ; réforme fiscale déc. 2025.
- cible : barème impôt 2026 (`cy_bulletin.rs`) + plafond SI 2026.

### Malte (MT) — `MÀJ`
- base : 2025 (`0096`).
- 2026 : SSC class 1 10 % sal / 10 % pat, seuils +~3 % (max 55,93 €/semaine pour salaire > 29 084 €) ; barème impôt — **1ᵉʳ tranche exonérée portée à 12 000 €** (réforme phasée du budget) ; COLA +4,66 €/semaine.
- source : MTCA (mtca.gov.mt, « Class 1 SSC rates 2026 ») ; PwC Malta « Payroll updates 2026 ».
- cible : barème impôt 2026 + seuils SSC (`mt_bulletin.rs`).

### Croatie (HR) — `MÀJ` (mineure)
- base : 2025 (`0097`).
- 2026 : mirovinsko 20 % (15 % I + 5 % II) sal, zdravstveno 16,5 % pat inchangés ; **minimalna plaća 1 050 €/mois** (+8,25 %) ; **osobni odbitak 600 €** (base) ; barèmes d'impôt locaux (JLS) variables par commune (publiés au NN).
- source : Porezna uprava (« Stope godišnjeg poreza na dohodak 2026 »).
- cible : salaire min + osobni odbitak 2026 ; vérifier les taux locaux par défaut. Taux cotisations : RAS.

### Irlande (IE) — `MÀJ`
- base : 2025 (`0098`) — PRSI salarié 4,10 %.
- 2026 (Budget 2026) : **PRSI salarié 4,10 % → 4,35 %** au 01/10/2026 (employeur 8,90→9,15 % / 11,15→11,40 %) ; USC — band 2 plafond 27 382 → **28 700 €** (taux 0,5/2/3/8 %) ; Income Tax bands inchangés (standard rate cut-off 44 000 €) ; crédits stables.
- source : Revenue (kpmg.com/ie Budget 2026 tables relaye les chiffres officiels) ; à confirmer Revenue.
- cible : `cotisation_taux` PRSI 4,35 % (01/10/2026) + bandes USC 2026 (`ie_bulletin.rs`).

### Roumanie (RO) — `MÀJ` (mineure)
- base : régime « figé » 2018-2025 (`0099`) — CAS 25 %, CASS 10 %, impôt 10 %, CAM 2,25 % pat.
- 2026 : taux **inchangés** ; salaire minimum brut **4 050 RON** au 01/01/2026 puis **4 325 RON au 01/07/2026** ; assiettes CAS/CASS calées sur le salaire min au 01/01/2026 (4 050 RON).
- source : Hotărâre de Guvern salariu minim 2026 ; ANAF.
- cible : `plafond_reference` salaire min RO 2026 (deux périodes). Taux : RAS.

### Bulgarie (BG) — `RÉFORME`
- base : `0100_bg_paie.sql` explicitement **« Devise BGN, données 2025 »** — cotisations 13,78 % sal / 18,92 % pat, plafond 3 750 BGN/mois (codé `bg_bulletin.rs`), impôt 10 %.
- 2026 : **adoption de l'euro au 01/01/2026** — tous les montants en EUR ; **revenu maximal assurable** porté de 4 130 à **4 600 BGN ≈ 2 352 €** (transitoire 2 111,64 € avant budget définitif) ; **cotisations retraite +2 pp** ; salaire min **620,20 €** ; impôt 10 % (flat) inchangé.
- source : НАП / НОИ ; budget 2026 ; législation d'adoption de l'euro.
- cible : conversion devise EUR + plafond assurable 2026 + retraite +2 pp dans `bg_bulletin.rs` / `0100`. **Confirmer** le plafond définitif (4 600 BGN vs transitoire) au budget 2026.

---

## Récapitulatif des actions (par type de chantier)

### A. Réformes structurelles (Rust + SQL) — priorité haute
1. **Italie** : 2ᵉ tranche IRPEF 33 % 2026 → branche `annee >= 2026` dans `it_irpef.rs`.
2. **Danemark** : refonte 4 paliers d'impôt (mellemskat) → `dk_bulletin.rs` + seuils.
3. **Lituanie** : barème GPM 3 paliers progressifs → `lt_bulletin.rs`.
4. **Estonie** : julgeolekumaks 2 % + abattement 700 €/mois → `ee_bulletin.rs`.
5. **Grèce** : barème impôt 2026 + logique âge/enfants → `gr_bulletin.rs` (barème complet à sourcer).
6. **Chypre** : barème impôt 2026 (exonéré 22 000) → `cy_bulletin.rs`.
7. **Bulgarie** : passage EUR + plafond + retraite +2 pp → `bg_bulletin.rs` / `0100`.
8. **Royaume-Uni** : rattrapage FY 2025/26 et 2026/27 (NI employeur 15 %, ST £5 000).

### B. Mises à jour de barèmes d'impôt en Rust
- Portugal `pt_irs.rs` (2026) · Canada `ca_impot.rs` (féd + Ontario) · Québec `qc_impot.rs` ·
  Suisse `ch_is.rs` (26 cantons) · Belgique `be_pp.rs` · Autriche `at_bulletin.rs` ·
  Suède (skiktgräns) · Slovénie · Malte · Irlande (USC) · Australie (FY 2026-27) ·
  Nouvelle-Zélande (PAYE inchangé, ACC/KiwiSaver).

### C. Migrations SQL — plafonds & taux indexés 2026
- **Plafonds** : ES (base max/min, SMI) · PT (SMN) · CA/QC (MGA, MGAP2, MAGA, RQAP) ·
  PL (salaire min, plafond ZUS) · CZ (salaire min, base max) · AT (Höchstbeitragsgrundlage) ·
  LV/SI/HR/AD/MC/RO (salaire min, abattements) · SE (skiktgräns) · BG (plafond assurable).
- **Taux** : FR (AGS 0,25 %) · ES (MEI 0,9 % + cuota solidaridad) · CA/QC (AE 2026) ·
  JP (協会けんぽ + 子育て支援金 + 雇用) · CN (maladie 6 %) · KR (pension 9,5 %, santé 7,19 %) ·
  FI (chômage employeur 0,31 %) · SK (santé 16 %) · IE (PRSI 4,35 %) · NZ (ACC 1,75 %).

### D. Lacunes assumées (à re-sourcer avant chiffrage)
- **FPT** : taux employeur CNRACL 2026.
- **Grèce** : tranches exactes du nouveau barème (texte officiel).
- **Slovaquie** : répartition salarié/employeur de la hausse santé 16 %.
- **Bulgarie** : plafond assurable définitif (budget 2026).
- **Italie** : délibérations régionales 2026 (addizionale) au cas par cas.

### Sources officielles de référence (compléter `veille_reglementaire_2026.txt`)
UK : HMRC · JP : 協会けんぽ / 厚生労働省 · CN : 北京市医保局・人社局 · KR : 국민연금공단 /
국민건강보험공단 · AU : ATO · NZ : IRD / ACC · les autorités fiscales et de sécurité sociale
nationales (Skatteverket, Maksuamet, VMI/Sodra, ÖGK, ČSSZ, Sociálna poisťovňa, NAV, FURS,
ΑΑΔΕ/e-EFKA, Cyprus Tax Dept, MTCA, Porezna uprava, Revenue, ANAF, НАП/НОИ, Belastingdienst,
CASS Andorre, Caisses Sociales de Monaco, Skatteministeriet) — à ajouter à la liste existante.

---

## Prochaine étape

Après validation de ce dossier, l'implémentation suivra l'ordre A → B → C, chaque
valeur étant rattachée à son `texte_loi` (source officielle) conformément au schéma,
et toute valeur encore non publiée laissée en **lacune assumée** (mécanisme
`pays_non_couvert` / message « aucun chiffre inventé »).
