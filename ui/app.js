"use strict";

const $ = (selector) => document.querySelector(selector);
const $$ = (selector) => [...document.querySelectorAll(selector)];
const defaults = {
  Bureau: {keys:{color:"#91ecd8",level:70,effect:"solid"},strip:{color:"#bda2f7",level:75,effect:"solid"}},
  Crépuscule: {keys:{color:"#bda2f7",level:65,effect:"solid"},strip:{color:"#ffb191",level:85,effect:"pulse"}},
  Nuit: {keys:{color:"#85b6ff",level:20,effect:"solid"},strip:{color:"#91ecd8",level:0,effect:"off"}}
};
const palette = [
  ["Menthe","#91ecd8"], ["Lavande","#bda2f7"], ["Pêche","#ffb191"], ["Bleu","#85b6ff"]
];
const prefix = "evo75.profil.v1.";
let slot = "Bureau";
let zone = "keys";
const profiles = structuredClone(defaults);

function validZone(z) {
  return z && /^#[0-9a-f]{6}$/i.test(z.color) && Number.isInteger(z.level) &&
    z.level >= 0 && z.level <= 100 && ["solid","pulse","off"].includes(z.effect);
}

function restore() {
  for (const name of Object.keys(defaults)) {
    try {
      const stored = JSON.parse(localStorage.getItem(prefix + name));
      if (stored && validZone(stored.keys) && validZone(stored.strip)) profiles[name] = stored;
    } catch { /* Le profil prédéfini reste disponible. */ }
  }
  try {
    const selected = localStorage.getItem(prefix + "active");
    if (Object.hasOwn(defaults, selected)) slot = selected;
  } catch { /* Le stockage peut être indisponible. */ }
}

function chooseProfile(name) {
  slot = name;
  let status = "";
  try { localStorage.setItem(prefix + "active", name); }
  catch { status = "Le stockage local est indisponible."; }
  $("#slot").value = name;
  showPage("lighting");
  renderEditor();
  $("#save-status").textContent = status;
}

function edit(change) {
  Object.assign(profiles[slot][zone], change);
  let status = "Enregistré sur ce PC.";
  try { localStorage.setItem(prefix + slot, JSON.stringify(profiles[slot])); }
  catch { status = "Aperçu modifié. L’enregistrement local a échoué."; }
  renderEditor();
  $("#save-status").textContent = status;
}

function showPage(page) {
  for (const section of $$("main > section")) section.hidden = section.id !== page;
  for (const button of $$("nav button")) {
    if (button.dataset.page === page) button.setAttribute("aria-current","page");
    else button.removeAttribute("aria-current");
  }
  if (page === "profiles") renderProfiles();
  if (page === "diagnostic") scan();
  window.scrollTo(0, 0);
}

function makeKeys() {
  const rows = [
    ["Esc","F1","F2","F3","F4","F5","F6","F7","F8","F9","F10","F11","F12","Prt","Del"],
    ["~","1","2","3","4","5","6","7","8","9","0","−","+","Back:2","Home"],
    ["Tab:1.5","Q","W","E","R","T","Y","U","I","O","P","[","]","\\:1.5","PgUp"],
    ["Caps:1.75","A","S","D","F","G","H","J","K","L",";","’","Enter:2.25","PgDn"],
    ["Shift:2.25","Z","X","C","V","B","N","M",",",".","/","Shift:1.75","↑","End"],
    ["Ctrl:1.25","Win:1.25","Alt:1.25"," :6.25","Alt","Fn","Ctrl","←","↓","→"]
  ];
  for (const [index, keys] of rows.entries()) {
    const row = document.createElement("div");
    row.className = "key-row";
    for (const item of keys) {
      const [label, width] = item.split(":");
      const key = document.createElement("span");
      key.className = "key";
      key.textContent = label;
      if (width) key.style.setProperty("--units",width);
      row.append(key);
      if (index === 0 && label === "F12") {
        const badge = document.createElement("i");
        badge.className = "badge";
        row.append(badge);
      }
    }
    $("#keybed").append(row);
  }
}

function ledColor(z) {
  const level = z.effect === "off" ? 0 : z.level;
  return "color-mix(in srgb, " + z.color + " " + level + "%, #18242a)";
}

function renderEditor() {
  const profile = profiles[slot];
  const z = profile[zone];
  $("#effect").value = z.effect;
  $("#color").value = z.color;
  $("#level").value = z.level;
  $("#percent").textContent = z.level + " %";
  $("#level").disabled = $("#color").disabled = z.effect === "off";
  for (const swatch of $$(".swatch")) {
    swatch.disabled = z.effect === "off";
    swatch.setAttribute("aria-pressed", swatch.dataset.color === z.color);
  }
  for (const button of $$("[data-zone]")) button.setAttribute("aria-pressed", button.dataset.zone === zone);
  for (const part of ["keys","strip"]) {
    $(".stage").style.setProperty(part === "keys" ? "--key-color" : "--strip-color", ledColor(profile[part]));
    $("#keybed").classList.toggle("pulse-" + part, profile[part].effect === "pulse" && profile[part].level > 0);
  }
  const shortcuts = zone === "keys" ? ["X / C","V / B","Z"] : ["W / E","R / T","Q"];
  $("#shortcuts").replaceChildren();
  ["Changer d’effet","Changer de teinte","Allumer / éteindre"].forEach((label,i) => {
    const row = document.createElement("div");
    const term = document.createElement("dt");
    const def = document.createElement("dd");
    const key = document.createElement("kbd");
    term.textContent = label;
    key.textContent = "Fn + " + shortcuts[i];
    def.append(key); row.append(term,def); $("#shortcuts").append(row);
  });
}

