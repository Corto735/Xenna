#!/usr/bin/env python3
"""Construit la fonte du mode Minitel à partir du jeu de caractères EF9345.

Point de départ : la fonte `Minitel.ttf` de Frédéric Bisson (Zigazou), tracée
d'après le générateur de caractères EF9345 du Minitel 1B, distribuée en CC0 1.0
dans le dépôt Minitel-Canvas. 126 glyphes : le seul jeu G0 alphanumérique.

Ce qu'elle n'a pas, et que ce script ajoute, sur SA PROPRE GRILLE (cellule de
8 × 10 pixels, 128 unités de fonte par pixel) :

  • les filets et les angles (U+2500…) et les pavés (U+2580…), tracés bord à
    bord pour que deux cellules voisines se raccordent sans fissure — c'est ce
    qui permet de dessiner un tableau entier en caractères ;
  • le signe €, que le Minitel n'a jamais connu (il est de 1982, l'euro de
    1999) mais dont un simulateur de paie ne peut pas se passer ;
  • les capitales accentuées, non pas dessinées mais REDIRIGÉES vers leur
    lettre de base. Ce n'est pas un renoncement : la cellule EF9345 ne laisse
    qu'une rangée de pixels au-dessus des capitales, et le Vidéotex n'a jamais
    su afficher « É ». Les services de l'époque écrivaient REMUNERATION. Le
    texte de la page reste accentué — seul le tracé perd l'accent, et la
    sélection comme le copier-coller rendent bien « RÉMUNÉRATION ».

Sortie : public/minitel.woff2, servi par l'application (aucun CDN : la fonte
d'un mode d'affichage ne doit pas dépendre d'un tiers pour s'afficher).

Usage : python3 scripts/build-minitel-font.py
"""

import os
import sys
import urllib.request

from fontTools.ttLib import TTFont, newTable
from fontTools.ttLib.tables._g_l_y_f import Glyph, GlyphCoordinates
from fontTools.pens.ttGlyphPen import TTGlyphPen

SOURCE = ("https://raw.githubusercontent.com/Zigazou/Minitel-Canvas/"
          "master/Minitel.ttf")
SOURCE_MD5 = "f0a976f7760b16ce024ef08d93a68482"

# ── Grille EF9345 ────────────────────────────────────────────────────────────
PX = 128                 # unités de fonte par pixel
CELL_W = 8 * PX          # 1024 — largeur d'avance, pleine chasse
CELL_BOT = -2 * PX       # -256 — bas de cellule (sous la ligne de base)
CELL_TOP = 8 * PX        # 1024 — haut de cellule
# Axes médians : le filet vertical occupe la colonne 3, l'horizontal la
# rangée qui la croise. Les deux se coupent donc franchement.
MID_X0, MID_X1 = 3 * PX, 4 * PX          # 384 … 512
MID_Y0, MID_Y1 = 3 * PX, 4 * PX          # 384 … 512


def rects(*boxes):
    """Trace une liste de rectangles (x0, y0, x1, y1) en un glyphe TrueType."""
    pen = TTGlyphPen(None)
    for x0, y0, x1, y1 in boxes:
        pen.moveTo((x0, y0))
        pen.lineTo((x1, y0))
        pen.lineTo((x1, y1))
        pen.lineTo((x0, y1))
        pen.closePath()
    return pen.glyph()


# Demi-filets, nommés depuis le centre de la cellule vers un bord.
H_LEFT = (0, MID_Y0, MID_X1, MID_Y1)
H_RIGHT = (MID_X0, MID_Y0, CELL_W, MID_Y1)
H_FULL = (0, MID_Y0, CELL_W, MID_Y1)
V_DOWN = (MID_X0, CELL_BOT, MID_X1, MID_Y1)
V_UP = (MID_X0, MID_Y0, MID_X1, CELL_TOP)
V_FULL = (MID_X0, CELL_BOT, MID_X1, CELL_TOP)

BOX = {
    0x2500: (H_FULL,),                      # ─
    0x2502: (V_FULL,),                      # │
    0x250C: (H_RIGHT, V_DOWN),              # ┌
    0x2510: (H_LEFT, V_DOWN),               # ┐
    0x2514: (H_RIGHT, V_UP),                # └
    0x2518: (H_LEFT, V_UP),                 # ┘
    0x251C: (V_FULL, H_RIGHT),              # ├
    0x2524: (V_FULL, H_LEFT),               # ┤
    0x252C: (H_FULL, V_DOWN),               # ┬
    0x2534: (H_FULL, V_UP),                 # ┴
    0x253C: (H_FULL, V_FULL),               # ┼
}

