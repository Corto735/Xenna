// ── Liens vers les textes officiels ──────────────────────────────────────────
//
// Rend cliquables les articles de code et les textes (lois, décrets, arrêtés,
// ordonnances) cités dans l'interface, vers leur publication officielle :
// Légifrance pour la France, le portail législatif de l'État pour chaque autre
// pays. Aucune autre source : un texte non publié par une autorité publique
// reste du texte simple.
//
// Règle de fond : on ne lie QUE ce qui figure dans les tables ci-dessous, et
// chaque entrée a été vérifiée le 01/10/2026 en ouvrant la page (titre, code,
// numéro ou article contrôlés ; pour les sites rendus en JavaScript, page
// affichée dans un navigateur ; à chaque fois, une adresse inventée a été
// essayée pour s'assurer que le site sait dire « introuvable »). Une citation
// absente des tables n'est pas devinée : elle reste sans lien.
//
// Périmètre : France et une trentaine de pays. Restent sans lien, faute de source officielle
// vérifiable depuis l'outil : Hongrie, Bulgarie, Grèce, Chypre, Inde, Mexique,
// Émirats, Nouvelle-Zélande, Québec (RLRQ), Japon (e-Gov en maintenance le
// jour du relevé) ; Monaco ne cite qu'un organisme.
//
// À chaque mise à jour de barèmes, revérifier les consolidations qui changent
// d'adresse (Danemark : nouvelle LBK ; Légifrance : nouvelles versions
// d'articles, signalées par le site lui-même).
//
// Fonctionnement : le lieur parcourt les nœuds de texte déjà affichés. Les
// citations ne sont jamais traduites (refs.rs : « les CITATIONS restent
// INTACTES »), elles se reconnaissent donc dans les six langues ; les mots
// descriptifs traduits (« Loi du » → « Law of »…) sont prévus. Pendant une
// traduction côté navigateur (dictionnaire lang.js, MyMemory), main.js retire
// les liens puis les repose : le dictionnaire voit ainsi ses phrases entières.

const LF = 'https://www.legifrance.gouv.fr';

// ── Articles de codes ────────────────────────────────────────────────────────
// Clé « CODE:numéro » → identifiant LEGIARTI de la version en vigueur au
// 01/10/2026. Légifrance signale lui-même les versions ultérieures.
const ARTICLES = {
  // Code de la sécurité sociale
  'CSS:L136-1':   'LEGIARTI000033712581',
  'CSS:L136-2':   'LEGIARTI000042683568',
  'CSS:L136-8':   'LEGIARTI000054336623',
  'CSS:L137-12':  'LEGIARTI000053282399',
  'CSS:L222-1':   'LEGIARTI000036503965',
  'CSS:L241-2':   'LEGIARTI000046805562',
  'CSS:L241-3':   'LEGIARTI000051289018',
  'CSS:L241-5':   'LEGIARTI000037947554',
  'CSS:L241-6':   'LEGIARTI000038834567',
  'CSS:L241-13':  'LEGIARTI000053280526',
  'CSS:L241-17':  'LEGIARTI000037947458',
  'CSS:L241-18':  'LEGIARTI000038610232',
  'CSS:L242-1':   'LEGIARTI000053282401',
  'CSS:L242-5':   'LEGIARTI000053282379',
  'CSS:L323-1':   'LEGIARTI000031687210',
  'CSS:L325-1':   'LEGIARTI000048702040',
  'CSS:L911-7':   'LEGIARTI000031686110',
  'CSS:L921-1':   'LEGIARTI000047452643',
  'CSS:D241-7':   'LEGIARTI000054252241',
  'CSS:D242-1':   'LEGIARTI000046317928',
  'CSS:D242-4':   'LEGIARTI000053302465',
  'CSS:R323-1':   'LEGIARTI000054856818',
  'CSS:R323-4':   'LEGIARTI000051226486',
  // Code du travail
  'CT:L1226-1':   'LEGIARTI000054331868',
  'CT:L1226-23':  'LEGIARTI000054336510',
  'CT:L1234-1':   'LEGIARTI000006901112',
  'CT:L1237-7':   'LEGIARTI000047453558',
  'CT:L1237-9':   'LEGIARTI000052437168',
  'CT:L1242-2':   'LEGIARTI000037312980',
  'CT:L3121-1':   'LEGIARTI000033020517',
  'CT:L3121-3':   'LEGIARTI000033020510',
  'CT:L3121-13':  'LEGIARTI000033020461',
  'CT:L3121-27':  'LEGIARTI000033020376',
  'CT:L3121-30':  'LEGIARTI000033020367',
  'CT:L3121-36':  'LEGIARTI000033020341',
  'CT:L3122-2':   'LEGIARTI000033020186',
  'CT:L3122-5':   'LEGIARTI000033020171',
  'CT:L3123-8':   'LEGIARTI000033020061',
  'CT:L3123-33':  'LEGIARTI000033020661',
  'CT:L3133-6':   'LEGIARTI000033020878',
  'CT:L3141-1':   'LEGIARTI000033020838',
  'CT:L3141-3':   'LEGIARTI000033020826',
  'CT:L3141-23':  'LEGIARTI000033020705',
  'CT:L3141-24':  'LEGIARTI000049461568',
  'CT:L3142-1':   'LEGIARTI000045205234',
  'CT:L3231-1':   'LEGIARTI000006902830',
  'CT:L3243-4':   'LEGIARTI000020625846',
  'CT:L3245-1':   'LEGIARTI000027566295',
  'CT:L4153-1':   'LEGIARTI000037386047',
  'CT:L5213-19':  'LEGIARTI000037388842',
  'CT:D1226-1':   'LEGIARTI000018537770',
  'CT:D1226-3':   'LEGIARTI000019225874',
  'CT:D3121-24':  'LEGIARTI000033509251',
  'CT:R1234-2':   'LEGIARTI000035644692',
  'CT:R3243-1':   'LEGIARTI000048841718',
  'CT:R3243-2':   'LEGIARTI000048841713',
  'CT:R5213-76':  'LEGIARTI000049127323',
  // Code de l'action sociale et des familles
  'CASF:L243-4':  'LEGIARTI000048598243',
  'CASF:L243-5':  'LEGIARTI000037075109',
  'CASF:L243-6':  'LEGIARTI000048598232',
  'CASF:R243-5':  'LEGIARTI000052140908',
  'CASF:R243-6':  'LEGIARTI000052140887',
  'CASF:R243-9':  'LEGIARTI000052140843',
  'CASF:R243-10': 'LEGIARTI000052140878',
  // Code général des impôts
  'CGI:154 quinquies': 'LEGIARTI000054336634',
  'CGI:81 quater':     'LEGIARTI000046195916',
  'CGI:204 A':         'LEGIARTI000033812067',
};
// Volontairement absents (vérifiés, à NE PAS ajouter sans correction de la
// citation) : CSS L241-6-1, D241-3-2, L834-1, R1221-1 et CASF L14-10-4
// n'existent pas (ou plus) sous ces numéros ; CGFP L712-3 existe mais traite
// du classement des policiers, pas de la CNRACL.

