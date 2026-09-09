# Polices des documents PDF

Romaines — contrat de travail :
`LiberationSerif-Regular.ttf`, `LiberationSerif-Bold.ttf`, `LiberationSerif-Italic.ttf`

Linéales — bulletin de paie :
`LiberationSans-Regular.ttf`, `LiberationSans-Bold.ttf`, `LiberationSans-Italic.ttf`

Deux familles parce que deux documents : le contrat est un texte courant, il se
lit en romaine ; le bulletin est une grille de chiffres, il se lit en linéale.

**Liberation Fonts**, publiées par Red Hat puis maintenues par le projet Liberation,
sous **SIL Open Font License 1.1** — https://github.com/liberationfonts/liberation-fonts

Cette licence autorise l'usage, la redistribution et l'incorporation dans un
logiciel, y compris commercial. Elle interdit la vente des fichiers de police
seuls et impose que toute version modifiée change de nom : ces fichiers ne sont
donc pas modifiés.

## Pourquoi elles sont ici plutôt que sur le système

`src-tauri/src/pdf/police.rs` les intègre au binaire par `include_bytes!`. Le
PDF produit doit être identique sur le poste de l'utilisateur (Tauri) et dans le
conteneur de production, où aucune police n'est installée. Les quatorze polices de
base du format PDF ne conviennent pas : elles n'exposent pas de métriques
exploitables pour la découpe des lignes, et leur couverture du français est
incomplète (œ, €, guillemets, apostrophe typographique).

Coût : environ 2,4 Mo de binaire — printpdf ne sait sous-ensembler les polices que
sous la feature `text_layout`, écartée pour les raisons expliquées dans
`src-tauri/Cargo.toml`. **Chaque PDF n'embarque en revanche que les faces qu'il
trace** : `crate::pdf::rendu::rendre` balaie les tracés avant d'ajouter les
fontes, si bien qu'un bulletin ne traîne pas les romaines du contrat, et
réciproquement — environ 640 Ko par document au lieu de 2,4 Mo.
