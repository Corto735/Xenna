# Polices du contrat de travail

`LiberationSerif-Regular.ttf`, `LiberationSerif-Bold.ttf`, `LiberationSerif-Italic.ttf`

**Liberation Fonts**, publiées par Red Hat puis maintenues par le projet Liberation,
sous **SIL Open Font License 1.1** — https://github.com/liberationfonts/liberation-fonts

Cette licence autorise l'usage, la redistribution et l'incorporation dans un
logiciel, y compris commercial. Elle interdit la vente des fichiers de police
seuls et impose que toute version modifiée change de nom : ces fichiers ne sont
donc pas modifiés.

## Pourquoi elles sont ici plutôt que sur le système

`src-tauri/src/contrat/police.rs` les intègre au binaire par `include_bytes!`. Le
PDF produit doit être identique sur le poste de l'utilisateur (Tauri) et dans le
conteneur de production, où aucune police n'est installée. Les quatorze polices de
base du format PDF ne conviennent pas : elles n'exposent pas de métriques
exploitables pour la découpe des lignes, et leur couverture du français est
incomplète (œ, €, guillemets, apostrophe typographique).

Coût : environ 1,1 Mo de binaire, et autant dans chaque PDF — printpdf ne sait
sous-ensembler les polices que sous la feature `text_layout`, écartée pour les
raisons expliquées dans `src-tauri/Cargo.toml`.
