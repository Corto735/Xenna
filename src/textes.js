// ── Gabarits de phrases à valeurs, en six langues ───────────────────────────
// Les dictionnaires de lang.js traduisent des nœuds de texte FIXES. Une phrase
// qui porte des chiffres (« le net payé passe de 2 394,02 € à… ») ne peut pas
// y figurer : sans gabarit, elle partait chez MyMemory (quota de 1 000 mots par
// jour) ou restait en français. Même principe que les explications côté Rust
// (crate::i18n) : un gabarit par langue, des paramètres NOMMÉS identiques dans
// les six, substitués à l'affichage. Ne jamais traduire un {paramètre}.
//
// Ordre des colonnes : [fr, en, de, nl, it, es]. Le néerlandais est celui de
// Belgique (terminologie RH belge), comme dans lang.js.
// Le HTML (<strong>, <sup>) est permis : les valeurs injectées sont déjà
// échappées ou formatées par l'appelant.

const IDX = { fr: 0, en: 1, de: 2, nl: 3, it: 4, es: 5 };

export const TEXTES = {
  // ── Droits ouverts par le mois
  'dr.resume': [
    '{pts} points de retraite complémentaire',
    '{pts} supplementary pension points',
    '{pts} Punkte Zusatzrente',
    '{pts} punten aanvullend pensioen',
    '{pts} punti di pensione complementare',
    '{pts} puntos de pensión complementaria',
  ],
  'dr.l.trim': [
    "Retraite de base : <strong>{n}/4 trimestres</strong> sur l'année à ce salaire",
    'Basic pension: <strong>{n}/4 quarters</strong> over the year at this pay',
    'Grundrente: <strong>{n}/4 Quartale</strong> im Jahr bei diesem Lohn',
    'Basispensioen: <strong>{n}/4 kwartalen</strong> over het jaar aan dit loon',
    'Pensione di base: <strong>{n}/4 trimestri</strong> nell’anno a questa retribuzione',
    'Pensión básica: <strong>{n}/4 trimestres</strong> en el año con este salario',
  ],
  'dr.l.compte': [
    'Salaire porté au compte retraite : <strong>{montant}</strong>{plafond}',
    'Pay credited to the pension account: <strong>{montant}</strong>{plafond}',
    'Dem Rentenkonto gutgeschrieben: <strong>{montant}</strong>{plafond}',
    'Op de pensioenrekening geboekt: <strong>{montant}</strong>{plafond}',
    'Retribuzione accreditata sul conto pensione: <strong>{montant}</strong>{plafond}',
    'Salario anotado en la cuenta de jubilación: <strong>{montant}</strong>{plafond}',
  ],
  'dr.l.plafond': [' (plafonné)', ' (capped)', ' (gedeckelt)', ' (begrensd)', ' (entro il massimale)', ' (con tope)'],
  'dr.l.points': [
    'Agirc-Arrco : <strong>{pts} points</strong>, soit {rente} de retraite par an',
    'Agirc-Arrco: <strong>{pts} points</strong>, i.e. {rente} of pension per year',
    'Agirc-Arrco: <strong>{pts} Punkte</strong>, also {rente} Rente pro Jahr',
    'Agirc-Arrco: <strong>{pts} punten</strong>, of {rente} pensioen per jaar',
    'Agirc-Arrco: <strong>{pts} punti</strong>, cioè {rente} di pensione all’anno',
    'Agirc-Arrco: <strong>{pts} puntos</strong>, es decir {rente} de pensión al año',
  ],
  'dr.l.cpf': [
    'CPF : <strong>{mois}</strong> ({annuel} par an, plafond {plafond}){motif}',
    'CPF (training account): <strong>{mois}</strong> ({annuel} a year, cap {plafond}){motif}',
    'CPF (Weiterbildungskonto): <strong>{mois}</strong> ({annuel} pro Jahr, Obergrenze {plafond}){motif}',
    'CPF (opleidingsrekening): <strong>{mois}</strong> ({annuel} per jaar, plafond {plafond}){motif}',
    'CPF (conto formazione): <strong>{mois}</strong> ({annuel} l’anno, tetto {plafond}){motif}',
    'CPF (cuenta de formación): <strong>{mois}</strong> ({annuel} al año, tope {plafond}){motif}',
  ],
  'dr.l.prorata': [' — au prorata, sous le mi-temps', ' — prorated, below half-time', ' — anteilig, unter halber Arbeitszeit', ' — naar rato, onder halftijds', ' — in proporzione, sotto la metà tempo', ' — a prorrata, por debajo de media jornada'],
  'dr.l.handicap': [' — majoré, travailleur handicapé', ' — increased, disabled worker', ' — erhöht, schwerbehinderter Beschäftigter', ' — verhoogd, werknemer met een handicap', ' — maggiorato, lavoratore con disabilità', ' — incrementado, trabajador con discapacidad'],
  'dr.l.esat': [' — majoré, ESAT', ' — increased, ESAT', ' — erhöht, ESAT', ' — verhoogd, ESAT', ' — maggiorato, ESAT', ' — incrementado, ESAT'],
  'dr.l.note': [
    "Ordre de grandeur sur ce seul mois : ces droits se calculent à l'année.",
    'Rough figure for this month alone: these rights are computed over the year.',
    'Größenordnung für diesen Monat allein: diese Rechte werden jährlich berechnet.',
    'Grootteorde voor deze maand alleen: deze rechten worden per jaar berekend.',
    'Ordine di grandezza per questo solo mese: questi diritti si calcolano su base annua.',
    'Orden de magnitud para este solo mes: estos derechos se calculan por año.',
  ],

  // ── Pastille « nouveau taux »
  'ev.badge': ['nouveau taux · {date}', 'new rate · {date}', 'neuer Satz · {date}', 'nieuw tarief · {date}', 'nuova aliquota · {date}', 'nuevo tipo · {date}'],
  'ev.titre': ['Taux modifié le {date} : {details}', 'Rate changed on {date}: {details}', 'Satz geändert am {date}: {details}', 'Tarief gewijzigd op {date}: {details}', 'Aliquota modificata il {date}: {details}', 'Tipo modificado el {date}: {details}'],
  'ev.sal': ['part salarié {a} → {b}', 'employee share {a} → {b}', 'Arbeitnehmeranteil {a} → {b}', 'werknemersaandeel {a} → {b}', 'quota dipendente {a} → {b}', 'parte del trabajador {a} → {b}'],
  'ev.pat': ['part patronale {a} → {b}', 'employer share {a} → {b}', 'Arbeitgeberanteil {a} → {b}', 'werkgeversaandeel {a} → {b}', 'quota datore {a} → {b}', 'parte de la empresa {a} → {b}'],

  // ── Alertes (code émis par calculs/alertes.rs)
  'al.smic.titre': ['Salaire de base inférieur au SMIC', 'Base salary below the minimum wage (SMIC)', 'Grundgehalt unter dem Mindestlohn (SMIC)', 'Basisloon onder het minimumloon (SMIC)', 'Retribuzione base inferiore al salario minimo (SMIC)', 'Salario base inferior al salario mínimo (SMIC)'],
  'al.smic.texte': [
    "Le salaire de base revient à {taux_h} € de l'heure pour {heures} heures par mois, sous le SMIC horaire de {smic_h} €. Le minimum pour ce temps de travail est de {minimum} € bruts. Seuls quelques cas y échappent (apprentis, jeunes de moins de 18 ans avec abattement).",
    'Base salary works out at €{taux_h} per hour for {heures} hours a month, below the hourly minimum wage of €{smic_h}. The minimum for this working time is €{minimum} gross. Only a few cases are exempt (apprentices, under-18s with a reduction).',
    'Das Grundgehalt entspricht {taux_h} € pro Stunde bei {heures} Stunden im Monat, unter dem Mindeststundenlohn von {smic_h} €. Das Minimum für diese Arbeitszeit beträgt {minimum} € brutto. Nur wenige Fälle sind ausgenommen (Auszubildende, unter 18-Jährige mit Abschlag).',
    'Het basisloon komt neer op {taux_h} € per uur voor {heures} uur per maand, onder het minimumuurloon van {smic_h} €. Het minimum voor deze arbeidsduur is {minimum} € bruto. Slechts enkele gevallen zijn uitgezonderd (leerlingen, jongeren onder 18 met vermindering).',
    'La retribuzione base equivale a {taux_h} € l’ora per {heures} ore al mese, sotto il salario minimo orario di {smic_h} €. Il minimo per questo orario è di {minimum} € lordi. Solo pochi casi sono esclusi (apprendisti, minori di 18 anni con riduzione).',
    'El salario base equivale a {taux_h} € por hora para {heures} horas al mes, por debajo del salario mínimo por hora de {smic_h} €. El mínimo para esta jornada es de {minimum} € brutos. Solo unos pocos casos quedan exentos (aprendices, menores de 18 años con reducción).',
  ],
  'al.partiel.titre': ['Temps partiel sous la durée minimale', 'Part-time below the minimum duration', 'Teilzeit unter der Mindestdauer', 'Deeltijd onder de minimale duur', 'Part-time sotto la durata minima', 'Jornada parcial por debajo de la duración mínima'],
  'al.partiel.texte': [
    "{hebdo} heures par semaine, sous la durée minimale de {min_hebdo} heures. C'est possible sur demande écrite et motivée du salarié, pour un étudiant de moins de 26 ans, ou si un accord de branche fixe une durée inférieure.",
    '{hebdo} hours a week, below the minimum of {min_hebdo} hours. This is allowed at the employee’s written, reasoned request, for a student under 26, or if a sector agreement sets a lower duration.',
    '{hebdo} Stunden pro Woche, unter der Mindestdauer von {min_hebdo} Stunden. Zulässig auf schriftlichen, begründeten Antrag des Arbeitnehmers, für Studierende unter 26 Jahren oder wenn ein Branchenabkommen eine kürzere Dauer vorsieht.',
    '{hebdo} uur per week, onder de minimale duur van {min_hebdo} uur. Dat kan op schriftelijk en gemotiveerd verzoek van de werknemer, voor een student jonger dan 26, of als een sectorakkoord een kortere duur vastlegt.',
    '{hebdo} ore a settimana, sotto la durata minima di {min_hebdo} ore. È possibile su richiesta scritta e motivata del dipendente, per uno studente sotto i 26 anni, o se un accordo di settore fissa una durata inferiore.',
    '{hebdo} horas semanales, por debajo de la duración mínima de {min_hebdo} horas. Es posible a petición escrita y motivada del trabajador, para un estudiante menor de 26 años, o si un convenio sectorial fija una duración inferior.',
  ],
  'al.ccn.titre': ['Minimum conventionnel (IDCC 0016)', 'Collective-agreement minimum (IDCC 0016)', 'Tarifliches Minimum (IDCC 0016)', 'Sectoraal minimum (IDCC 0016)', 'Minimo contrattuale (IDCC 0016)', 'Mínimo de convenio (IDCC 0016)'],
  'al.ccn.texte': [
    "Le salaire de base doit atteindre le taux de votre coefficient, majoré de l'ancienneté, et jamais moins que le SMIC. Choisissez le classement dans Paramètres pour un contrôle automatique.",
    'Base salary must reach the rate for your grade, increased for seniority, and never less than the minimum wage. Choose the grade in Settings for an automatic check.',
    'Das Grundgehalt muss den Satz Ihrer Einstufung samt Betriebszugehörigkeitszuschlag erreichen und nie unter dem Mindestlohn liegen. Wählen Sie die Einstufung in den Einstellungen für eine automatische Prüfung.',
    'Het basisloon moet het tarief van uw coëfficiënt halen, verhoogd met de anciënniteit, en nooit lager dan het minimumloon. Kies de indeling in Instellingen voor een automatische toetsing.',
    'La retribuzione base deve raggiungere l’importo del vostro livello, maggiorato dell’anzianità, e mai meno del salario minimo. Scegliete l’inquadramento nelle Impostazioni per una verifica automatica.',
    'El salario base debe alcanzar el importe de su coeficiente, incrementado por antigüedad, y nunca menos que el salario mínimo. Elija la clasificación en Ajustes para una comprobación automática.',
  ],
  'al.ccn_bas.titre': ['Salaire de base inférieur au minimum conventionnel', 'Base salary below the collective-agreement minimum', 'Grundgehalt unter dem tariflichen Minimum', 'Basisloon onder het sectorale minimum', 'Retribuzione base inferiore al minimo contrattuale', 'Salario base inferior al mínimo de convenio'],
  'al.ccn_bas.texte': [
    "Coefficient {coef} : minimum conventionnel de {taux_h} € de l'heure, soit {minimum} € bruts par mois pour ce temps de travail (palier d'ancienneté : {palier} an(s)). Le salaire de base ({base} €) est inférieur de {ecart} €.",
    'Grade {coef}: agreement minimum of €{taux_h} per hour, i.e. €{minimum} gross a month for this working time (seniority step: {palier} year(s)). Base salary (€{base}) falls short by €{ecart}.',
    'Koeffizient {coef}: tarifliches Minimum von {taux_h} € pro Stunde, also {minimum} € brutto im Monat für diese Arbeitszeit (Stufe der Betriebszugehörigkeit: {palier} Jahr(e)). Das Grundgehalt ({base} €) liegt {ecart} € darunter.',
    'Coëfficiënt {coef}: sectoraal minimum van {taux_h} € per uur, of {minimum} € bruto per maand voor deze arbeidsduur (anciënniteitstrap: {palier} jaar). Het basisloon ({base} €) ligt {ecart} € lager.',
    'Coefficiente {coef}: minimo contrattuale di {taux_h} € l’ora, cioè {minimum} € lordi al mese per questo orario (scatto di anzianità: {palier} anno/i). La retribuzione base ({base} €) è inferiore di {ecart} €.',
    'Coeficiente {coef}: mínimo de convenio de {taux_h} € por hora, es decir {minimum} € brutos al mes para esta jornada (tramo de antigüedad: {palier} año(s)). El salario base ({base} €) queda {ecart} € por debajo.',
  ],
  'al.ccn_ok.titre': ['Minimum conventionnel respecté', 'Collective-agreement minimum met', 'Tarifliches Minimum eingehalten', 'Sectoraal minimum gehaald', 'Minimo contrattuale rispettato', 'Mínimo de convenio respetado'],
  'al.ccn_ok.texte': [
    "Coefficient {coef} : minimum conventionnel de {taux_h} € de l'heure, soit {minimum} € bruts par mois pour ce temps de travail (palier d'ancienneté : {palier} an(s)). Le salaire de base ({base} €) le respecte. La garantie annuelle de rémunération, elle, se vérifie sur l'année civile.",
    'Grade {coef}: agreement minimum of €{taux_h} per hour, i.e. €{minimum} gross a month for this working time (seniority step: {palier} year(s)). Base salary (€{base}) meets it. The guaranteed annual pay is checked over the calendar year.',
    'Koeffizient {coef}: tarifliches Minimum von {taux_h} € pro Stunde, also {minimum} € brutto im Monat für diese Arbeitszeit (Stufe der Betriebszugehörigkeit: {palier} Jahr(e)). Das Grundgehalt ({base} €) hält es ein. Die garantierte Jahresvergütung wird über das Kalenderjahr geprüft.',
    'Coëfficiënt {coef}: sectoraal minimum van {taux_h} € per uur, of {minimum} € bruto per maand voor deze arbeidsduur (anciënniteitstrap: {palier} jaar). Het basisloon ({base} €) haalt het. De gegarandeerde jaarbeloning wordt over het kalenderjaar getoetst.',
    'Coefficiente {coef}: minimo contrattuale di {taux_h} € l’ora, cioè {minimum} € lordi al mese per questo orario (scatto di anzianità: {palier} anno/i). La retribuzione base ({base} €) lo rispetta. La garanzia annua di retribuzione si verifica sull’anno solare.',
    'Coeficiente {coef}: mínimo de convenio de {taux_h} € por hora, es decir {minimum} € brutos al mes para esta jornada (tramo de antigüedad: {palier} año(s)). El salario base ({base} €) lo respeta. La garantía anual de remuneración se comprueba sobre el año natural.',
  ],
  'al.ccn_smic.titre': ['Minimum conventionnel rattrapé par le SMIC', 'Agreement minimum overtaken by the minimum wage', 'Tarifliches Minimum vom Mindestlohn überholt', 'Sectoraal minimum ingehaald door het minimumloon', 'Minimo contrattuale superato dal salario minimo', 'Mínimo de convenio superado por el salario mínimo'],
  'al.ccn_smic.texte': [
    "Coefficient {coef} : le minimum conventionnel ({minimum} € pour ce temps de travail) est inférieur au SMIC ({smic} €). C'est le SMIC qui s'applique : la grille de la branche n'a pas suivi la revalorisation du salaire minimum.",
    'Grade {coef}: the agreement minimum (€{minimum} for this working time) is below the minimum wage (€{smic}). The minimum wage applies: the sector scale has not kept up with its increases.',
    'Koeffizient {coef}: das tarifliche Minimum ({minimum} € für diese Arbeitszeit) liegt unter dem Mindestlohn ({smic} €). Es gilt der Mindestlohn: die Branchentabelle ist seinen Erhöhungen nicht gefolgt.',
    'Coëfficiënt {coef}: het sectorale minimum ({minimum} € voor deze arbeidsduur) ligt onder het minimumloon ({smic} €). Het minimumloon geldt: de sectorschaal heeft de verhogingen niet gevolgd.',
    'Coefficiente {coef}: il minimo contrattuale ({minimum} € per questo orario) è inferiore al salario minimo ({smic} €). Si applica il salario minimo: la tabella di settore non ne ha seguito gli aumenti.',
    'Coeficiente {coef}: el mínimo de convenio ({minimum} € para esta jornada) es inferior al salario mínimo ({smic} €). Se aplica el salario mínimo: la tabla del sector no ha seguido sus subidas.',
  ],
  'al.ccn_inconnu.titre': ['Minimum conventionnel non contrôlé', 'Agreement minimum not checked', 'Tarifliches Minimum nicht geprüft', 'Sectoraal minimum niet getoetst', 'Minimo contrattuale non verificato', 'Mínimo de convenio no comprobado'],
  'al.ccn_inconnu.texte': [
    "Aucune grille en base pour le coefficient {coef} à la date de paie (les grilles recopiées ne remontent qu'à leur dernier avenant) : pas de contrôle.",
    'No scale on record for grade {coef} at the pay date (the copied scales only go back to their latest amendment): no check.',
    'Keine Tabelle für Koeffizient {coef} zum Abrechnungsdatum (die übernommenen Tabellen reichen nur bis zu ihrer letzten Änderung zurück): keine Prüfung.',
    'Geen schaal voor coëfficiënt {coef} op de loondatum (de overgenomen schalen gaan maar terug tot hun laatste wijziging): geen toetsing.',
    'Nessuna tabella per il coefficiente {coef} alla data di paga (le tabelle riprese risalgono solo all’ultimo accordo): nessuna verifica.',
    'Ninguna tabla para el coeficiente {coef} en la fecha de pago (las tablas copiadas solo remontan a su última modificación): sin comprobación.',
  ],
  'al.ccn.lien': ['Voir la grille dans le Chakrram ›', 'See the scale in the Chakrram ›', 'Tabelle im Chakrram ansehen ›', 'Bekijk de schaal in het Chakrram ›', 'Vedi la tabella nel Chakrram ›', 'Ver la tabla en el Chakrram ›'],
};

