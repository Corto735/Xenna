// Traductions curées des libellés et explications de cotisations.
//
// Le français est la langue native du code (rédigée en dur dans les
// fonctions de cotisation) : il sert de repli. Les autres langues
// (en, de, nl, it, es — celles du menu 🌐 LANGUE) sont fournies ici,
// indexées par le `code` stable de chaque cotisation.
//
// Les explications DYNAMIQUES (PMSS, dates, formule Fillon) sont stockées
// sous forme de gabarits à placeholders nommés `{pmss}`, `{annee}`, `{coeff}`…
// identiques au gabarit français, et substitués côté appelant via `.replace`.
//
// Périmètre actuel : France. À étendre pays par pays.

pub mod cotisations;
