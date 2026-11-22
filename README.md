# Evo75-KeyLab

Prototype Windows en Rust pour identifier les interfaces HID du récepteur **EVO75 2.4G** avant de développer les réglages RGB.

## Lancer le diagnostic sous Windows

Dans [GitHub Actions](https://github.com/ERICAntoine/Evo75-KeyLab/actions/workflows/windows.yml), ouvre une compilation réussie et télécharge l’archive `evo75-keylab-windows-x64` dans la section **Artifacts**. Extrais le ZIP, puis double-clique sur `evo75-keylab.exe`. La fenêtre affiche le résultat ou l’erreur et attend que tu appuies sur **Entrée**. Tu peux aussi lancer le diagnostic depuis PowerShell.

```powershell
.\evo75-keylab.exe
```

Pour compiler depuis les sources, installe Rust stable avec la chaîne MSVC pour Windows, puis lance `cargo run` à la racine du dépôt.

## Lire le résultat

Laisse le récepteur branché. L’outil énumère les collections HID de `36B0:3002`, l’identifiant sous lequel ce récepteur a été observé. Chaque ligne affiche le nom déclaré, le numéro d’interface et les deux valeurs HID qui décrivent son usage. Plusieurs lignes peuvent appartenir au même récepteur.

La mention `QMK Raw HID` signale la paire d’usage par défaut `FF60:0061`. C’est une piste pour tester VIA. Les commandes RGB acceptées en 2,4 GHz restent à établir. Cette version effectue uniquement une énumération et peut aussi être lancée sur un PC sans récepteur. [Documentation QMK Raw HID](https://docs.qmk.fm/features/rawhid)