/**
 * Phrase `cle` dans la langue `lang`, paramètres {nom} substitués. Repli sur
 * le français si la langue ou la clé manque ; un paramètre absent reste visible
 * tel quel ({nom}) plutôt que de disparaître en silence.
 */
export function tx(cle, lang, vars = {}) {
  const row = TEXTES[cle];
  if (!row) return cle;
  const gabarit = row[IDX[lang] ?? 0] ?? row[0];
  return gabarit.replace(/\{(\w+)\}/g, (m, k) => (k in vars ? String(vars[k]) : m));
}

// ── « En clair » : ce que finance chaque ligne du bulletin, pour qui la lit
// sans être du métier. Une phrase, sans chiffre (un chiffre se périme ; le
// détail technique et la référence suivent). Clé = code de cotisation du moteur.
export const EN_CLAIR = {
  "titre": [
    "En clair",
    "In plain words",
    "Einfach gesagt",
    "Eenvoudig gezegd",
    "In parole semplici",
    "En pocas palabras"
  ],
  "SS_MALADIE": [
    "Finance l'Assurance maladie : remboursement de vos soins, indemnités journalières en arrêt maladie ou maternité, pension d'invalidité, capital décès. Aujourd'hui payée par l'employeur seul.",
    "Funds health insurance: reimbursement of your care, daily allowances during sick or maternity leave, disability pension, death benefit. Now paid by the employer only.",
    "Finanziert die Krankenversicherung: Erstattung Ihrer Behandlungen, Krankengeld bei Krankheit oder Mutterschaft, Invalidenrente, Sterbegeld. Heute allein vom Arbeitgeber getragen.",
    "Financiert de ziekteverzekering: terugbetaling van uw zorg, uitkeringen bij ziekte of moederschap, invaliditeitspensioen, overlijdenskapitaal. Vandaag enkel ten laste van de werkgever.",
    "Finanzia l’assicurazione malattia: rimborso delle cure, indennità in caso di malattia o maternità, pensione di invalidità, capitale in caso di decesso. Oggi a carico del solo datore.",
    "Financia el seguro de enfermedad: reembolso de su atención médica, prestaciones por enfermedad o maternidad, pensión de invalidez, capital por fallecimiento. Hoy a cargo exclusivo de la empresa."
  ],
  "ALSACE_MOSELLE_MALADIE": [
    "Cotisation du régime local d'Alsace-Moselle, payée par le salarié : elle améliore vos remboursements de soins au-delà du régime général.",
    "Contribution to the local Alsace-Moselle scheme, paid by the employee: it improves your healthcare reimbursements beyond the general scheme.",
    "Beitrag zum lokalen System Elsass-Mosel, vom Arbeitnehmer getragen: Er verbessert Ihre Erstattungen über das allgemeine System hinaus.",
    "Bijdrage aan het lokale stelsel Elzas-Moezel, betaald door de werknemer: ze verbetert uw terugbetalingen bovenop het algemeen stelsel.",
    "Contributo al regime locale di Alsazia-Mosella, a carico del dipendente: migliora i rimborsi delle cure oltre il regime generale.",
    "Cotización al régimen local de Alsacia-Mosela, a cargo del trabajador: mejora los reembolsos de la atención médica más allá del régimen general."
  ],
  "SS_VIEILLESSE_PLAF": [
    "Votre retraite de base de la Sécurité sociale. Elle porte sur le salaire jusqu’au plafond : c’est ce salaire qui entre dans votre compte retraite et servira au calcul de votre pension.",
    "Your social security basic pension. It applies to pay up to the ceiling: that pay is credited to your pension account and will be used to calculate your pension.",
    "Ihre Grundrente der Sozialversicherung. Sie gilt für den Lohn bis zur Bemessungsgrenze: Dieser Lohn wird Ihrem Rentenkonto gutgeschrieben und dient zur Berechnung Ihrer Rente.",
    "Uw basispensioen van de sociale zekerheid. Het geldt voor het loon tot het plafond: dat loon komt op uw pensioenrekening en dient voor de berekening van uw pensioen.",
    "La vostra pensione di base della sicurezza sociale. Riguarda la retribuzione fino al massimale: è questa retribuzione che entra nel conto pensione e servirà al calcolo della pensione.",
    "Su jubilación básica de la seguridad social. Se aplica al salario hasta el tope: ese salario se anota en su cuenta de jubilación y servirá para calcular su pensión."
  ],
  "SS_VIEILLESSE_DEPLAF": [
    "Complément de cotisation à la retraite de base, sur tout le salaire. Elle finance le régime mais, au-delà du plafond, n’augmente pas votre pension.",
    "Additional basic pension contribution on the whole salary. It funds the scheme but, above the ceiling, does not increase your pension.",
    "Zusätzlicher Beitrag zur Grundrente auf den gesamten Lohn. Er finanziert das System, erhöht aber oberhalb der Grenze Ihre Rente nicht.",
    "Aanvullende bijdrage voor het basispensioen op het volledige loon. Ze financiert het stelsel maar verhoogt boven het plafond uw pensioen niet.",
    "Contributo aggiuntivo alla pensione di base su tutta la retribuzione. Finanzia il regime ma, oltre il massimale, non aumenta la pensione.",
    "Cotización adicional a la jubilación básica sobre todo el salario. Financia el régimen pero, por encima del tope, no aumenta su pensión."
  ],
  "FAMILLE": [
    "Finance les allocations familiales et les autres prestations de la CAF. Payée par l’employeur seul ; elle ne dépend pas de votre situation familiale.",
    "Funds family allowances and other family benefits (CAF). Paid by the employer only; it does not depend on your family situation.",
    "Finanziert das Kindergeld und andere Familienleistungen (CAF). Allein vom Arbeitgeber getragen; sie hängt nicht von Ihrer Familiensituation ab.",
    "Financiert de kinderbijslag en andere gezinsuitkeringen (CAF). Enkel ten laste van de werkgever; ze hangt niet af van uw gezinssituatie.",
    "Finanzia gli assegni familiari e le altre prestazioni della CAF. A carico del solo datore; non dipende dalla vostra situazione familiare.",
    "Financia las asignaciones familiares y demás prestaciones de la CAF. A cargo exclusivo de la empresa; no depende de su situación familiar."
  ],
  "AT_MP": [
    "Couvre les accidents du travail, de trajet et les maladies professionnelles : soins pris en charge, indemnités, rente en cas de séquelles. Payée par l’employeur seul, selon les risques de son activité.",
    "Covers work and commuting accidents and occupational diseases: care paid for, allowances, annuity for lasting effects. Paid by the employer only, according to the risks of its activity.",
    "Deckt Arbeits- und Wegeunfälle sowie Berufskrankheiten: übernommene Behandlung, Leistungen, Rente bei Folgeschäden. Allein vom Arbeitgeber getragen, je nach Risiko seiner Tätigkeit.",
    "Dekt arbeids- en wegongevallen en beroepsziekten: zorg ten laste, uitkeringen, rente bij blijvende letsels. Enkel ten laste van de werkgever, volgens de risico’s van zijn activiteit.",
    "Copre infortuni sul lavoro e in itinere e malattie professionali: cure a carico, indennità, rendita in caso di postumi. A carico del solo datore, secondo i rischi dell’attività.",
    "Cubre los accidentes de trabajo y de trayecto y las enfermedades profesionales: atención cubierta, prestaciones, renta en caso de secuelas. A cargo exclusivo de la empresa, según los riesgos de su actividad."
  ],
  "CHOMAGE": [
    "Finance l’assurance chômage : les allocations versées par France Travail si vous perdez votre emploi. Payée par l’employeur seul.",
    "Funds unemployment insurance: the benefits paid by France Travail if you lose your job. Paid by the employer only.",
    "Finanziert die Arbeitslosenversicherung: das von France Travail gezahlte Arbeitslosengeld, falls Sie Ihre Stelle verlieren. Allein vom Arbeitgeber getragen.",
    "Financiert de werkloosheidsverzekering: de uitkeringen van France Travail als u uw baan verliest. Enkel ten laste van de werkgever.",
    "Finanzia l’assicurazione contro la disoccupazione: le indennità versate da France Travail se perdete il lavoro. A carico del solo datore.",
    "Financia el seguro de desempleo: las prestaciones abonadas por France Travail si pierde su empleo. A cargo exclusivo de la empresa."
  ],
  "AGS": [
    "Garantit le paiement de vos salaires si votre employeur fait faillite. Payée par l’employeur seul.",
    "Guarantees payment of your wages if your employer goes bankrupt. Paid by the employer only.",
    "Garantiert die Zahlung Ihrer Löhne bei Insolvenz Ihres Arbeitgebers. Allein vom Arbeitgeber getragen.",
    "Waarborgt de betaling van uw loon als uw werkgever failliet gaat. Enkel ten laste van de werkgever.",
    "Garantisce il pagamento dei salari se il datore fallisce. A carico del solo datore.",
    "Garantiza el pago de sus salarios si su empresa quiebra. A cargo exclusivo de la empresa."
  ],
  "AGIRC_ARRCO_T1": [
    "Votre retraite complémentaire, sur la partie du salaire jusqu’au plafond. Partagée entre vous et l’employeur, elle vous achète des points de retraite.",
    "Your supplementary pension, on the part of pay up to the ceiling. Shared between you and the employer, it buys you pension points.",
    "Ihre Zusatzrente auf den Lohnanteil bis zur Grenze. Zwischen Ihnen und dem Arbeitgeber geteilt, kauft sie Ihnen Rentenpunkte.",
    "Uw aanvullend pensioen, op het deel van het loon tot het plafond. Gedeeld tussen u en de werkgever, koopt het u pensioenpunten.",
    "La vostra pensione complementare, sulla parte di retribuzione fino al massimale. Ripartita tra voi e il datore, vi acquista punti di pensione.",
    "Su pensión complementaria, sobre la parte del salario hasta el tope. Repartida entre usted y la empresa, le compra puntos de jubilación."
  ],
  "AGIRC_ARRCO_T2": [
    "Retraite complémentaire sur la partie du salaire au-dessus du plafond, à un taux plus élevé. Elle vous achète aussi des points de retraite.",
    "Supplementary pension on the part of pay above the ceiling, at a higher rate. It also buys you pension points.",
    "Zusatzrente auf den Lohnanteil oberhalb der Grenze, zu einem höheren Satz. Auch sie kauft Ihnen Rentenpunkte.",
    "Aanvullend pensioen op het deel van het loon boven het plafond, tegen een hoger tarief. Ook dit koopt u pensioenpunten.",
    "Pensione complementare sulla parte di retribuzione oltre il massimale, con un’aliquota più alta. Anche questa vi acquista punti di pensione.",
    "Pensión complementaria sobre la parte del salario por encima del tope, a un tipo más alto. También le compra puntos de jubilación."
  ],
  "AGIRC_ARRCO_CEG_T1": [
    "Contribution d’équilibre général : elle finance l’équilibre de la retraite complémentaire Agirc-Arrco, mais ne vous donne aucun point.",
    "General balancing contribution: it funds the balance of the Agirc-Arrco supplementary pension but gives you no points.",
    "Allgemeiner Ausgleichsbeitrag: Er finanziert das Gleichgewicht der Zusatzrente Agirc-Arrco, bringt Ihnen aber keine Punkte.",
    "Algemene evenwichtsbijdrage: ze financiert het evenwicht van het aanvullend pensioen Agirc-Arrco, maar levert u geen punten op.",
    "Contributo di equilibrio generale: finanzia l’equilibrio della pensione complementare Agirc-Arrco, ma non vi dà alcun punto.",
    "Contribución de equilibrio general: financia el equilibrio de la pensión complementaria Agirc-Arrco, pero no le da ningún punto."
  ],
  "PREVOYANCE_CADRE_MIN": [
    "Prévoyance obligatoire des cadres, payée par l’employeur : elle finance notamment un capital versé à vos proches en cas de décès.",
    "Mandatory provident cover for executives, paid by the employer: it notably funds a lump sum paid to your relatives in the event of death.",
    "Pflichtvorsorge für Führungskräfte, vom Arbeitgeber getragen: Sie finanziert insbesondere ein Kapital für Ihre Angehörigen im Todesfall.",
    "Verplichte voorzorg voor kaderleden, betaald door de werkgever: ze financiert onder meer een kapitaal voor uw naasten bij overlijden.",
    "Previdenza obbligatoria dei quadri, a carico del datore: finanzia in particolare un capitale versato ai vostri familiari in caso di decesso.",
    "Previsión obligatoria de los directivos, a cargo de la empresa: financia en particular un capital para sus allegados en caso de fallecimiento."
  ],
  "CSG_DEDUCTIBLE": [
    "Contribution sociale généralisée : un impôt qui finance la Sécurité sociale (santé, famille, autonomie), sans ouvrir de droits. Cette part diminue votre revenu imposable.",
    "General social contribution: a tax funding social security (health, family, long-term care), without creating entitlements. This part reduces your taxable income.",
    "Allgemeiner Sozialbeitrag: eine Abgabe zur Finanzierung der Sozialversicherung (Gesundheit, Familie, Pflege), ohne Ansprüche zu begründen. Dieser Teil senkt Ihr steuerpflichtiges Einkommen.",
    "Algemene sociale bijdrage: een heffing die de sociale zekerheid financiert (gezondheid, gezin, zelfredzaamheid), zonder rechten te openen. Dit deel verlaagt uw belastbaar inkomen.",
    "Contributo sociale generalizzato: un’imposta che finanzia la sicurezza sociale (salute, famiglia, autonomia), senza generare diritti. Questa parte riduce il reddito imponibile.",
    "Contribución social generalizada: un impuesto que financia la seguridad social (salud, familia, autonomía), sin generar derechos. Esta parte reduce su renta imponible."
  ],
  "CSG_NON_DEDUCTIBLE": [
    "Autre part de la CSG : retenue sur votre salaire mais imposable. Vous payez donc l’impôt sur une somme que vous ne touchez pas.",
    "The other part of CSG: deducted from your pay but taxable. You therefore pay tax on money you do not receive.",
    "Der andere Teil der CSG: vom Lohn einbehalten, aber steuerpflichtig. Sie zahlen also Steuer auf Geld, das Sie nicht erhalten.",
    "Het andere deel van de CSG: ingehouden op uw loon maar belastbaar. U betaalt dus belasting op geld dat u niet ontvangt.",
    "L’altra parte della CSG: trattenuta sul salario ma imponibile. Pagate quindi l’imposta su una somma che non incassate.",
    "La otra parte de la CSG: retenida de su salario pero imponible. Paga por tanto impuestos sobre un dinero que no cobra."
  ],
  "CRDS": [
    "Contribution au remboursement de la dette sociale : elle rembourse les déficits passés de la Sécurité sociale. Retenue sur votre salaire, elle reste imposable.",
    "Contribution to repaying the social debt: it repays social security’s past deficits. Deducted from your pay, it remains taxable.",
    "Beitrag zur Tilgung der Sozialschuld: Er tilgt frühere Defizite der Sozialversicherung. Vom Lohn einbehalten, bleibt er steuerpflichtig.",
    "Bijdrage tot aflossing van de sociale schuld: ze lost de vroegere tekorten van de sociale zekerheid af. Ingehouden op uw loon, blijft ze belastbaar.",
    "Contributo al rimborso del debito sociale: rimborsa i deficit passati della sicurezza sociale. Trattenuto sul salario, resta imponibile.",
    "Contribución al reembolso de la deuda social: reembolsa los déficits pasados de la seguridad social. Retenida de su salario, sigue siendo imponible."
  ],
  "REDUCTION_FILLON": [
    "Réduction de cotisations accordée à l’employeur sur les salaires modestes : plus le salaire est proche du SMIC, plus elle est forte. Elle ne change ni votre net ni vos droits.",
    "Contribution reduction granted to the employer on modest wages: the closer pay is to the minimum wage, the larger it is. It changes neither your net pay nor your entitlements.",
    "Beitragsermäßigung für den Arbeitgeber bei niedrigen Löhnen: Je näher der Lohn am Mindestlohn liegt, desto höher fällt sie aus. Sie ändert weder Ihr Netto noch Ihre Ansprüche.",
    "Bijdrageverlaging voor de werkgever op lage lonen: hoe dichter het loon bij het minimumloon ligt, hoe groter ze is. Ze verandert noch uw netto noch uw rechten.",
    "Riduzione dei contributi concessa al datore sui salari modesti: più la retribuzione è vicina al salario minimo, più è forte. Non cambia né il netto né i diritti.",
    "Reducción de cotizaciones concedida a la empresa sobre los salarios modestos: cuanto más cerca del salario mínimo, mayor es. No cambia ni su neto ni sus derechos."
  ],
  "REDUC_SAL_HS": [
    "Vos heures supplémentaires sont allégées d’une partie de vos cotisations retraite : cette réduction augmente votre net.",
    "Your overtime carries lower pension contributions: this reduction increases your net pay.",
    "Auf Ihre Überstunden fallen geringere Rentenbeiträge an: Diese Ermäßigung erhöht Ihr Netto.",
    "Op uw overuren worden minder pensioenbijdragen ingehouden: deze vermindering verhoogt uw netto.",
    "Sugli straordinari si pagano meno contributi pensionistici: questa riduzione aumenta il netto.",
    "Sus horas extra soportan menos cotizaciones de jubilación: esta reducción aumenta su neto."
  ],
  "DFP_HS": [
    "Déduction accordée à l’employeur pour chaque heure supplémentaire, dans les entreprises de moins de 250 salariés. Elle ne change pas votre net.",
    "Deduction granted to the employer for each overtime hour, in companies with fewer than 250 employees. It does not change your net pay.",
    "Abzug für den Arbeitgeber je Überstunde in Unternehmen mit weniger als 250 Beschäftigten. Er ändert Ihr Netto nicht.",
    "Aftrek voor de werkgever per overuur, in ondernemingen met minder dan 250 werknemers. Hij verandert uw netto niet.",
    "Deduzione concessa al datore per ogni ora di straordinario, nelle imprese con meno di 250 dipendenti. Non cambia il netto.",
    "Deducción concedida a la empresa por cada hora extra, en las empresas de menos de 250 trabajadores. No cambia su neto."
  ],
  "AIDE_POSTE_EA": [
    "Aide de l’État versée à l’entreprise adaptée pour chaque poste occupé par un travailleur handicapé. Elle réduit le coût pour l’employeur, pas votre salaire.",
    "State aid paid to the adapted company for each job held by a disabled worker. It lowers the employer’s cost, not your pay.",
    "Staatliche Hilfe an das Inklusionsunternehmen für jeden mit einer behinderten Person besetzten Arbeitsplatz. Sie senkt die Kosten des Arbeitgebers, nicht Ihren Lohn.",
    "Staatssteun aan het aangepast bedrijf voor elke betrekking ingevuld door een werknemer met een handicap. Ze verlaagt de kost voor de werkgever, niet uw loon.",
    "Aiuto dello Stato versato all’impresa adattata per ogni posto occupato da un lavoratore disabile. Riduce il costo per il datore, non il salario.",
    "Ayuda del Estado abonada a la empresa adaptada por cada puesto ocupado por un trabajador con discapacidad. Reduce el coste para la empresa, no su salario."
  ],
  "ESAT_AIDE_POSTE": [
    "Aide de l’État qui finance une partie de la rémunération garantie du travailleur d’ESAT.",
    "State aid funding part of the guaranteed pay of the ESAT worker.",
    "Staatliche Hilfe, die einen Teil der garantierten Vergütung des ESAT-Beschäftigten finanziert.",
    "Staatssteun die een deel van de gewaarborgde vergoeding van de ESAT-werknemer financiert.",
    "Aiuto dello Stato che finanzia una parte della retribuzione garantita del lavoratore di ESAT.",
    "Ayuda del Estado que financia una parte de la remuneración garantizada del trabajador del ESAT."
  ],
  "ESAT_COMPENSATION": [
    "L’État rembourse à l’ESAT les cotisations dues sur la part de rémunération qu’il finance.",
    "The State reimburses the ESAT for the contributions due on the share of pay it funds.",
    "Der Staat erstattet dem ESAT die Beiträge auf den von ihm finanzierten Vergütungsanteil.",
    "De Staat betaalt het ESAT de bijdragen terug op het deel van de vergoeding dat hij financiert.",
    "Lo Stato rimborsa all’ESAT i contributi dovuti sulla parte di retribuzione che finanzia.",
    "El Estado reembolsa al ESAT las cotizaciones debidas sobre la parte de la remuneración que financia."
  ],
  "FPT_CNRACL": [
    "Votre retraite de fonctionnaire territorial ou hospitalier (CNRACL), calculée sur le traitement indiciaire. Partagée entre vous et la collectivité.",
    "Your pension as a local or hospital civil servant (CNRACL), based on index-linked pay. Shared between you and the public employer.",
    "Ihre Rente als kommunaler oder Krankenhausbeamter (CNRACL), berechnet auf das Grundgehalt nach Index. Zwischen Ihnen und dem Dienstherrn geteilt.",
    "Uw pensioen als lokaal of ziekenhuisambtenaar (CNRACL), berekend op de geïndexeerde wedde. Gedeeld tussen u en de overheidswerkgever.",
    "La vostra pensione di funzionario territoriale od ospedaliero (CNRACL), calcolata sul trattamento indiciario. Ripartita tra voi e l’ente.",
    "Su pensión de funcionario territorial u hospitalario (CNRACL), calculada sobre el sueldo indiciario. Repartida entre usted y la administración."
  ],
  "FPT_MALADIE": [
    "Assurance maladie, maternité, invalidité et décès des fonctionnaires, payée par la collectivité.",
    "Health, maternity, disability and death insurance for civil servants, paid by the public employer.",
    "Kranken-, Mutterschafts-, Invaliditäts- und Todesfallversicherung der Beamten, vom Dienstherrn getragen.",
    "Verzekering ziekte, moederschap, invaliditeit en overlijden van ambtenaren, betaald door de overheidswerkgever.",
    "Assicurazione malattia, maternità, invalidità e decesso dei funzionari, a carico dell’ente.",
    "Seguro de enfermedad, maternidad, invalidez y fallecimiento de los funcionarios, a cargo de la administración."
  ],
  "FPT_ATIACL": [
    "Finance l’allocation versée aux agents restés en fonction après un accident de service ou une maladie professionnelle. Payée par la collectivité.",
    "Funds the allowance paid to staff who remain in post after a work accident or occupational disease. Paid by the public employer.",
    "Finanziert die Zulage für Bedienstete, die nach einem Dienstunfall oder einer Berufskrankheit im Dienst bleiben. Vom Dienstherrn getragen.",
    "Financiert de toelage voor personeelsleden die na een arbeidsongeval of beroepsziekte in dienst blijven. Betaald door de overheidswerkgever.",
    "Finanzia l’indennità versata agli agenti rimasti in servizio dopo un infortunio di servizio o una malattia professionale. A carico dell’ente.",
    "Financia la prestación abonada a los agentes que siguen en servicio tras un accidente de servicio o una enfermedad profesional. A cargo de la administración."
  ],
  "FPT_CNFPT": [
    "Finance la formation des agents territoriaux par le Centre national de la fonction publique territoriale. Payée par la collectivité.",
    "Funds training for local government staff by the national local civil service centre (CNFPT). Paid by the public employer.",
    "Finanziert die Fortbildung der Kommunalbediensteten durch das nationale Zentrum (CNFPT). Vom Dienstherrn getragen.",
    "Financiert de opleiding van het lokale overheidspersoneel door het nationaal centrum (CNFPT). Betaald door de overheidswerkgever.",
    "Finanzia la formazione degli agenti territoriali da parte del centro nazionale (CNFPT). A carico dell’ente.",
    "Financia la formación de los agentes territoriales por el centro nacional (CNFPT). A cargo de la administración."
  ],
  "FPT_CSA": [
    "Contribution solidarité autonomie : finance l’accompagnement des personnes âgées et handicapées. Payée par l’employeur.",
    "Solidarity contribution for autonomy: funds support for elderly and disabled people. Paid by the employer.",
    "Solidaritätsbeitrag für Selbstständigkeit: finanziert die Unterstützung älterer und behinderter Menschen. Vom Arbeitgeber getragen.",
    "Solidariteitsbijdrage voor zelfredzaamheid: financiert de ondersteuning van ouderen en personen met een handicap. Betaald door de werkgever.",
    "Contributo di solidarietà per l’autonomia: finanzia l’assistenza a anziani e disabili. A carico del datore.",
    "Contribución de solidaridad para la autonomía: financia la atención a personas mayores y con discapacidad. A cargo de la empresa."
  ],
  "FPT_FNAL": [
    "Fonds national d’aide au logement : finance les aides au logement. Payée par l’employeur.",
    "National housing aid fund: funds housing benefits. Paid by the employer.",
    "Nationaler Wohngeldfonds: finanziert das Wohngeld. Vom Arbeitgeber getragen.",
    "Nationaal fonds voor huisvestingssteun: financiert de huurtoelagen. Betaald door de werkgever.",
    "Fondo nazionale per l’aiuto all’alloggio: finanzia gli aiuti per la casa. A carico del datore.",
    "Fondo nacional de ayuda a la vivienda: financia las ayudas a la vivienda. A cargo de la empresa."
  ],
};

/** Phrase « en clair » d'un code, ou '' si le code n'en a pas. */
export function enClair(code, lang) {
  const row = EN_CLAIR[code];
  return row ? (row[IDX[lang] ?? 0] ?? row[0]) : '';
}