function renderProfiles() {
  $("#profile-list").replaceChildren();
  for (const [name, profile] of Object.entries(profiles)) {
    const button = document.createElement("button");
    button.className = "card profile-card";
    const colors = document.createElement("i");
    colors.style.setProperty("--a",ledColor(profile.keys));
    colors.style.setProperty("--b",ledColor(profile.strip));
    const text = document.createElement("span");
    const title = document.createElement("strong");
    const note = document.createElement("small");
    title.textContent = name;
    note.textContent = "Touches " + profile.keys.level + " % · Bandeau " + profile.strip.level + " %";
    const arrow = document.createElement("b");
    arrow.textContent = "→";
    text.append(title,note); button.append(colors,text,arrow);
    button.onclick = () => chooseProfile(name);
    $("#profile-list").append(button);
  }
}

const hex = (n) => "0x" + n.toString(16).toUpperCase().padStart(4,"0");
async function scan() {
  const button = $("#scan");
  if (button.disabled) return;
  button.disabled = true;
  $("#connection").textContent = "Recherche…";
  try {
    const rows = await window.__TAURI__.core.invoke("scan");
    $("#connection").textContent = rows.length ? "Récepteur détecté" : "Récepteur absent";
    $("#status-dot").classList.toggle("online", rows.length > 0);
    $("#interfaces").hidden = rows.length === 0;
    const qmk = rows.some(row => row.usage_page === 0xff60 && row.usage === 0x61);
    $("#device-summary").textContent = rows.length
      ? rows.length + " collection(s) HID. " + (qmk ? "Usage QMK Raw HID détecté." : "Usage QMK Raw HID par défaut absent.")
      : "Aucun récepteur 2,4 GHz détecté. Branche le dongle Evo75, puis clique sur Actualiser.";
    const body = $("#interfaces tbody");
    body.replaceChildren();
    for (const row of rows) {
      const tr = body.insertRow();
      for (const value of [row.product,row.interface,hex(row.usage_page),hex(row.usage)]) tr.insertCell().textContent = value;
    }
  } catch (error) {
    $("#status-dot").classList.remove("online");
    $("#connection").textContent = "Détection indisponible";
    $("#interfaces").hidden = true;
    $("#device-summary").textContent = "Le diagnostic a échoué. " + String(error);
  } finally { button.disabled = false; }
}

restore();
makeKeys();
for (const name of Object.keys(profiles)) $("#slot").add(new Option(name,name));
$("#slot").value = slot;
$("#slot").onchange = (event) => chooseProfile(event.target.value);
for (const button of $$("[data-page]")) button.onclick = () => showPage(button.dataset.page);
for (const button of $$("[data-zone]")) button.onclick = () => { zone = button.dataset.zone; renderEditor(); };
for (const [name,color] of palette) {
  const button = document.createElement("button");
  button.className = "swatch";
  button.title = name;
  button.setAttribute("aria-label",name);
  button.dataset.color = color;
  button.style.setProperty("--swatch",color);
  button.onclick = () => edit({color});
  $("#palette").append(button);
}
$("#effect").onchange = e => edit({effect:e.target.value});
$("#color").oninput = e => edit({color:e.target.value});
$("#level").oninput = e => edit({level:Number(e.target.value)});
$("#scan").onclick = scan;
$("#official").onclick = async () => {
  $("#open-error").textContent = "";
  try { await window.__TAURI__.core.invoke("open_configurator"); }
  catch (error) { $("#open-error").textContent = "Ouverture impossible : " + String(error); }
};
renderEditor();
scan();

let viaReady = false;
async function queryVia(apply) {
  const probe = $("#probe");
  const send = $("#apply-color");
  probe.disabled = send.disabled = true;
  $("#via-result").textContent = "Échange avec le récepteur…";
  try {
    let color = null;
    if (apply) {
      const rgb = profiles[slot].keys.color.slice(1).match(/../g).map(v => parseInt(v, 16));
      const [r,g,b] = rgb, max = Math.max(...rgb), delta = max - Math.min(...rgb);
      let h = 0;
      if (delta) h = max === r ? (g-b)/delta : max === g ? 2+(b-r)/delta : 4+(r-g)/delta;
      color = [Math.round(((h+6)%6)/6*255), max ? Math.round(delta/max*255) : 0];
    }
    $("#via-result").textContent = await window.__TAURI__.core.invoke("query_via", {color});
    viaReady = true;
  } catch (error) {
    viaReady = false;
    $("#via-result").textContent = String(error);
  } finally {
    probe.disabled = false;
    send.disabled = !viaReady;
  }
}
$("#probe").onclick = () => queryVia(false);
$("#apply-color").onclick = () => queryVia(true);
