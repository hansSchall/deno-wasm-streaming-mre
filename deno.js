import { main } from "./main.js";
import __wbg_init, { foo } from "./pkg/src.js";

const data = await Deno.readFile(new URL("./pkg/src_bg.wasm", import.meta.url));

__wbg_init({
    module_or_path: data,
}).then(main);
