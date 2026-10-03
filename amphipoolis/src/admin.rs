//! Modération supérieure : ce que le super administrateur de Xenna voit et
//! supprime, en dernier recours, même après publication.
//!
//! Aucune route ici : la crate n'a pas d'authentification d'administrateur.
//! Xenna expose ces fonctions derrière son JWT (admin/routes.rs), comme la
//! suppression des séquences Meliinda.
//!
//! Le super administrateur voit le vrai nom des pseudonymes, même refusés
//! (`a_renommer`), et tous les statuts : en attente, publié, refusé.

use std::collections::HashMap;

use serde::Serialize;
use sqlx::SqlitePool;

#[derive(sqlx::FromRow, Serialize)]
pub struct MessageAdmin {
    pub id:         i64,
    #[serde(skip)]
    pub sujet_id:   i64,
    pub texte:      String,
    pub auteur:     String,
    pub statut:     String,
    pub motif:      Option<String>,
    pub cree_le:    String,
    pub expire_le:  Option<String>,
    pub decide_par: Option<String>,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct SujetAdmin {
    pub id:         i64,
    pub titre:      String,
    pub texte:      String,
    pub auteur:     String,
    pub statut:     String,
    pub motif:      Option<String>,
    pub cree_le:    String,
    pub expire_le:  Option<String>,
    pub decide_par: Option<String>,
    #[sqlx(skip)]
    pub messages:   Vec<MessageAdmin>,
}

/// Tous les sujets, du plus récent au plus ancien, chacun avec ses messages
/// dans l'ordre du fil.
pub async fn tout_lister(pool: &SqlitePool) -> Result<Vec<SujetAdmin>, sqlx::Error> {
    let mut sujets = sqlx::query_as::<_, SujetAdmin>(
        "SELECT s.id, s.titre, s.texte, p.nom AS auteur, s.statut, s.motif,
                s.cree_le, s.expire_le, d.nom AS decide_par
           FROM amph_sujets s
           JOIN amph_pseudos p      ON p.id = s.auteur_id
           LEFT JOIN amph_pseudos d ON d.id = s.decide_par
          ORDER BY s.cree_le DESC, s.id DESC",
    )
    .fetch_all(pool)
    .await?;

    let messages = sqlx::query_as::<_, MessageAdmin>(
        "SELECT x.id, x.sujet_id, x.texte, p.nom AS auteur, x.statut, x.motif,
                x.cree_le, x.expire_le, d.nom AS decide_par
           FROM amph_messages x
           JOIN amph_pseudos p      ON p.id = x.auteur_id
           LEFT JOIN amph_pseudos d ON d.id = x.decide_par
          ORDER BY x.id ASC",
    )
    .fetch_all(pool)
    .await?;

    let index: HashMap<i64, usize> = sujets.iter().enumerate().map(|(i, s)| (s.id, i)).collect();
    for m in messages {
        if let Some(&i) = index.get(&m.sujet_id) {
            sujets[i].messages.push(m);
        }
    }
    Ok(sujets)
}

/// Supprime un sujet (avec toutes ses réponses) ou un message, ainsi que les
/// +1 et le journal de modération qui s'y rapportent. `Ok(false)` : objet
/// inconnu ou déjà disparu.
pub async fn supprimer(pool: &SqlitePool, objet: &str, id: i64) -> Result<bool, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let n = match objet {
        "message" => {
            let n = sqlx::query("DELETE FROM amph_messages WHERE id = ?")
                .bind(id).execute(&mut *tx).await?.rows_affected();
            if n > 0 {
                sqlx::query("DELETE FROM amph_plus1 WHERE objet = 'message' AND objet_id = ?")
                    .bind(id).execute(&mut *tx).await?;
                sqlx::query("DELETE FROM amph_journal WHERE objet = 'message' AND objet_id = ?")
                    .bind(id).execute(&mut *tx).await?;
            }
            n
        }
        "sujet" => {
            // Les réponses d'abord : elles référencent le sujet.
            for sql in [
                "DELETE FROM amph_plus1 WHERE objet = 'message'
                   AND objet_id IN (SELECT id FROM amph_messages WHERE sujet_id = ?)",
                "DELETE FROM amph_journal WHERE objet = 'message'
                   AND objet_id IN (SELECT id FROM amph_messages WHERE sujet_id = ?)",
                "DELETE FROM amph_messages WHERE sujet_id = ?",
                "DELETE FROM amph_plus1   WHERE objet = 'sujet' AND objet_id = ?",
                "DELETE FROM amph_journal WHERE objet = 'sujet' AND objet_id = ?",
            ] {
                sqlx::query(sql).bind(id).execute(&mut *tx).await?;
            }
            sqlx::query("DELETE FROM amph_sujets WHERE id = ?")
                .bind(id).execute(&mut *tx).await?.rows_affected()
        }
        _ => 0,
    };
    if n == 0 {
        tx.rollback().await?;
        return Ok(false);
    }
    tx.commit().await?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn base() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        crate::db::run_migrations(&pool).await.unwrap();
        for sql in [
            "INSERT INTO amph_pseudos (id, nom, phrase_hash, cree_le) VALUES (1, 'Xena', 'h', 't')",
            "INSERT INTO amph_sujets (id, auteur_id, titre, texte, statut, cree_le) VALUES (1, 1, 'Un', 'a', 'publie', '2026-10-01')",
            "INSERT INTO amph_sujets (id, auteur_id, titre, texte, cree_le) VALUES (2, 1, 'Deux', 'b', '2026-10-02')",
            "INSERT INTO amph_messages (id, sujet_id, auteur_id, texte, cree_le) VALUES (10, 1, 1, 'r1', 't')",
            "INSERT INTO amph_messages (id, sujet_id, auteur_id, texte, cree_le) VALUES (11, 1, 1, 'r2', 't')",
            "INSERT INTO amph_plus1 (objet, objet_id, pseudo_id, le) VALUES ('message', 10, 1, 't')",
            "INSERT INTO amph_journal (objet, objet_id, decision, moderateur_id, le) VALUES ('sujet', 1, 'publier', 1, 't')",
        ] {
            sqlx::query(sql).execute(&pool).await.unwrap();
        }
        pool
    }

    async fn compte(pool: &SqlitePool, sql: &str) -> i64 {
        sqlx::query_scalar(sql).fetch_one(pool).await.unwrap()
    }

    #[tokio::test]
    async fn liste_regroupe_les_messages_sous_leur_sujet() {
        let pool = base().await;
        let s = tout_lister(&pool).await.unwrap();
        assert_eq!(s.iter().map(|x| x.id).collect::<Vec<_>>(), vec![2, 1]);
        assert_eq!(s[1].messages.iter().map(|m| m.id).collect::<Vec<_>>(), vec![10, 11]);
        assert!(s[0].messages.is_empty());
    }

    #[tokio::test]
    async fn supprimer_un_message_emporte_ses_plus1() {
        let pool = base().await;
        assert!(supprimer(&pool, "message", 10).await.unwrap());
        assert_eq!(compte(&pool, "SELECT COUNT(*) FROM amph_messages").await, 1);
        assert_eq!(compte(&pool, "SELECT COUNT(*) FROM amph_plus1").await, 0);
        assert!(!supprimer(&pool, "message", 10).await.unwrap());
    }

    #[tokio::test]
    async fn supprimer_un_sujet_emporte_tout_son_fil() {
        let pool = base().await;
        assert!(supprimer(&pool, "sujet", 1).await.unwrap());
        assert_eq!(compte(&pool, "SELECT COUNT(*) FROM amph_sujets").await, 1);
        assert_eq!(compte(&pool, "SELECT COUNT(*) FROM amph_messages").await, 0);
        assert_eq!(compte(&pool, "SELECT COUNT(*) FROM amph_plus1").await, 0);
        assert_eq!(compte(&pool, "SELECT COUNT(*) FROM amph_journal").await, 0);
        assert!(!supprimer(&pool, "pseudo", 1).await.unwrap());
    }
}