# La cellule fait 10 rangées : sa vraie moitié tombe à y = 384, pas sur la
# bande médiane des filets (384…512, large d'une rangée).
HALF_Y = CELL_BOT + 5 * PX               # 384

BLOCKS = {
    0x2588: ((0, CELL_BOT, CELL_W, CELL_TOP),),          # █
    0x2580: ((0, HALF_Y, CELL_W, CELL_TOP),),            # ▀ moitié haute
    0x2584: ((0, CELL_BOT, CELL_W, HALF_Y),),            # ▄ moitié basse
    0x258C: ((0, CELL_BOT, MID_X1, CELL_TOP),),          # ▌ moitié gauche
    0x2590: ((MID_X1, CELL_BOT, CELL_W, CELL_TOP),),     # ▐ moitié droite
    0x2581: ((0, CELL_BOT, CELL_W, CELL_BOT + PX),),     # ▁ une rangée basse
    0x2594: ((0, CELL_TOP - PX, CELL_W, CELL_TOP),),     # ▔ une rangée haute
}


def shade(keeps):
    """Trame de pavés : allume les pixels du damier 8 × 10 retenus par `keeps`,
    prédicat appliqué à (colonne, rangée)."""
    boxes = []
    for col in range(8):
        for row in range(10):
            if not keeps(col, row):
                continue
            x0 = col * PX
            y0 = CELL_BOT + row * PX
            boxes.append((x0, y0, x0 + PX, y0 + PX))
    return tuple(boxes)


SHADES = {
    0x2591: shade(lambda c, r: (c % 2 == 0) and (r % 2 == 0)),    # ░ un quart
    0x2592: shade(lambda c, r: (c + r) % 2 == 0),                 # ▒ moitié
    0x2593: shade(lambda c, r: not ((c % 2) and (r % 2))),        # ▓ trois quarts
}


def euro():
    """« € » sur la matrice 5 × 7 : un C barré de deux filets horizontaux."""
    x = lambda n: PX + n * PX          # colonnes 1…6 de la matrice
    y = lambda n: n * PX               # rangées 0…7 au-dessus de la ligne
    return (
        (x(1), y(6), x(5), y(7)),      # arc supérieur
        (x(0), y(5), x(1), y(6)),
        (x(0), y(1), x(1), y(5)),      # dos vertical gauche
        (x(0), y(0), x(1), y(1)),
        (x(1), y(0), x(5), y(1)),      # arc inférieur
        (x(-1), y(4), x(4), y(5)),     # barre haute, débordant à gauche
        (x(-1), y(2), x(4), y(3)),     # barre basse
    )


