use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, Salarie};
use super::de_cotisations::*;
use super::de_lohnsteuer::{lohnsteuer_mensuel, Vorsorge};

pub fn generer_bulletin_de(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut        = salarie.salaire_brut;
    let steuerklasse = salarie.steuerklasse.unwrap_or(1);
    let kinderlos   = salarie.kinderlos.unwrap_or(false);
    let kirchenmitglied = salarie.kirchenmitglied.unwrap_or(false);
    let land        = salarie.land.as_deref().unwrap_or("NW");

    let mut cotisations = vec![
        de_krankenversicherung(brut, ctx),
        de_rentenversicherung(brut, ctx),
        de_arbeitslosenversicherung(brut, ctx),
        de_pflegeversicherung(brut, ctx),
    ];

    if kinderlos {
        cotisations.push(de_pv_kinderlos(brut, ctx));
    }

    cotisations.push(de_unfallversicherung(brut, ctx));

    // Cotisations salariales retenues dans la Vorsorgepauschale de la Lohnsteuer.
    let part = |codes: &[&str]| -> Decimal {
        cotisations.iter().filter(|c| codes.contains(&c.code.as_str())).map(|c| c.montant_sal).sum()
    };
    let vorsorge = Vorsorge {
        rv: part(&["DE_RENTENVERSICHERUNG"]),
        // Le PAP retient la maladie au taux RÉDUIT (14,0 % au lieu de 14,6 %, sans
        // indemnités journalières) : part salariale 7,0 % + moitié du Zusatzbeitrag,
        // soit 0,3 point de moins que la cotisation réelle.
        kv: {
            let kv = part(&["DE_KRANKENVERSICHERUNG"]);
            let taux = ctx.taux_sal("DE_KRANKENVERSICHERUNG");
            if taux > Decimal::ZERO { kv * (taux - dec!(0.003)) / taux } else { kv }
        },
        pv: part(&["DE_PFLEGEVERSICHERUNG", "DE_PV_KINDERLOS"]),
        av: part(&["DE_ARBEITSLOSENVERSICHERUNG"]),
    };
    cotisations.extend(lohnsteuer_mensuel(brut, vorsorge, steuerklasse, kirchenmitglied, land, ctx));

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();

    // Net imposable = brut − cotisations sociales salariales uniquement
    // (Lohnsteuer, Soli et Kirchensteuer sont déjà dans total_sal mais ne
    //  réduisent pas la base imposable — ils sont prélevés sur le net social)
    let cots_sociales_sal: Decimal = cotisations.iter()
        .filter(|c| matches!(
            c.code.as_str(),
            "DE_KRANKENVERSICHERUNG" | "DE_RENTENVERSICHERUNG"
            | "DE_ARBEITSLOSENVERSICHERUNG" | "DE_PFLEGEVERSICHERUNG"
            | "DE_PV_KINDERLOS"
        ))
        .map(|c| c.montant_sal)
        .sum();

    let net_imposable = (brut - cots_sociales_sal).round_dp(2);
    let net_a_payer   = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations,
        brut,
        net_imposable,
        net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "EUR".into(),
        absence: None,
        heures_sup: None, conges: None, frais_professionnels: Vec::new(), avantages_nature: Vec::new(),
        salarie,
    }
}
