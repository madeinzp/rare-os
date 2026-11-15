// Checks the on-screen keyboard Rare injects into web games and apps (webgame.js) in headless
// Chrome: every ASCII symbol can be typed with the pad through ?123 and #+=, LT and Y switch the
// pages, web address and email fields get their keys and .com, held X and a finger held on Delete
// repeat, and touch types without taking focus from the field.
//
// Usage: node scripts/osk-check.mjs [--shots dir]     (CHROME=/path/to/chrome to pick a browser)
import puppeteer from "puppeteer-core";
import { mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

const CHROME = process.env.CHROME ?? "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
const shotsAt = process.argv.indexOf("--shots");
const shots = shotsAt > 0 ? process.argv[shotsAt + 1] : null;
if (shots) mkdirSync(shots, { recursive: true });
const js = readFileSync(new URL("../crates/rare-webgame/src/webgame.js", import.meta.url), "utf8");
const KB = JSON.parse(js.match(/const KB = (.*);\n/)[1]);
const SYMBOLS = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

const page = `<!doctype html><body style="background:#000;margin:0;height:100vh">
<input id="t" type="text" style="width:400px;height:40px">
<input id="u" type="url" style="width:400px;height:40px">
<input id="e" type="email" style="width:400px;height:40px"></body>`;

const fails = [];
const check = (ok, what) => {
  console.log(`${ok ? "ok  " : "FAIL"} ${what}`);
  if (!ok) fails.push(what);
};

const b = await puppeteer.launch({ executablePath: CHROME, headless: true, args: ["--window-size=1280,800"] });
try {
  const p = await b.newPage();
  await p.setViewport({ width: 1280, height: 800 });
  await p.evaluateOnNewDocument('window.__rareProfile = {"input":"focus"};');
  await p.evaluateOnNewDocument(js);
  await p.goto("data:text/html," + encodeURIComponent(page));
  const pad = (btn, down = true) => p.evaluate((b, d) => window.__rarePad(b, d), btn, down);
  const tap = async (btn) => {
    await pad(btn, true);
    await pad(btn, false);
  };
  const val = (id) => p.$eval(id, (e) => e.value);
  const open = async (id) => {
    await p.$eval(id, (e) => e.focus());
    await tap("select");
  };
  const keyText = (r, c) => p.$eval(`#__rare_osk b[data-k="${r},${c}"]`, (e) => e.textContent);

  // Every symbol through the pad: LT (?123), Y (#+=) when it is only there, then the d-pad.
  await open("#t");
  for (const ch of SYMBOLS) {
    const inSym = KB.sym.findIndex((r) => r.includes(ch));
    const grid = inSym >= 0 ? KB.sym : KB.more;
    const r = grid.findIndex((row) => row.includes(ch));
    const c = Array.from(grid[r]).indexOf(ch);
    await tap("lt");
    if (inSym < 0) await tap("y");
    for (let i = 1; i < r; i++) await tap("down");
    if (r === 0) await tap("up");
    for (let i = 0; i < c; i++) await tap("right");
    await tap("a");
    // B closes; the next symbol starts from a fresh keyboard.
    await tap("b");
    await open("#t");
  }
  check((await val("#t")) === SYMBOLS, `pad types every ASCII symbol (${await val("#t")})`);
  // The keys on screen say where they lead.
  await tap("lt");
  check((await keyText(4, 0)).endsWith("#+=") && (await keyText(4, 1)).endsWith("abc"), "?123 page: #+= and abc keys");
  if (shots) await p.screenshot({ path: join(shots, "osk-web-symbols.png") });
  await tap("y");
  check((await keyText(4, 0)).endsWith("123") && (await keyText(1, 6)) === "^", "#+= page: 123 key and ^");
  if (shots) await p.screenshot({ path: join(shots, "osk-web-more.png") });
  await tap("lt");
  check((await keyText(1, 0)) === "q" && (await keyText(4, 1)).endsWith("?123"), "LT back to letters");

  // Held X deletes again and again.
  const before = (await val("#t")).length;
  await pad("x", true);
  await new Promise((r) => setTimeout(r, 1100));
  await pad("x", false);
  const gone = before - (await val("#t")).length;
  check(gone >= 6 && gone <= 14, `held X repeats (${gone} deleted in 1.1 s)`);
  await new Promise((r) => setTimeout(r, 300));
  check(before - (await val("#t")).length === gone, "and stops on release");
  // A held on Shift does not repeat.
  await tap("b");

  // A web address field: . - / : on the letters, .com by touch, the field keeps focus.
  await open("#u");
  check((await keyText(3, 9)) === "/" && (await keyText(2, 9)) === ":", "URL field: / and : on the letters");
  check((await keyText(4, 2)) === ".com", "URL field: .com key");
  if (shots) await p.screenshot({ path: join(shots, "osk-web-url.png") });
  const touch = async (r, c, ms = 30) => {
    const box = await (await p.$(`#__rare_osk b[data-k="${r},${c}"]`)).boundingBox();
    await p.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
    await p.mouse.down();
    await new Promise((res) => setTimeout(res, ms));
    await p.mouse.up();
  };
  await touch(1, 2);
  await touch(4, 2);
  check((await val("#u")) === "e.com", `touch types (${await val("#u")})`);
  check(await p.$eval("#u", (e) => document.activeElement === e), "the field keeps focus");
  // A finger held on Delete repeats, then stops.
  await p.$eval("#u", (e) => (e.value = "x".repeat(40)));
  await touch(4, 4, 1000);
  const left = (await val("#u")).length;
  check(left <= 34 && left >= 26, `finger held on Delete repeats (${40 - left} deleted)`);
  await tap("b");

  // Email: @ stays, .com is there.
  await open("#e");
  check((await keyText(2, 9)) === "@" && (await keyText(4, 2)) === ".com", "email field: @ and .com");
  await tap("b");
} finally {
  await b.close();
}
console.log(fails.length ? `${fails.length} failed` : "all passed");
process.exit(fails.length ? 1 : 0);