# ── Signes manquants, dessinés sur la matrice 5 × 7 ──────────────────────────
# La fonte EF9345 s'arrête au jeu G0 : tout ce que l'interface emploie au-delà
# (guillemets français, flèches, coches, pictogrammes de menu) tombait sur une
# fonte de secours, aux proportions étrangères — c'est le même défaut que les
# accents minuscules, en moins visible. On les dessine donc ici, sur la grille
# de la fonte : 5 colonnes, 7 rangées, la rangée 6 posée sur la ligne de base.
#
# Lecture : une chaîne par rangée, du haut vers le bas ; '#' allume le pixel.
PIXELS = {
    0x00AB: ("....."   # «
             "..#.#"
             ".#.#."
             "#.#.."
             ".#.#."
             "..#.#"
             "....."),
    0x00BB: ("....."   # »
             "#.#.."
             ".#.#."
             "..#.#"
             ".#.#."
             "#.#.."
             "....."),
    0x00B7: ("....."   # ·
             "....."
             "....."
             "..#.."
             "....."
             "....."
             "....."),
    0x00D7: ("....."   # ×
             "....."
             "#...#"
             ".#.#."
             "..#.."
             ".#.#."
             "#...#"),
    0x0394: ("....."   # Δ
             "..#.."
             ".#.#."
             ".#.#."
             "#...#"
             "#...#"
             "#####"),
    0x2013: ("....."   # –
             "....."
             "....."
             ".###."
             "....."
             "....."
             "....."),
    0x2014: ("....."   # —
             "....."
             "....."
             "#####"
             "....."
             "....."
             "....."),
    0x2026: ("....."   # …
             "....."
             "....."
             "....."
             "....."
             "....."
             "#.#.#"),
    0x21E2: ("....."   # ⇢
             "....."
             "...#."
             "#.###"
             "...#."
             "....."
             "....."),
    0x21E9: ("..#.."   # ⇩
             "..#.."
             "..#.."
             "..#.."
             "#.#.#"
             ".###."
             "..#.."),
    0x2211: ("....."   # ∑
             "#####"
             ".#..."
             "..#.."
             ".#..."
             "#####"
             "....."),
    0x2260: ("...#."   # ≠
             "...#."
             "#####"
             "..#.."
             "#####"
             ".#..."
             ".#..."),
    0x2264: ("....."   # ≤
             "...#."
             ".##.."
             "#...."
             ".##.."
             "...#."
             "#####"),
    0x2265: ("....."   # ≥
             ".#..."
             "..##."
             "....#"
             "..##."
             ".#..."
             "#####"),
    0x229E: ("....."   # ⊞
             "#####"
             "#.#.#"
             "#####"
             "#.#.#"
             "#####"
             "....."),
    0x2315: ("....."   # ⌕
             ".###."
             "#...#"
             "#...#"
             ".###."
             "...#."
             "....#"),
    0x25A1: ("....."   # □
             "#####"
             "#...#"
             "#...#"
             "#...#"
             "#####"
             "....."),
    0x25A4: ("....."   # ▤
             "#####"
             "#...#"
             "#####"
             "#...#"
             "#####"
             "....."),
    0x25A6: ("....."   # ▦
             "#####"
             "#.#.#"
             "#####"
             "#.#.#"
             "#####"
             "....."),
    0x25B2: ("....."   # ▲
             "....."
             "..#.."
             ".###."
             "#####"
             "....."
             "....."),
    0x25BC: ("....."   # ▼
             "....."
             "#####"
             ".###."
             "..#.."
             "....."
             "....."),
    0x25B6: ("....."   # ▶
             "#...."
             "##..."
             "###.."
             "##..."
             "#...."
             "....."),
    0x25B8: ("....."   # ▸
             "....."
             ".#..."
             ".##.."
             ".#..."
             "....."
             "....."),
    0x25C9: ("....."   # ◉
             ".###."
             "#...#"
             "#.#.#"
             "#...#"
             ".###."
             "....."),
    0x25CE: ("....."   # ◎
             ".###."
             "#...#"
             "#.#.#"
             "#...#"
             ".###."
             "....."),
    0x25D1: ("....."   # ◑
             ".###."
             "#..##"
             "#..##"
             "#..##"
             ".###."
             "....."),
    0x2630: ("....."   # ☰
             "#####"
             "....."
             "#####"
             "....."
             "#####"
             "....."),
    0x2665: ("....."   # ♥
             ".#.#."
             "#####"
             "#####"
             ".###."
             "..#.."
             "....."),
    0x267F: ("..#.."   # ♿
             "....."
             "#####"
             "..#.."
             "..#.#"
             ".##.."
             "....."),
    0x26B7: ("....."   # ⚷
             ".###."
             "#...#"
             "#...#"
             ".###."
             "..#.."
             "..##."),
    0x270E: ("....#"   # ✎
             "...##"
             "..##."
             ".##.."
             "###.."
             "##..."
             "....."),
    0x2713: ("....."   # ✓
             "....#"
             "...#."
             "#..#."
             "#.#.."
             ".##.."
             "....."),
    0x271A: ("....."   # ✚
             "..#.."
             "..#.."
             "#####"
             "..#.."
             "..#.."
             "....."),
    0x2726: ("....."   # ✦
             "..#.."
             ".###."
             "#####"
             ".###."
             "..#.."
             "....."),
    0x2B21: ("....."   # ⬡
             ".###."
             "#...#"
             "#...#"
             "#...#"
             ".###."
             "....."),
}


def from_pixels(bits):
    """Convertit une matrice 5 × 7 en rectangles. Colonne 0 → x = 128 ;
    rangée 0 → le haut de la capitale (y = 768…896)."""
    boxes = []
    for row in range(7):
        for col in range(5):
            if bits[row * 5 + col] != "#":
                continue
            x0 = PX + col * PX
            y0 = (6 - row) * PX
            boxes.append((x0, y0, x0 + PX, y0 + PX))
    return tuple(boxes)


# Signes qui n'ont pas besoin d'un dessin : ils redoublent un glyphe existant.
ALIAS = {0xFF0B: "+", 0x00A0: " ", 0x202F: " "}

