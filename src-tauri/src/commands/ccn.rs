// Commandes Tauri de consultation des conventions collectives.
//
// Le serveur web expose `POST /api/dossier_ccn` ; le bureau expose la
// commande `dossier_ccn`. Même nom, même charge utile, même réponse :
// côté front, `api('dossier_ccn', { idcc })` fonctionne dans les deux
// mondes sans une seule branche conditionnelle.
//
// Contenu éditorial : aucune de ces données n'entre dans un bulletin.

use crate::{
    ccn::routes::{charger_conventions, charger_dossier, idcc_valide},
    ccn::models::{ConventionResume, DossierCcn},
    AppState,
};

#[tauri::command]
pub async fn dossier_ccn(
    state: tauri::State<'_, AppState>,
    idcc: Option<String>,
) -> Result<DossierCcn, String> {
    let idcc = idcc.unwrap_or_else(|| "0016".to_string());
    if !idcc_valide(&idcc) {
        return Err(format!("Code IDCC invalide : '{idcc}'"));
    }

    charger_dossier(&state.db, &idcc)
        .await
        .map_err(|e| format!("Lecture des conventions impossible : {e}"))?
        .ok_or_else(|| format!("Convention IDCC {idcc} introuvable"))
}

#[tauri::command]
pub async fn conventions_ccn(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<ConventionResume>, String> {
    charger_conventions(&state.db)
        .await
        .map_err(|e| format!("Lecture des conventions impossible : {e}"))
}
