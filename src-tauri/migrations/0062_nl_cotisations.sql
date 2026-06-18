-- 0062 — Pays-Bas : cotisations patronales (werknemersverzekeringen + Zvw) 2026
-- Régime général, secteur privé. Toutes 100 % patronales → n'affectent pas le net,
-- seulement le coût employeur. Assiette plafonnée au maximumpremieloon (79 409 €/an 2026).
-- L'impôt + premies volksverzekeringen (côté salarié) sont calculés en Rust (nl_loonheffing.rs).
--
-- Source : Belastingdienst / UWV — premiepercentages werknemers- en volksverzekeringen 2026.

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('NL_ZVW', 'Zvw — Werkgeversheffing assurance santé',
   (SELECT id FROM organisme WHERE code = 'NL_BELASTINGDIENST'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
   '2026 : 6,10 % patronal (werkgeversheffing). Assiette plafonnée à 79 409 €/an. Zorgverzekeringswet.'),

  ('NL_AWF', 'AWf — Premie chômage (WW, Algemeen Werkloosheidsfonds)',
   (SELECT id FROM organisme WHERE code = 'NL_UWV'), 'CHOMAGE', 1, 1, 'BRUT_PLAFONNÉ',
   '2026 : tarif bas (laag, contrat à durée indéterminée écrit) 2,74 % patronal. Tarif haut (hoog) 7,74 % pour CDD/flex. Assiette plafonnée à 79 409 €/an. Wfsv.'),

  ('NL_AOF', 'Aof — Premie invalidité (WIA, Arbeidsongeschiktheidsfonds)',
   (SELECT id FROM organisme WHERE code = 'NL_UWV'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
   '2026 : tarif bas (laag, petit employeur) 6,27 % patronal. Tarif haut (hoog, grand employeur) 7,63 %. Assiette plafonnée à 79 409 €/an. Wfsv.'),

  ('NL_WHK', 'Whk — Premie différenciée (WGA + Ziektewet)',
   (SELECT id FROM organisme WHERE code = 'NL_UWV'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
   '2026 : valeur moyenne ~1,52 % patronal (WGA ~0,96 % + ZW-flex ~0,56 %). Taux réellement différencié par employeur. Assiette plafonnée à 79 409 €/an.'),

  ('NL_OPSLAG_KO', 'Opslag kinderopvang — Surtaxe accueil de l''enfance',
   (SELECT id FROM organisme WHERE code = 'NL_UWV'), 'AUTRES', 1, 1, 'BRUT_PLAFONNÉ',
   '2026 : 0,50 % patronal, uniforme. Finance la kinderopvangtoeslag. Assiette plafonnée à 79 409 €/an.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'NL_ZVW'),       '2026-01-01', NULL, '0', '0.0610',
   (SELECT id FROM texte_loi WHERE code = 'NL_ZVW'), 'Werkgeversheffing Zvw 2026 : 6,10 %.'),

  ((SELECT id FROM cotisation WHERE code = 'NL_AWF'),       '2026-01-01', NULL, '0', '0.0274',
   (SELECT id FROM texte_loi WHERE code = 'NL_WFSV'), 'AWf-laag 2026 : 2,74 % (CDI écrit).'),

  ((SELECT id FROM cotisation WHERE code = 'NL_AOF'),       '2026-01-01', NULL, '0', '0.0627',
   (SELECT id FROM texte_loi WHERE code = 'NL_WFSV'), 'Aof-laag 2026 : 6,27 % (petit employeur).'),

  ((SELECT id FROM cotisation WHERE code = 'NL_WHK'),       '2026-01-01', NULL, '0', '0.0152',
   (SELECT id FROM texte_loi WHERE code = 'NL_WFSV'), 'Whk 2026 : moyenne ~1,52 % (différenciée par employeur).'),

  ((SELECT id FROM cotisation WHERE code = 'NL_OPSLAG_KO'), '2026-01-01', NULL, '0', '0.0050',
   (SELECT id FROM texte_loi WHERE code = 'NL_WFSV'), 'Opslag kinderopvang 2026 : 0,50 %.');
