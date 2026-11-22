# Evo75-KeyLab

Application Windows en français pour préparer les profils d’éclairage de l’Evo75 et identifier son récepteur 2,4 GHz. La fenêtre associe une interface locale Tauri à la détection HID en Rust.

## Ouvrir l’application

Télécharge l’archive **evo75-keylab-windows-x64** depuis une [compilation Windows réussie](https://github.com/ERICAntoine/Evo75-KeyLab/actions/workflows/windows.yml). Extrais le ZIP et double-clique sur **evo75-keylab.exe**. La fenêtre utilise le [runtime Microsoft Edge WebView2](https://developer.microsoft.com/microsoft-edge/webview2/).

- **Éclairage** affiche un aperçu des touches et du bandeau. Pour chaque zone, choisis une teinte, une luminosité et un effet. Les raccourcis affichés commandent les LED du clavier et le bouton **Ouvrir Ticktype** ouvre le configurateur du fabricant dans le navigateur.
- **Profils** donne accès à Bureau, Crépuscule et Nuit. KeyLab enregistre automatiquement les modifications sur ce PC.
- **Diagnostic** recherche le récepteur `36B0:3002` et affiche les collections HID présentes. **Actualiser** relance la recherche après un changement de connexion.

Dans **Sur le clavier**, **Tester VIA** interroge la version du protocole puis lit la couleur du canal RGB Matrix standard. Si ces lectures aboutissent, **Envoyer la teinte des touches** écrit la teinte et la saturation et les relit. Le bandeau, la luminosité et les effets restent locaux. Pour le réglage matériel, consulte le [manuel Evo75](https://ticktype.com/pages/product-user-manual-evo75).

## Développement

Sur Windows x64 avec Rust stable MSVC, les outils de compilation Visual Studio et WebView2, lance `cargo run` depuis le dépôt. La commande `cargo run --bin diagnostic` ouvre le diagnostic en console.

La compilation Windows exécute le diagnostic puis teste la fenêtre, la navigation entre les trois vues et la persistance des profils. La capture de l’interface est disponible dans l’archive **apercu-keylab** du même workflow.
