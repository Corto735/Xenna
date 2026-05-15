use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Une frappe individuelle capturée dans le navigateur.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyEvent {
    pub key:  String,   // valeur de la touche ("a", "Backspace", " ", …)
    pub t:    f64,      // performance.now() en ms depuis le début de la session
    #[serde(rename = "type")]
    pub kind: EventKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventKind {
    Down,
    Up,
}

/// Séquence complète : métadonnées + événements bruts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sequence {
    pub id:         Uuid,
    pub label:      Option<String>,
    pub events:     Vec<KeyEvent>,
    pub created_at: DateTime<Utc>,
}

/// Payload reçu lors de l'enregistrement.
#[derive(Debug, Deserialize)]
pub struct RecordRequest {
    pub label:  Option<String>,
    pub events: Vec<KeyEvent>,
}

/// Réponse minimale après enregistrement.
#[derive(Debug, Serialize)]
pub struct RecordResponse {
    pub id: Uuid,
}