// Marqueurs de code : la citation d'un article se rattache au code nommé dans
// la même portion de phrase (« CSS art. L241-3 », « art. L. 911-7 CSS »,
// « article D. 242-1 du code de la sécurité sociale », « CT L3121-36 »).
// « CSS LU » est le code luxembourgeois : exclu.
const MARQUEURS = [
  ['CSS',  /\bCSS\b(?!\s*LU)|code de la sécurité sociale|C\.\s?séc\.\s?soc\./gi],
  ['CT',   /\bCT\b|code du travail|C\.\s?trav\./gi],
  ['CASF', /\bCASF\b|code de l['’]action sociale et des familles/gi],
  ['CGI',  /\bCGI\b|code général des impôts/gi],
  ['CGFP', /\bCGFP\b|code général de la fonction publique/gi],
];

// Numéro d'article : « L241-13 », « L. 241-13 », « D.1226-1 ».
const RE_ARTICLE = /\b([LRD])\.?\s?(\d{1,4}(?:-\d+){1,3})\b/g;
// Articles du CGI (numérotation sans lettre de partie).
const RE_ARTICLE_CGI = /\b(154 quinquies|81 quater|204 A)\b/g;

// Portions de phrase : une citation ne franchit ni tiret long, ni point-virgule,
// ni fin de ligne, ni fin de phrase (point suivi d'un mot qui commence par une
// majuscule puis une minuscule — « art. L241 » n'est donc pas coupé).
const RE_COUPURE = /—|;|\n|[.!?]\s+(?=[A-ZÉÈÀÂÎÔ][a-zéèàâêîôûç])/g;

// ── Textes : lois, ordonnances, décrets, arrêtés ─────────────────────────────
// [motif, URL]. L'ordre compte : les motifs les plus précis d'abord, un
// passage déjà lié n'est plus examiné.
const jorf = id => `${LF}/jorf/id/${id}`;
const JUSTEL = 'https://www.ejustice.just.fgov.be/eli';
const N = 'n[°o]\\s?';
// Les références § du bulletin passent par refs.rs, qui traduit les mots
// descriptifs (« Loi du » → « Law of », « Gesetz vom »…) mais jamais les
// numéros ni les dates. Les motifs de ces textes acceptent donc les six
// langues, avec exactement les traductions de refs.rs.
const LOI    = '(?:loi|law|gesetz|wet|legge|ley)';
const DECRET = '(?:décret|decree|dekret|decreet|decreto)';
const ARRETE = '(?:arrêté|decree|erlass|besluit|decreto|orden)';
const ORDO   = '(?:ordonnance|order|verordnung|verordening|ordinanza|orden)';
const DU     = '(?:du|of|vom|van|del)';
// Avec `prefixe`, le motif a deux groupes : (contexte)(citation) ; seule la
// citation est liée. Pas de lookbehind : les anciens WebKitGTK (version
// bureau Linux) ne le connaissent pas et l'erreur bloquerait toute la page.
const t = (src, url, prefixe = false) => [new RegExp(src, 'gi'), url, prefixe];

const TEXTES = [
  // LFSS 2018, art. 8 (suppression des cotisations salariales maladie/chômage).
  t('(\\(LFSS 2018\\), )(art\\. 8)\\b', `${LF}/loda/article_lc/LEGIARTI000042683772`, true),
  t(`\\b${LOI} ${N}2017-1836(?: ${DU} 30\\/12\\/2017)?(?: \\(LFSS 2018\\))?|\\bLFSS 2018\\b`, jorf('JORFTEXT000036339090')),
  // Ordonnances fondatrices de 1945 (citées ensemble : « Ordonnances des 4 et 19 octobre 1945 »).
  t('\\bOrdonnances des 4(?= et 19 octobre 1945)', jorf('JORFTEXT000000698857')),
  t('(Ordonnances des 4 et )(19 octobre 1945)', jorf('JORFTEXT000000333985'), true),
  t(`\\b${ORDO} ${N}45-2250(?: ${DU} 4\\/10\\/1945)?`, jorf('JORFTEXT000000698857')),
  t(`\\bordonnance ${N}45-993\\b`, jorf('JORFTEXT000000338923')),
  t(`\\bloi ${N}47-1465\\b`, jorf('JORFTEXT000000315799')),
  t(`\\b${LOI} (?:${N})?2023-270\\b|\\bloi du 14 avril 2023\\b`, jorf('JORFTEXT000047445077')),
  t(`\\b${LOI} ${DU} 9(?:\\/04\\/| avril )1898\\b`, jorf('JORFTEXT000000692875')),
  t('\\bloi du 25 octobre 1919\\b', jorf('JORFTEXT000000869167')),
  t(`\\b${LOI} ${N}90-1168(?: ${DU} 29\\/12\\/1990)?|\\bloi de finances (?:du 29 décembre 1990|pour 1991)\\b`, jorf('JORFTEXT000000717191')),
  t(`\\b${ORDO} ${N}96-50(?: ${DU} 24\\/01\\/1996)?|\\bordonnance du 24 janvier 1996\\b`, jorf('JORFTEXT000000190291')),
  t(`\\b${DECRET} ${N}2026-509(?: ${DU} 12\\/06\\/2026| du 12 juin 2026)?`, jorf('JORFTEXT000054248488')),
  t(`\\b${LOI} ${N}2003-47(?: ${DU} 17\\/01\\/2003)?|\\bloi (?:Fillon )?du 17(?:\\/01\\/| janvier )2003\\b`, jorf('JORFTEXT000000594652')),
  t('\\b(?:loi (?:locale )?du|local law of|lokales gesetz vom|lokale wet van|legge locale del|ley local del) 1(?:er juin |\\/06\\/)1924\\b', `${LF}/loda/id/LEGITEXT000006069443`),
  t(`\\b${LOI} ${N}2018-1213(?: ${DU} 24\\/12\\/2018)?|\\b${LOI} ${DU} 24\\/12\\/2018\\b`, jorf('JORFTEXT000037851899')),
  t(`\\b${DECRET} ${N}2018-194(?: ${DU} 21\\/03\\/2018)?`, jorf('JORFTEXT000036735805')),
  t(`\\b${ARRETE} ${DU} 28(?:\\/12\\/| décembre )2006\\b`, jorf('JORFTEXT000000823047')),
  t(`\\b${ARRETE} ${DU} 16(?:\\/01\\/| janvier )2025\\b`, jorf('JORFTEXT000051020705')),
  // Frais professionnels : abrogé le 07/09/2025 (remplacé par l'arrêté du
  // 04/09/2025). Lié tel que cité ; Légifrance affiche l'abrogation.
  t(`\\b${ARRETE} ${DU} 20(?:\\/12\\/| décembre )2002\\b`, jorf('JORFTEXT000000782916')),
  t(`\\b${ARRETE} ${DU} 25(?:\\/02\\/| février )2025\\b`, jorf('JORFTEXT000051254024')),
  t(`\\b${DECRET} ${N}2025-86\\b(?: ${DU} 30\\/01\\/2025| du 30 janvier 2025)?`, jorf('JORFTEXT000051070354')),
  t(`\\b${LOI} ${N}83-634(?: ${DU} 13\\/07\\/1983)?`, jorf('JORFTEXT000000504704')),
  t(`\\bL\\. 84-53\\b`, jorf('JORFTEXT000000320434')),
  t('\\barrêté du 19\\/12\\/2023\\b', jorf('JORFTEXT000048708693')),   // PMSS 2024
  t(`\\bloi ${N}2014-288(?: du 0?5\\/03\\/2014| du 5 mars 2014)?`, jorf('JORFTEXT000028683576')),
  t(`\\bloi ${N}2018-771\\b|\\bloi Avenir professionnel\\b`, jorf('JORFTEXT000037367660')),
  t('\\bloi TEPA\\b', jorf('JORFTEXT000000278649')),
  t(`\\bdécret ${N}2023-1216\\b`, jorf('JORFTEXT000048604676')),
  t(`\\bdécret ${N}2003-1306\\b`, jorf('JORFTEXT000000611945')),
  t(`\\bdécret ${N}70-1277\\b`, jorf('JORFTEXT000000306984')),
  // Transports routiers (IDCC 0016) : convention et arrêtés d'extension. Deux
  // arrêtés portent la date du 19 décembre 2023 : celui de l'avenant n° 77
  // (frais de déplacement) et celui de l'accord salarial du 11 octobre 2023.
  t('\\b(?:Convention du|Agreement of|Abkommen vom|Overeenkomst van|Convenzione del|Convenio del) 21\\/12\\/1950 \\(IDCC 0016\\)', `${LF}/conv_coll/id/KALICONT000005635624`),
  t('(avenant n° 77 \\(11 octobre 2023\\) a été étendu par )(arrêté du 19 décembre 2023)', jorf('JORFTEXT000048642553'), true),
  t('\\barrêté du 19 décembre 2023\\b', jorf('JORFTEXT000048642564')),
  t('\\barrêté du 3 février 2026\\b', jorf('JORFTEXT000053447794')),
  t('\\barrêté du 7 avril 2026\\b', jorf('JORFTEXT000053788378')),
  t('\\barrêté du 12 février 2026\\b', jorf('JORFTEXT000053642426')),
  t('\\barrêté du 22 juillet 2025\\b', jorf('JORFTEXT000051993998')),
  t('\\barrêté du 6 mai 2026\\b', jorf('JORFTEXT000054104263')),
  // Durée du travail dans les transports (2003-1242 abrogé en 2017, 2001-679 en 2009).
  t(`\\bdécret ${N}83-40\\b(?: du 26 janvier 1983)?`, jorf('JORFTEXT000000503264')),
  t(`\\bdécret ${N}2003-1242\\b(?: du 22 décembre 2003)?`, jorf('JORFTEXT000000416669')),
  t(`\\bdécret ${N}2001-679\\b`, jorf('JORFTEXT000000394140')),
  // Bulletin de paie (modèle rénové).
  t('\\barrêté du 25 février 2016\\b', jorf('JORFTEXT000032106923')),
  t('\\barrêté du 31 janvier 2023\\b', jorf('JORFTEXT000047096915')),
  t('\\barrêté du 11 août 2025\\b', jorf('JORFTEXT000052097236')),
  // Histoire de la paie (quiz, anecdotes).
  t('\\bloi du 20 juin 1936\\b', jorf('JORFTEXT000000325218')),
  t('\\bloi du 21 juin 1936\\b', jorf('JORFTEXT000000325219')),
  t('\\bloi du 27 mars 1956\\b', jorf('JORFTEXT000000692216')),
  t('\\bloi du 16 mai 1969\\b', jorf('JORFTEXT000000511693')),
  t('\\bordonnance du 16 janvier 1982\\b', jorf('JORFTEXT000000889135')),
  t('\\bloi du 11 février 1950\\b', jorf('JORFTEXT000000693160')),
  t('\\bloi du 2 janvier 1970\\b', jorf('JORFTEXT000000693898')),
  t('\\bloi du 19 janvier 1978\\b', jorf('JORFTEXT000000704804')),
  t('\\bloi de finances 2017\\b', jorf('JORFTEXT000033734169')),
  t('\\bloi du 30 juin 2004\\b', jorf('JORFTEXT000000622485')),
  t('\\bloi du 14 mars 1941\\b', jorf('JORFTEXT000000521099')),
  t('\\bordonnance du 26 mars 1982\\b', jorf('JORFTEXT000000888522')),
  t('\\bloi du 22 juillet 1993\\b', jorf('JORFTEXT000000545719')),
  t(`\\bloi du 9 novembre 2010\\b|\\bloi ${N}2010-1330\\b`, jorf('JORFTEXT000023022127')),
  t('\\bloi du 10 juillet 1987\\b', jorf('JORFTEXT000000512481')),
  t('\\bloi du 11 février 2005\\b', jorf('JORFTEXT000000809647')),
  t('\\bAubry I(?= et II)', jorf('JORFTEXT000000558109')),
  t('(Aubry I et )(II)\\b', jorf('JORFTEXT000000398162'), true),
  // Allemagne : loi entière (gesetze-im-internet.de).
  t('\\bSolZG\\b', 'https://www.gesetze-im-internet.de/solzg_1995/'),
  // Luxembourg : Mémorial (legilux.public.lu). Existence et titre vérifiés dans
  // les données officielles (data.legilux.public.lu). Le 17/12/2010 a vu quinze
  // lois : la n12 est celle des soins de santé, seule citée dans ce sens.
  t(`\\b${LOI} ${DU} 17\\/12\\/1925\\b`, 'https://legilux.public.lu/eli/etat/leg/loi/1925/12/17/n1/jo'),
  t(`\\b${LOI} ${DU} 27\\/07\\/1987\\b`, 'https://legilux.public.lu/eli/etat/leg/loi/1987/07/27/n1/jo'),
  t(`\\b${LOI} ${DU} 19\\/06\\/1998\\b`, 'https://legilux.public.lu/eli/etat/leg/loi/1998/06/19/n1/jo'),
  t(`\\b${LOI} ${DU} 17\\/12\\/2010\\b`, 'https://legilux.public.lu/eli/etat/leg/loi/2010/12/17/n12/jo'),
  // Belgique : Justel (ejustice.just.fgov.be, SPF Justice), numéros relevés par
  // date via l'ELI et titres contrôlés. Le CIR 92 n'y est plus mis à jour
  // depuis 2002 (Justel l'indique) : lié au texte, sans article.
  t(`\\b${LOI} (?:${DU} )?27\\/06\\/1969\\b`, `${JUSTEL}/loi/1969/06/27/1969062710/justel`),
  t(`\\b${LOI} (?:${DU} )?20\\/12\\/1999\\b`, `${JUSTEL}/loi/1999/12/20/2000022052/justel`),
  t('\\bAR (?:du )?16\\/05\\/2003\\b', `${JUSTEL}/arrete/2003/05/16/2003012302/justel`),
  t('\\bCIR ?92\\b', `${JUSTEL}/loi/1992/04/10/1992041050/justel`),
  // Royaume-Uni — legislation.gov.uk ; Irlande — irishstatutebook.ie ; Malte —
  // legislation.mt ; Australie — legislation.gov.au. Titres contrôlés (une
  // adresse inventée renvoie 404 ou une page vide). « Income Tax Act 2007 »
  // existe au Royaume-Uni ET en Nouvelle-Zélande : seul le libellé britannique
  // (suivi de « Finance Act 2024 ») est lié ; legislation.govt.nz impose une
  // vérification anti-robot, la Nouvelle-Zélande reste sans lien.
  t('\\bIncome Tax Act 2007(?= — Finance Act 2024)', 'https://www.legislation.gov.uk/ukpga/2007/3/contents'),
  t('\\bNational Insurance Contributions Act 2014\\b', 'https://www.legislation.gov.uk/ukpga/2014/7/contents'),
  t('\\bFinance Act 2024\\b', 'https://www.legislation.gov.uk/ukpga/2024/3/contents'),
  t('\\bTaxes Consolidation Act 1997\\b', 'https://www.irishstatutebook.ie/eli/1997/act/39/enacted/en/html'),
  t('\\bSocial Welfare Consolidation Act 2005\\b', 'https://www.irishstatutebook.ie/eli/2005/act/26/enacted/en/html'),
  t('\\bIncome Tax Act \\(Cap\\. 123\\)', 'https://legislation.mt/eli/cap/123/eng'),
  t('\\bSocial Security Act \\(Cap\\. 318\\)', 'https://legislation.mt/eli/cap/318/eng'),
  t('\\bIncome Tax Assessment Act 1997\\b', 'https://www.legislation.gov.au/C2004A05138/latest/text'),
  t('\\bMedicare Levy Act 1986\\b', 'https://www.legislation.gov.au/C2004A03351/latest/text'),
  t('\\bSuperannuation Guarantee \\(Administration\\) Act 1992\\b', 'https://www.legislation.gov.au/C2004A04402/latest/text'),
  // Suisse : Recueil officiel (modification AVS 21 de la LAVS).
  t('\\bRO 2023 92\\b', 'https://www.fedlex.admin.ch/eli/oc/2023/92/fr'),
];
// Volontairement absents : décret n° 2011-291 (c'est le régime SNCF, pas la
// CNRACL), décret n° 2015-390 (traitements de données de l'assurance maladie,
// pas les allocations familiales) ; lois des 5 avril 1910, 5 avril 1928,
// 30 avril 1930, 4 mars 1931 et 11 mars 1932 (non publiées sur Légifrance).

// ── Lois désignées par un sigle (étranger) ───────────────────────────────────
// Une loi se reconnaît à son sigle ou à son nom (« LAVS », « SGB V », « EStG »).
// Ses articles se citent « Art. 5 et 13 LAVS », « LAA, art. 15 », « SGB V
// §241-242 » : chaque numéro se rattache à la loi la plus proche dans la
// portion de phrase, et n'est lié que s'il figure dans la liste vérifiée de
// cette loi (un intervalle est lié à son premier article). Une loi citée sans
// article lié renvoie au texte entier.
//
// { re: sigle, url: texte entier, art: n → URL de l'article, arts: [vérifiés],
//   syntaxe: 'art' (« Art. N ») ou '§' (« §N ») }
const LOIS = [];
const loi = (re, url, arts = [], art = null, syntaxe = 'art') =>
  LOIS.push({ re: new RegExp(re, 'g'), url, arts, art, syntaxe });

// Allemagne — gesetze-im-internet.de (ministère fédéral de la Justice).
// Vérifié le 01/10/2026 : titre et intitulé de chaque paragraphe.
// « EStG 1988 » est la loi autrichienne : exclue.
const GII = 'https://www.gesetze-im-internet.de';
const gii = (id, arts) => loi(
  { sgb_3: '\\bSGB\\s?III\\b', sgb_5: '\\bSGB\\s?V\\b', sgb_6: '\\bSGB\\s?VI\\b',
    sgb_7: '\\bSGB\\s?VII\\b', sgb_11: '\\bSGB\\s?XI\\b', estg: '\\bEStG\\b(?!\\s?1988)' }[id],
  `${GII}/${id}/`, arts, n => `${GII}/${id}/__${n}.html`, '§');
gii('sgb_3', ['341', '342']);
gii('sgb_5', ['241', '242']);
gii('sgb_6', ['158', '160']);
gii('sgb_7', ['150', '162']);
gii('sgb_11', ['54', '55']);
gii('estg', ['32a', '38', '39', '51a']);
// Lois modificatives allemandes (GKV-VEG, PUEG, Qualifizierungschancengesetz,
// Jahressteuergesetz…) : publiées au Bundesgesetzblatt, pages non vérifiables
// (recht.bund.de injoignable, archive bgbl.de qui n'ouvre pas le texte
// demandé) — sans lien. Lois d'Église des Länder (« KiStG {land} ») : seize
// portails régionaux, pas encore relevés.

// Suisse — fedlex.admin.ch (Chancellerie fédérale). Identifiants ELI tirés du
// point SPARQL officiel (fedlex.data.admin.ch) par numéro RS ; ancres d'articles
// contrôlées dans la version applicable au 01/10/2026. LIFD art. 90a n'existe
// pas (borne de l'intervalle « 83-90a » cité) ; « ORIS » est en réalité l'OIS.
const FEDLEX = 'https://www.fedlex.admin.ch/eli/cc';
const fedlex = (re, eli, arts = []) => loi(re, `${FEDLEX}/${eli}/fr`, arts,
  n => `${FEDLEX}/${eli}/fr#art_${n.replace(/([0-9])([a-z])$/, '$1_$2')}`);
fedlex('\\bLAVS\\b|\\bRS 831\\.10\\b', '63/837_843_843', ['5', '13', '53']);
fedlex('\\bLAI\\b|\\bRS 831\\.20\\b', '1959/827_857_845', ['3']);
fedlex('\\bLAPG\\b|\\bRS 834\\.1\\b', '1952/1021_1046_1050', ['27']);
fedlex('\\bLACI\\b|\\bRS 837\\.0\\b', '1982/2184_2184_2184', ['3', '23']);
fedlex('\\bOACI\\b|\\bRS 837\\.02\\b', '1983/1205_1205_1205');
fedlex('\\bLAMal\\b|\\bLAMAL\\b|\\bRS 832\\.10\\b', '1995/1328_1328_1328', ['67', '77']);
fedlex('\\bLCA\\b|\\bRS 221\\.229\\.1\\b', '24/719_735_717');
fedlex('\\bLPP\\b|\\bRS 831\\.40\\b', '1983/797_797_797', ['2', '7', '8', '16']);
fedlex('\\bOPP 2\\b|\\bRS 831\\.441\\.1\\b', '1984/543_543_543', ['8']);
fedlex('\\bLIFD\\b|\\bRS 642\\.11\\b', '1991/1184_1184_1184', ['83', '98']);
fedlex('\\bORIS\\b|\\bOIS\\b|\\bRS 642\\.118\\.2\\b', '2018/274');
fedlex('\\bLAA\\b|\\bRS 832\\.20\\b', '1982/1676_1676_1676', ['15', '61', '92', '93']);
fedlex('\\bOLAA\\b|\\bRS 832\\.202\\b', '1983/38_38_38');
fedlex('\\bLAFam\\b|\\bRS 836\\.2\\b', '2008/51', ['3', '5']);
fedlex('\\bConstitution fédérale\\b', '1999/404', ['111']);

// Luxembourg — Code de la sécurité sociale consolidé (Legilux). Les ancres
// d'articles ne sont pas vérifiables (page rendue en JavaScript, sans version
// HTML exposée) : « CSS LU » renvoie au code entier. Lois du 07/10/1960, du
// 12/03/1969 et RGD des 29/12/1995 et 29/09/2017 : introuvables, sans lien.
loi('\\bCSS LU\\b', 'https://legilux.public.lu/eli/etat/leg/code/securite_sociale');

// Italie — Normattiva (portail officiel de la législation en vigueur), URN
// « urn:nir:stato:type:année;numéro~artN ». Chaque acte et chaque article cité
// vérifiés le 01/10/2026 (titre exact ; un article inexistant renvoie à
// l'article 1, contrôle fait). « D.P.R. 663/1979 » (quiz) est en réalité le
// décret-loi n° 663 du 30/12/1979 : Normattiva y renvoie, on suit.
const NORMATTIVA = 'https://www.normattiva.it/uri-res/N2Ls?urn:nir:stato:';
const SIGLE_IT = {
  'legge': 'L\\.',
  'decreto.legge': 'D\\.?L\\.?',
  'decreto.legislativo': 'D\\.\\s?Lgs\\.',
  'decreto.del.presidente.della.repubblica': 'D\\.?P\\.?R\\.?',
};
const nir = (type, annee, num, arts = [], alias = '') => {
  const urn = `${NORMATTIVA}${type}:${annee};${num}`;
  loi(`\\b${SIGLE_IT[type]} ${num}\\/${annee}\\b${alias ? '|' + alias : ''}`, urn, arts, n => `${urn}~art${n}`);
};
nir('decreto.legge', 2022, 115, ['20']);
nir('legge', 2022, 142);
nir('decreto.legislativo', 2001, 151, ['16', '17', '22']);
nir('legge', 2019, 160);
nir('legge', 2024, 207, ['1']);
nir('decreto.legislativo', 2015, 22);
nir('legge', 2012, 92, ['2']);
nir('legge', 2012, 228);
nir('decreto.legislativo', 1997, 446, ['50']);
nir('decreto.del.presidente.della.repubblica', 1965, 1124, ['268']);
nir('legge', 1969, 153, ['12']);
nir('decreto.del.presidente.della.repubblica', 1976, 1026);
nir('legge', 2022, 197, ['1']);
nir('legge', 2023, 213, ['1']);
nir('legge', 1982, 297, ['2']);
nir('legge', 2006, 296, ['1']);
nir('decreto.del.presidente.della.repubblica', 1986, 917, ['11', '17', '23'], '\\bTUIR\\b');
nir('legge', 1995, 335);
nir('decreto.legislativo', 2015, 81);
nir('legge', 2025, 199);
nir('legge', 2021, 234);
nir('decreto.legislativo', 1994, 509);
loi('\\bD\\.?P\\.?R\\.? 663\\/1979\\b|\\bD\\.?L\\.? 663\\/1979\\b', `${NORMATTIVA}decreto.legge:1979;663`);

// Espagne — BOE (Boletín Oficial del Estado), textes consolidés. Identifiants
// vérifiés par l'API officielle (datosabiertos) ; ancres d'articles « #a33 »
// contrôlées dans la page. « LGSS art. 19 bis » : aucun article de ce numéro
// dans la LGSS consolidée, non lié. L'ordre de cotisation 2026 n'a pas encore
// de version consolidée : lié au texte publié, sans article.
const BOE = 'https://www.boe.es/buscar/act.php?id=';
const boe = (re, id, arts = []) => loi(re, BOE + id, arts, n => `${BOE}${id}#a${n}`);
boe('\\bRDL 2\\/2015\\b|\\bET\\b(?= \\(RDL)', 'BOE-A-2015-11430', ['33']);
boe('\\bLGSS\\b|\\bRDL 8\\/2015\\b', 'BOE-A-2015-11724', ['7', '143', '144', '270']);
boe('\\bLey 21\\/2021\\b', 'BOE-A-2021-21652', ['2']);
boe('\\bRDL 2\\/2023\\b', 'BOE-A-2023-6967');
loi('\\bOrden PJC\\/297\\/2026\\b', 'https://www.boe.es/diario_boe/txt.php?id=BOE-A-2026-7296');

// Portugal — Diário da República (diariodarepublica.pt). Site rendu en
// JavaScript : chaque page a été ouverte dans un navigateur et son titre lu
// (une adresse inventée reste vide). Les articles n'ont pas d'adresse stable
// exposée : on lie les textes. « Lei OE {année} » (une par an) : non liée.
const DRE = 'https://diariodarepublica.pt/dr/';
loi('\\bCIRS\\b', DRE + 'legislacao-consolidada/lei/2014-70048167-70051792');
loi('\\bLei 110\\/2009\\b|\\bCódigo Contributivo\\b', DRE + 'legislacao-consolidada/lei/2009-34514575');
loi('\\bLei 98\\/2009\\b', DRE + 'legislacao-consolidada/lei/2009-58661980');
loi('\\bDL 210\\/2015\\b', DRE + 'detalhe/decreto-lei/210-2015-70386137');

// États-Unis — Code fédéral via GovInfo (Government Publishing Office) : le
// service de liens officiel ouvre la section demandée, refuse une section
// inexistante. Code californien : leginfo.legislature.ca.gov (contenu de la
// section contrôlé). Le site de la SSA bloque l'accès : « Social Security
// Act » reste sans lien.
// Pays-Bas — wetten.overheid.nl (identifiants BWBR, titres contrôlés ; une
// adresse inventée renvoie 404). Les sigles des assurances (AOW, WW, WIA,
// Wlz) renvoient à leur loi. « Belastingplan {année} » : non lié.
const wetten = (re, bwbr) => loi(re, `https://wetten.overheid.nl/${bwbr}`);
wetten('\\bWet IB 2001\\b', 'BWBR0011353');
wetten('\\bWfsv\\b', 'BWBR0017745');
wetten('\\bZvw\\b', 'BWBR0018450');
wetten('\\bAOW\\b', 'BWBR0002221');
wetten('\\bWW\\b', 'BWBR0004045');
wetten('\\bWIA\\b', 'BWBR0019057');
wetten('\\bWlz\\b', 'BWBR0035917');

// Suède — Svensk författningssamling (riksdagen.se) ; Finlande — Finlex ;
// Danemark — Retsinformation, consolidation « GÆLDENDE » relevée dans la page
// (une loi danoise change d'adresse à chaque nouvelle consolidation : à
// revérifier lors des mises à jour de barèmes).
const SFS = 'https://www.riksdagen.se/sv/dokument-och-lagar/dokument/svensk-forfattningssamling/';
loi('\\bInkomstskattelagen \\(1999:1229\\)|\\bInkomstskattelag(?:en)?\\b', SFS + 'inkomstskattelag-19991229_sfs-1999-1229/');
loi('\\bSocialavgiftslagen \\(2000:980\\)|\\bSocialavgiftslag(?:en)?\\b', SFS + 'socialavgiftslag-2000980_sfs-2000-980/');
loi('\\bTuloverolaki\\b', 'https://www.finlex.fi/fi/lainsaadanto/1992/1535');
loi('\\bSairausvakuutuslaki\\b', 'https://www.finlex.fi/fi/lainsaadanto/2004/1224');
loi('\\bPersonskatteloven\\b', 'https://www.retsinformation.dk/eli/lta/2021/1284');
loi('\\bArbejdsmarkedsbidragsloven\\b', 'https://www.retsinformation.dk/eli/lta/2020/121');
loi('\\bATP-loven\\b', 'https://www.retsinformation.dk/eli/lta/2024/1142');

// Autriche — RIS (Rechtsinformationssystem, Chancellerie fédérale), version
// en vigueur ; titres contrôlés (un numéro inventé renvoie 404).
const RIS = n => `https://www.ris.bka.gv.at/GeltendeFassung.wxe?Abfrage=Bundesnormen&Gesetzesnummer=${n}`;
loi('\\bASVG\\b', RIS(10008147));
loi('\\bEStG 1988\\b', RIS(10004570));
// Monaco : « Caisses Sociales de Monaco » désigne un organisme, pas un texte.
// Andorre — textes publiés au BOPA, servis par le Consell General (titre exact
// retrouvé dans le PDF).
loi('\\bLlei 17\\/2008\\b', 'https://www.consellgeneral.ad/fitxers/documents/lleis-2008/llei-17-2008.pdf');
loi('\\bLlei 5\\/2014\\b', 'https://www.consellgeneral.ad/ca/arxiu/arxiu-de-lleis-i-textos-aprovats-en-legislatures-anteriors/vi-legislatura-2011-2015/copy_of_lleis-aprovades/llei-5-2014-del-24-d2019abril-de-l2019impost-sobre-la-renda-de-les-persones-fisiques/at_download/PDF');

// Pologne — Dziennik Ustaw (dziennikustaw.gov.pl, Centre gouvernemental de
// législation). Actes confirmés par l'API ELI officielle du Sejm, page
// publique rendue et lue (ISAP est derrière un pare-feu anti-robot).
loi('\\bUstawa o PIT\\b', 'https://dziennikustaw.gov.pl/DU/1991/s/80/350');
loi('\\bUstawa o systemie ubezpieczeń społecznych', 'https://dziennikustaw.gov.pl/DU/1998/s/137/887');
loi('\\bUstawa o świadczeniach opieki zdrowotnej', 'https://dziennikustaw.gov.pl/DU/2004/s/210/2135');

// Tchéquie — e-Sbírka (sbírka zákonů officielle) ; Slovaquie — Slov-Lex
// (ministère de la Justice). Pages rendues et titres lus. « Zákony o pojistném »
// (au pluriel) vise deux lois (589/1992 et 592/1992) : non lié.
loi('\\bZákon o daních z příjmů', 'https://www.e-sbirka.cz/sb/1992/586');
loi('\\bZákon o dani z príjmov', 'https://www.slov-lex.sk/pravne-predpisy/SK/ZZ/2003/595/');
loi('\\bZákon o sociálnom poistení', 'https://www.slov-lex.sk/pravne-predpisy/SK/ZZ/2003/461/');
loi('\\bZákon o zdravotnom poistení', 'https://www.slov-lex.sk/pravne-predpisy/SK/ZZ/2004/580/');

// Roumanie — Portail législatif du ministère de la Justice (titre contrôlé ;
// un identifiant inventé renvoie « Error »). Hongrie : njt.hu refuse toute
// connexion depuis l'outil de vérification — lois hongroises sans lien.
loi('\\bLegea 227\\/2015\\b|\\bCodul fiscal\\b', 'https://legislatie.just.ro/Public/DetaliiDocument/171282');

// Slovénie — PISRS (Pravno-informacijski sistem, gouvernement) : titres lus
// après rendu, un identifiant inventé reste vide.
loi('\\bZDoh-2\\b|\\bZakon o dohodnini\\b', 'https://pisrs.si/pregledPredpisa?id=ZAKO4697');
loi('\\bZPIZ-2\\b', 'https://pisrs.si/pregledPredpisa?id=ZAKO6280');
loi('\\bZZVZZ\\b', 'https://pisrs.si/pregledPredpisa?id=ZAKO213');
// Croatie — Narodne novine (Journal officiel) : publication d'origine, le
// Journal ne tient pas de version consolidée. Titres contrôlés.
const NN = 'https://narodne-novine.nn.hr/clanci/sluzbeni/';
loi('\\bZakon o porezu na dohodak\\b', NN + '2016_12_115_2525.html');
loi('\\bZakon o mirovinskom osiguranju\\b', NN + '2013_12_157_3290.html');
loi('\\bZakon o obveznom zdravstvenom osiguranju\\b(?! i zdravstvenoj)', NN + '2013_06_80_1666.html');
// Estonie — Riigi Teataja (version en vigueur, adresse par sigle ; un sigle
// inventé affiche « ei leitud ») ; Lettonie — likumi.lv (Latvijas Vēstnesis,
// éditeur officiel) ; Lituanie — e-Seimas, version consolidée (« asr »). La loi
// d'assurance sociale I-1336 a été entièrement reformulée par la loi XII-2508,
// dont la version consolidée est le texte en vigueur.
loi('\\bTulumaksuseadus\\b', 'https://www.riigiteataja.ee/akt/TuMS');
loi('\\bSotsiaalmaksuseadus\\b', 'https://www.riigiteataja.ee/akt/SMS');
loi('«Par iedzīvotāju ienākuma nodokli»|\\bPar iedzīvotāju ienākuma nodokli\\b', 'https://likumi.lv/ta/id/56880');
loi('«Par valsts sociālo apdrošināšanu»|\\bPar valsts sociālo apdrošināšanu\\b', 'https://likumi.lv/ta/id/45466');
loi('\\bGyventojų pajamų mokesčio įstatymas', 'https://e-seimas.lrs.lt/portal/legalAct/lt/TAD/TAIS.171369/asr');
loi('\\bValstybinio socialinio draudimo įstatymas', 'https://e-seimas.lrs.lt/portal/legalAct/lt/TAD/adf586f244fe11e68f45bcf65e0a17ee/asr');
// Chine — site de l'Assemblée populaire nationale (npc.gov.cn), version en
// vigueur (révisions de 2018 vérifiées dans le texte). « 国税发〔2018〕164号 »
// est en réalité le 财税〔2018〕164号 (Finances / administration fiscale) :
// citation à corriger, non liée.
loi('个人所得税法', 'http://www.npc.gov.cn/npc/c2/c183/c198/201905/t20190523_10668.html');
loi('社会保险法', 'http://www.npc.gov.cn/zgrdw/npc/xinwen/2019-01/07/content_2070267.htm');
// Corée — Centre national d'information juridique (law.go.kr, ministère de la
// Législation), adresse par nom de loi = version en vigueur ; un nom inventé
// ouvre une page d'erreur (contrôlé).
for (const nom of ['소득세법', '국민연금법', '국민건강보험법', '노인장기요양보험법', '고용보험법'])
  loi(nom, `https://www.law.go.kr/법령/${nom}`);
// Brésil — Planalto (présidence de la République), textes consolidés ; ancre
// d'article « #art22 » contrôlée.
const PLANALTO = 'https://www.planalto.gov.br/ccivil_03/leis/';
loi('\\bLei 8\\.036\\/1990\\b', PLANALTO + 'l8036consol.htm');
loi('\\bLei 8\\.212\\/1991\\b', PLANALTO + 'l8212cons.htm', ['22'], n => `${PLANALTO}l8212cons.htm#art${n}`);
// Inde (India Code : 403 / délai dépassé), Mexique (diputados.gob.mx et DOF
// injoignables), Émirats (Cloudflare), Nouvelle-Zélande (vérification
// anti-robot), Hongrie (connexion refusée), Québec (LégisQuébec 403 / 502) :
// sources officielles non vérifiables depuis l'outil — sans lien.
// Grèce (et.gr redirige vers une adresse IP brute), Chypre (fichiers du fisc
// déplacés, portail gov.cy en 403), Bulgarie (aucune version consolidée
// officielle stable) : sans lien.

// Canada — Lois codifiées (laws-lois.justice.gc.ca, ministère de la Justice) :
// une page par article, contrôlée (un article inexistant renvoie 404). La loi
// annuelle « L.C. 2018, ch. 12 » est liée sans article. Ontario : Lois-en-ligne
// (titre lu après rendu). Québec : LégisQuébec refuse tout accès automatisé
// (403 / 502) — références RLRQ non vérifiables, sans lien.
const JUSTICE = 'https://laws-lois.justice.gc.ca/fra/lois/';
const justice = (re, id, arts) => loi(re, `${JUSTICE}${id}/`, arts, n => `${JUSTICE}${id}/section-${n}.html`);
justice('\\bL\\.C\\. 1996, ch\\. 23\\b', 'E-5.6', ['66', '67', '68', '69']);
justice('\\bL\\.R\\.C\\. 1985, ch\\. C-8\\b', 'C-8', ['8', '9']);
justice('\\bL\\.R\\.C\\. 1985, ch\\. 1 \\(5e suppl\\.\\)', 'I-3.3', ['117']);
loi('\\bL\\.C\\. 2018, ch\\. 12\\b', 'https://laws-lois.justice.gc.ca/fra/LoisAnnuelles/2018_12/');
loi('\\bL\\.O\\. 2007, ch\\. 11\\b', 'https://www.ontario.ca/fr/lois/loi/07t11');

const USC26 = n => `https://www.govinfo.gov/link/uscode/26/${n}?type=usc&year=mostrecent&link-type=html`;
loi('\\b26 U\\.S\\.C\\.', USC26(1), ['1', '63', '3101', '3111', '3301'], USC26, '§');
loi('\\bCalifornia Unemployment Insurance Code\\b',
  'https://leginfo.legislature.ca.gov/faces/codes_displaySection.xhtml?lawCode=UIC&sectionNum=984', ['984'],
  n => `https://leginfo.legislature.ca.gov/faces/codes_displaySection.xhtml?lawCode=UIC&sectionNum=${n}`, '§');

// Numéros d'articles : « Art. 5 et 13 », « art. 83-98 », « Art. 7 et 16 »
// (connecteurs des six langues, traduits par refs.rs), « §32a », « §241-242 ».
// Numéros décimaux admis (« 117.1 », « 50.0.1 » au Canada).
const RE_ART_LISTE = /\b[Aa]rt(?:\.|icle|ikel|icolo|ículo)?\s?(\d+(?:\.\d+)*[a-z]?(?:\s?-\s?\d+(?:\.\d+)*[a-z]?)?(?:\s?(?:et|and|und|en|e|y|,)\s?\d+(?:\.\d+)*[a-z]?(?:\s?-\s?\d+(?:\.\d+)*[a-z]?)?)*)/g;
const RE_NUM = /\d+(?:\.\d+)*[a-z]?(?:\s?-\s?\d+(?:\.\d+)*[a-z]?)?/g;
const LETTRE = /[A-Za-zÀ-ÖØ-öø-ÿ]/;
const RE_PARA = /§\s?(\d+[a-z]?)(?:\s?-\s?\d+[a-z]?)?/g;

// Préfiltre : la plupart des nœuds de texte ne citent rien.
// On cherche ce qui n'est jamais traduit : numéros d'articles, numéros de
// textes, dates, années, et les quelques intitulés propres.
const RE_INDICE = /[LRD]\.?\s?\d{1,4}-\d|n[°o]\s?\d|\d\/\d\d\/\d{4}|\b(?:18|19|20)\d\d\b|LFSS|Aubry|TEPA|Avenir professionnel|quinquies|quater|204 A|§|Constitution|Cap\. \d|\bAct\b|\bCode\b|Código|Wfsv|Zvw|Wlz|lag(?:en)?\b|laki\b|loven\b|Ustawa|Zákon|Zakon|törvény|Legea|Νόμος|Ν\. \d|Закон|Кодекс|Likums|įstatymas|seadus|法|법/i;
// Sigles de lois (LAVS, SGB V, EStG…) : au moins deux capitales, casse exacte.
const RE_SIGLE = /\b[A-Z][A-Za-z]*[A-Z]/;
const indice = t => RE_INDICE.test(t) || RE_SIGLE.test(t);

// Recherche les citations d'un texte brut. Renvoie des plages triées
// [{ debut, fin, url }] sans chevauchement.
export function trouverCitations(texte) {
  if (!texte || !indice(texte)) return [];
  const plages = [];
  const libre = (d, f) => plages.every(p => f <= p.debut || d >= p.fin);

  for (const [re, url, prefixe] of TEXTES) {
    re.lastIndex = 0;
    let m;
    while ((m = re.exec(texte))) {
      const d = prefixe ? m.index + m[1].length : m.index;
      const f = prefixe ? d + m[2].length : m.index + m[0].length;
      if (f > d && libre(d, f)) plages.push({ debut: d, fin: f, url });
      if (m[0].length === 0) re.lastIndex++;
    }
  }

  // Articles : découpage en portions, rattachement au code le plus proche.
  const bornes = [0];
  RE_COUPURE.lastIndex = 0;
  let c;
  while ((c = RE_COUPURE.exec(texte))) bornes.push(c.index + c[0].length);
  bornes.push(texte.length);

  for (let i = 0; i < bornes.length - 1; i++) {
    const a = bornes[i], b = bornes[i + 1];
    const portion = texte.slice(a, b);
    const marques = [];
    for (const [code, re] of MARQUEURS) {
      re.lastIndex = 0;
      let m;
      while ((m = re.exec(portion))) marques.push({ code, pos: m.index });
    }
    const codePour = pos => {
      if (!marques.length) return null;
      return marques.reduce((x, y) => Math.abs(y.pos - pos) < Math.abs(x.pos - pos) ? y : x).code;
    };
    const ajouter = (cle, d, f) => {
      const id = ARTICLES[cle];
      if (id && libre(a + d, a + f)) plages.push({ debut: a + d, fin: a + f, url: `${LF}/codes/article_lc/${id}` });
    };

    RE_ARTICLE.lastIndex = 0;
    let m;
    while ((m = RE_ARTICLE.exec(portion))) {
      const num = m[1] + m[2];
      let code = codePour(m.index);
      // Sans code nommé dans la portion : on ne lie que si le numéro n'existe
      // que dans un seul code de la table (« L3141-24 » seul = Code du travail).
      if (!code) {
        const codes = Object.keys(ARTICLES).filter(k => k.endsWith(':' + num));
        if (codes.length !== 1) continue;
        code = codes[0].split(':')[0];
      }
      ajouter(`${code}:${num}`, m.index, m.index + m[0].length);
    }
    if (marques.some(x => x.code === 'CGI')) {
      RE_ARTICLE_CGI.lastIndex = 0;
      while ((m = RE_ARTICLE_CGI.exec(portion))) ajouter(`CGI:${m[1]}`, m.index, m.index + m[0].length);
    }

    // Lois désignées par un sigle (Allemagne, Suisse…).
    const lois = [];
    for (const l of LOIS) {
      l.re.lastIndex = 0;
      while ((m = l.re.exec(portion))) {
        // \b ignore les lettres accentuées : « DÉLAI » contiendrait « LAI ».
        const fin = m.index + m[0].length;
        if (LETTRE.test(portion[m.index - 1] || '') || LETTRE.test(portion[fin] || '')) continue;
        lois.push({ l, pos: m.index, fin });
      }
    }
    if (lois.length) {
      const liees = new Set();
      const lier = (syntaxe, num, d, f) => {
        const cand = lois.filter(x => x.l.syntaxe === syntaxe);
        if (!cand.length) return;
        const x = cand.reduce((u, v) => Math.abs(v.pos - d) < Math.abs(u.pos - d) ? v : u);
        if (!x.l.arts.includes(num) || !libre(a + d, a + f)) return;
        plages.push({ debut: a + d, fin: a + f, url: x.l.art(num) });
        liees.add(x.l);
      };
      RE_ART_LISTE.lastIndex = 0;
      while ((m = RE_ART_LISTE.exec(portion))) {
        const debutListe = m.index + m[0].length - m[1].length;
        RE_NUM.lastIndex = 0;
        let n, premier = true;
        while ((n = RE_NUM.exec(m[1]))) {
          const num = n[0].match(/^\d+(?:\.\d+)*[a-z]?/)[0];
          // Le premier numéro emporte « Art. » dans le lien.
          const d = premier ? m.index : debutListe + n.index;
          lier('art', num, d, debutListe + n.index + n[0].length);
          premier = false;
        }
      }
      RE_PARA.lastIndex = 0;
      while ((m = RE_PARA.exec(portion))) lier('§', m[1], m.index, m.index + m[0].length);
      // Sans article lié : la première mention de la loi renvoie au texte entier
      // (« OPP 2 (RS 831.441.1) » ne donne qu'un lien).
      for (const x of lois.sort((u, v) => u.pos - v.pos)) {
        if (liees.has(x.l) || !libre(a + x.pos, a + x.fin)) continue;
        plages.push({ debut: a + x.pos, fin: a + x.fin, url: x.l.url });
        liees.add(x.l);
      }
    }
  }

  return plages.sort((x, y) => x.debut - y.debut);
}

// ── Application au DOM ───────────────────────────────────────────────────────
const CLASSE = 'loi-lien';
// Jamais de lien dans les contrôles (un choix de quiz deviendrait un lien),
// les zones de saisie, un lien existant, ni les libellés de ligne du bulletin
// (« Zvw — Health insurance ») : un clic y déplie l'explication, et le mode
// dactylo réécrit ces libellés lettre à lettre.
const EXCLUS = 'a,button,select,option,textarea,input,script,style,svg,[contenteditable],.no-loi-lien,'
  + '.mob-lbl,.mob-cot-lbl,tr.data-row > td:first-child';

function lierNoeud(n) {
  const texte = n.nodeValue;
  const plages = trouverCitations(texte);
  if (!plages.length) return;
  const frag = document.createDocumentFragment();
  let pos = 0;
  for (const p of plages) {
    if (p.debut > pos) frag.appendChild(document.createTextNode(texte.slice(pos, p.debut)));
    const a = document.createElement('a');
    a.className = CLASSE;
    a.href = p.url;
    a.target = '_blank';
    a.rel = 'noopener noreferrer';
    a.title = 'Texte officiel';
    a.textContent = texte.slice(p.debut, p.fin);
    frag.appendChild(a);
    pos = p.fin;
  }
  if (pos < texte.length) frag.appendChild(document.createTextNode(texte.slice(pos)));
  n.parentNode.replaceChild(frag, n);
}

// Pose les liens dans un sous-arbre.
export function lierDans(racine) {
  if (!racine) return;
  if (racine.nodeType === 3) {
    if (racine.parentElement && !racine.parentElement.closest(EXCLUS)) lierNoeud(racine);
    return;
  }
  if (racine.nodeType !== 1 || racine.closest?.(EXCLUS)) return;
  const walker = document.createTreeWalker(racine, NodeFilter.SHOW_TEXT, {
    acceptNode(n) {
      if (!indice(n.nodeValue)) return NodeFilter.FILTER_REJECT;
      return n.parentElement?.closest(EXCLUS) ? NodeFilter.FILTER_REJECT : NodeFilter.FILTER_ACCEPT;
    },
  });
  const noeuds = [];
  while (walker.nextNode()) noeuds.push(walker.currentNode);
  noeuds.forEach(lierNoeud);
}

// Retire les liens (avant une traduction) : le texte redevient un seul nœud.
export function delierDans(racine) {
  if (!racine) return;
  const parents = new Set();
  racine.querySelectorAll(`a.${CLASSE}`).forEach(a => {
    parents.add(a.parentNode);
    a.replaceWith(document.createTextNode(a.textContent));
  });
  parents.forEach(p => p.normalize());
}

// Surveillance : tout contenu ajouté à la page est lié dès son insertion.
// Seuls les ajouts de nœuds sont observés (pas les modifications de texte) :
// la frappe du mode dactylo et la traduction écrivent dans des nœuds
// existants et ne doivent pas être découpées en cours de route.
let _pause = false;
let _enAttente = new Set();
let _planifie = false;

export function suspendreLiens(oui) {
  _pause = oui;
  if (!oui) lierDans(document.body);
}

export function demarrerLiens() {
  lierDans(document.body);
  new MutationObserver(records => {
    if (_pause) return;
    for (const r of records) r.addedNodes.forEach(n => _enAttente.add(n));
    if (_planifie) return;
    _planifie = true;
    requestAnimationFrame(() => {
      _planifie = false;
      const lot = _enAttente;
      _enAttente = new Set();
      if (_pause) return;
      lot.forEach(n => { if (n.isConnected) lierDans(n); });
    });
  }).observe(document.body, { childList: true, subtree: true });

  // Un lien dans une ligne cliquable (cotisation, absence…) ne doit pas
  // replier la ligne : on arrête la propagation dès la phase de capture.
  // En version bureau (Tauri), le lien s'ouvre dans le navigateur système.
  document.addEventListener('click', e => {
    const a = e.target.closest?.(`a.${CLASSE}`);
    if (!a) return;
    e.stopPropagation();
    if (window.__TAURI_INTERNALS__) {
      e.preventDefault();
      import('@tauri-apps/plugin-shell').then(({ open }) => open(a.href));
    }
  }, true);
}
