#!/usr/bin/env python3
"""Illustration d'accueil (bureau) : hexega2.jpg → public/hexega2.webp.

Le fond marine de l'image d'origine ne correspond pas au fond du site (--bg,
#0a0c10) : posée telle quelle, elle ferait un rectangle bleu. On la DÉ-MÉLANGE
contre son propre fond — chaque pixel est lu comme un trait de couleur C et
d'opacité α posé sur ce fond, estimé par un minimum local lissé — et on l'écrit
en WebP TRANSPARENT : seuls restent les néons, la page se voit au travers. Les
couleurs sont préservées (une simple soustraction aurait viré le bleu au cyan)
et l'image s'accorde d'elle-même à n'importe quelle couleur de fond.

Usage : python3 scripts/build-hexega.py [source.jpg]   (défaut : ../hexega2.jpg)
"""
import sys
import numpy as np
from PIL import Image, ImageFilter

source = sys.argv[1] if len(sys.argv) > 1 else '../hexega2.jpg'

src = Image.open(source).convert('RGB')
W, H = src.size
a = np.asarray(src).astype(np.float32) / 255
petit = src.resize((W // 8, H // 8), Image.BILINEAR).filter(ImageFilter.MinFilter(9))
fond = np.asarray(petit.resize((W, H), Image.BICUBIC).filter(ImageFilter.GaussianBlur(24))).astype(np.float32) / 255

alpha = np.clip(((a - fond) / np.maximum(1 - fond, 1e-3)).max(axis=2, keepdims=True), 0, 1)
# Couleur du trait, dé-mélangée AVANT tout retouche de l'opacité.
couleur = np.clip(fond + (a - fond) / np.maximum(alpha, 1e-3), 0, 1)

def lisse(x, bas, haut):
    t = np.clip((x - bas) / (haut - bas), 0, 1)
    return t * t * (3 - 2 * t)

# Le fond marine du JPEG porte du bruit de compression : dé-mélangé, il
# deviendrait un voile pommelé semi-transparent. Un seuil DOUX l'éteint (sous
# 2 % d'opacité) sans toucher aux traits ni à leurs vrais halos (au-delà de 9 %).
alpha = alpha * lisse(alpha, 0.02, 0.09)

# Fondu des bords : sur une bande étroite de chaque côté, l'opacité descend en
# douceur jusqu'à zéro, pour qu'aucun halo ne soit tranché net au ras du bord.
def rampe(n, part):
    t = np.clip(np.minimum(np.arange(n), np.arange(n)[::-1]) / (n * part), 0, 1)
    return t * t * (3 - 2 * t)
alpha = alpha * (rampe(H, 0.05)[:, None] * rampe(W, 0.05)[None, :])[:, :, None]

# Image TRANSPARENTE, SANS PERTE : les silhouettes seules, avec leur opacité ;
# c'est la page qui se voit au travers. Une compression avec perte faisait des
# blocs dans les halos. Une seule image, pleine résolution, sert au bureau comme
# au mobile (le navigateur la réduit proprement).
rgba = np.concatenate([couleur, alpha], axis=2)
img = Image.fromarray((rgba * 255 + 0.5).astype(np.uint8), 'RGBA')
img.save('public/hexega2.webp', lossless=True, quality=100, method=6)
print('public/hexega2.webp', W, 'x', H)
