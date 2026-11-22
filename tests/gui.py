"""Exerce la fenêtre WebView2 de l’exécutable compilé sur le runner Windows."""
import os
from pathlib import Path
import subprocess
import tempfile
import time
import urllib.error
import urllib.request

from playwright.sync_api import expect, sync_playwright


def exercise():
    endpoint = "http://127.0.0.1:9222"
    env = dict(os.environ, WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS="--remote-debugging-port=9222")
    exe = Path("target/release/evo75-keylab.exe").resolve()
    env["WEBVIEW2_USER_DATA_FOLDER"] = tempfile.mkdtemp(prefix="keylab-webview-")
    local_http = urllib.request.build_opener(urllib.request.ProxyHandler({}))
    with subprocess.Popen([str(exe)], env=env) as app:
        try:
            for _ in range(50):
                if app.poll() is not None:
                    raise RuntimeError(f"La fenêtre s’est fermée (code {app.returncode})")
                try:
                    with local_http.open(endpoint + "/json/list", timeout=1) as response:
                        if b'"page"' in response.read():
                            break
                except (OSError, urllib.error.URLError):
                    pass
                time.sleep(0.5)
            else:
                subprocess.run(["powershell", "-NoProfile", "-Command",
                                "Get-Process evo75-keylab,msedgewebview2 -ErrorAction SilentlyContinue | "
                                "Select-Object ProcessName,MainWindowTitle,Responding | Format-Table"])
                raise RuntimeError("Aucune fenêtre WebView2 détectée")

            with sync_playwright() as driver:
                browser = driver.chromium.connect_over_cdp(endpoint)
                page = browser.contexts[0].pages[0]
                expect(page.get_by_role("heading", name="Atelier d’éclairage")).to_be_visible()
                expect(page.locator("#connection")).to_have_text("Récepteur absent")
                page.screenshot(path="target/keylab.png")
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
                expect(page.locator("#device-summary")).to_contain_text("Aucun récepteur")
        finally:
            if app.poll() is None:
                app.terminate()


if __name__ == "__main__":
    exercise()
