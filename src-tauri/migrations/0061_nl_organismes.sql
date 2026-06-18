-- 0061 — Pays-Bas : organismes collecteurs et textes de loi
-- Périmètre : salarié secteur privé, sous l'âge AOW, régime général.
-- Données : 2026 (pilote). Extension 2015-2025 à suivre (sourcing officiel par année).
--
-- Modèle néerlandais : le salarié ne supporte que la « loonheffing »
--   = impôt sur le revenu (loonbelasting) + premies volksverzekeringen (AOW/Anw/Wlz),
--     diminuée des heffingskortingen (algemene heffingskorting + arbeidskorting).
-- Les premies werknemersverzekeringen (AWf, Aof, Whk) et la Zvw sont à 100 % patronales
-- (werkgeversheffing) → elles n'entrent pas dans le net, mais dans le coût employeur.

INSERT INTO organisme (code, libelle, url) VALUES
  ('NL_BELASTINGDIENST', 'Belastingdienst — Administration fiscale (loonbelasting, heffingskortingen)', 'https://www.belastingdienst.nl'),
  ('NL_SVB',             'Sociale Verzekeringsbank — volksverzekeringen (AOW, Anw, Wlz)',                 'https://www.svb.nl'),
  ('NL_UWV',             'UWV — werknemersverzekeringen (WW/AWf, WIA/Aof, Whk)',                          'https://www.uwv.nl');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('NL_WET_IB_2001', 'LOI', 'Wet inkomstenbelasting 2001 — Loi sur l''impôt sur le revenu', 'Stb. 2000, 215', '2000-05-11', '2001-01-01',
   'https://wetten.overheid.nl/BWBR0011353',
   'Barème box 1 (revenu du travail et du logement) et heffingskortingen. 2026 (sous âge AOW) : tranche 1 jusqu''à 38 883 € à 35,75 % (dont 27,65 % premies volksverzekeringen), tranche 2 38 883–78 426 € à 37,56 %, tranche 3 au-delà à 49,50 %.'),

  ('NL_WFSV', 'LOI', 'Wet financiering sociale verzekeringen (Wfsv) — Financement des assurances sociales', 'Stb. 2004, 311', '2004-07-16', '2006-01-01',
   'https://wetten.overheid.nl/BWBR0017745',
   'Fixe les premies volksverzekeringen (AOW 17,90 %, Anw 0,10 %, Wlz 9,65 % en 2026, dans la tranche 1) et werknemersverzekeringen (AWf, Aof). Maximumpremieloon 2026 : 79 409 €/an.'),

  ('NL_ZVW', 'LOI', 'Zorgverzekeringswet (Zvw) — Loi sur l''assurance santé', 'Stb. 2005, 358', '2005-06-16', '2006-01-01',
   'https://wetten.overheid.nl/BWBR0018450',
   'Cotisation santé. Pour un salarié : werkgeversheffing Zvw à 100 % patronale, 6,10 % en 2026, sur un revenu de cotisation plafonné à 79 409 €/an.'),

  ('NL_BELASTINGPLAN_2026', 'LOI', 'Belastingplan 2026 — Loi de finances 2026', 'Stb. 2025', '2025-12-01', '2026-01-01',
   'https://www.rijksoverheid.nl/onderwerpen/belastingplan',
   'Tarifs box 1, heffingskortingen et premiepercentages 2026. Arbeidskorting max 5 685 € ; algemene heffingskorting max 3 115 € (dégressivité 6,398 % au-delà de 29 736 €).');
