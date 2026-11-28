"""Vérifie le démarrage natif puis les interactions de la vue dans Edge."""
from pathlib import Path
import subprocess
from playwright.sync_api import expect, sync_playwright


def check_window(exe):
    with subprocess.Popen([str(exe)]) as app:
        try:
            subprocess.run(["powershell", "-NoProfile", "-Command",
                f"$p = Get-Process -Id {app.pid}; "
                "for ($i = 0; $i -lt 60; $i++) { $p.Refresh(); "
                "if ($p.MainWindowTitle -eq 'Evo75 KeyLab' -and $p.Responding) { exit 0 }; "
                "Start-Sleep -Milliseconds 500 }; exit 1"], check=True)
        finally:
            if app.poll() is None:
                app.terminate()


def check_editor():
    with sync_playwright() as driver:
        browser = driver.chromium.launch(channel="msedge")
        page = browser.new_page(viewport={"width": 1160, "height": 815})
        page.add_init_script("""
            window.__TAURI__ = {core: {invoke: async (command, args) => {
                if (command === "query_via") {
                    window.appliedColor = args.color;
                    return "Couleur confirmée";
                }
                return [];
            }}};
        """)
        page.goto(Path("ui/index.html").resolve().as_uri())
        expect(page.get_by_role("heading", name="Atelier d’éclairage")).to_be_visible()
        expect(page.locator("#connection")).to_have_text("Evo75 absent")
        page.screenshot(path="target/keylab.png")
        page.locator("#color").fill("#ff0000")
        page.get_by_role("button", name="Appliquer la couleur des touches").click()
        expect(page.locator("#via-result")).to_have_text("Couleur confirmée")
        assert page.evaluate("window.appliedColor") == [0, 255]
        page.get_by_role("button", name="Bandeau", exact=True).click()
        page.get_by_role("button", name="Pêche", exact=True).click()
        page.locator("#effect").select_option("pulse")
        expect(page.locator("#keybed")).to_have_class("pulse-strip")
        page.reload()
        page.get_by_role("button", name="Bandeau", exact=True).click()
        expect(page.locator("#color")).to_have_value("#ffb191")
        expect(page.locator("#effect")).to_have_value("pulse")
        page.get_by_role("button", name="Profils", exact=True).click()
        page.get_by_role("button", name="Nuit", exact=False).click()
        expect(page.locator("#slot")).to_have_value("Nuit")
        expect(page.locator("#level")).to_be_disabled()
        page.get_by_role("button", name="Diagnostic", exact=True).click()
        page.get_by_role("button", name="Actualiser").click()
        expect(page.locator("#device-summary")).to_contain_text("Aucun Evo75")


if __name__ == "__main__":
    check_window(Path("target/release/evo75-keylab.exe").resolve())
    check_editor()
