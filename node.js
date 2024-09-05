import { readFile } from "node:fs/promises";
import __wbg_init from "./pkg/src.js";
import { main } from "./main.js";

const data = await readFile(new URL("./pkg/src_bg.wasm", import.meta.url));

__wbg_init({
    module_or_path: data,
}).then(main);