# Minuscules accentuées hors du jeu français : le Minitel ne les avait pas non
# plus, et les replier vaut mieux qu'une fonte de secours mal proportionnée.
FOLD_LOWER = {
    "áãåā": "a", "ìíĩ": "i", "òóõø": "o", "úũ": "u", "ñ": "n",
    "ý": "y", "š": "s", "ž": "z", "ćč": "c", "đ": "d", "ğ": "g",
}

# Capitales accentuées → lettre de base. Le Vidéotex n'a jamais eu les
# premières ; les afficher revenait à ne pas les accentuer.
FOLD = {
    "ÀÁÂÃÄÅ": "A", "Ç": "C", "ÈÉÊË": "E", "ÌÍÎÏ": "I", "Ñ": "N",
    "ÒÓÔÕÖ": "O", "ÙÚÛÜ": "U", "ÝŸ": "Y", "Œ": "O", "Æ": "A", "ẞ": "S",
}


def main():
    here = os.path.dirname(os.path.abspath(__file__))
    root = os.path.dirname(here)
    cache = os.path.join(root, "scripts", ".minitel-source.ttf")
    out = os.path.join(root, "public", "minitel.woff2")

    if not os.path.exists(cache):
        print("Téléchargement de la fonte source (CC0) …")
        urllib.request.urlretrieve(SOURCE, cache)

    import hashlib
    digest = hashlib.md5(open(cache, "rb").read()).hexdigest()
    if digest != SOURCE_MD5:
        sys.exit("La fonte source a changé (md5 %s). Vérifier avant de "
                 "poursuivre." % digest)

    font = TTFont(cache)
    glyf, hmtx, cmap = font["glyf"], font["hmtx"], font.getBestCmap()

    added = 0
    drawn = [(c, from_pixels(b)) for c, b in PIXELS.items()]
    for code, boxes in list(BOX.items()) + list(BLOCKS.items()) + \
            list(SHADES.items()) + drawn + [(0x20AC, euro())]:
        name = "uni%04X" % code
        # `glyf.__setitem__` tient l'ordre des glyphes à jour de lui-même ;
        # l'allonger à la main le ferait en double.
        glyph = rects(*boxes)
        glyf[name] = glyph
        # Le « left side bearing » DOIT valoir le xMin du tracé : sinon le
        # moteur de rendu décale le glyphe de la différence, et un filet
        # vertical (xMin 384) se retrouve 3 pixels trop à gauche — les angles
        # ne se referment plus. C'est l'avance qui reste pleine chasse.
        glyph.recalcBounds(glyf)
        hmtx[name] = (CELL_W, glyph.xMin)
        added += 1

    order = glyf.glyphOrder
    font.setGlyphOrder(order)
    if hasattr(font, "_reverseGlyphOrderDict"):
        del font._reverseGlyphOrderDict
    font["maxp"].numGlyphs = len(order)

    # Réécriture complète du cmap : les nouveaux codes, plus les replis.
    table = dict(cmap)
    for code in list(BOX) + list(BLOCKS) + list(SHADES) + list(PIXELS) + [0x20AC]:
        table[code] = "uni%04X" % code
    for code, same_as in ALIAS.items():
        if cmap.get(ord(same_as)):
            table[code] = cmap[ord(same_as)]

    folded = 0
    for accented, base in list(FOLD.items()) + list(FOLD_LOWER.items()):
        target = cmap.get(ord(base))
        if not target:
            continue
        for ch in accented:
            table[ord(ch)] = target
            folded += 1

    sub = font["cmap"].tables[0].__class__(4)
    sub.platformID, sub.platEncID, sub.language = 3, 1, 0
    sub.cmap = table
    sub4 = newTable("cmap")
    sub4.tableVersion = 0
    sub4.tables = [sub]
    font["cmap"] = sub4

    font["name"].setName("Xenna Minitel", 1, 3, 1, 0x409)
    font["name"].setName("Xenna Minitel", 4, 3, 1, 0x409)
    font["name"].setName(
        "Jeu G0 EF9345 de Frédéric Bisson (CC0 1.0), étendu pour Xenna Paie : "
        "filets, pavés, €, capitales repliées.", 10, 3, 1, 0x409)

    os.makedirs(os.path.dirname(out), exist_ok=True)
    font.flavor = "woff2"
    font.save(out)
    print("%s — %d glyphes ajoutés, %d capitales repliées, %d octets"
          % (out, added, folded, os.path.getsize(out)))


if __name__ == "__main__":
    main()
