import { serveDir } from "@std/http";

export default {
    async fetch(req: Request) {
        const res = await serveDir(req);
        res.headers.set("Cross-Origin-Opener-Policy", "same-origin");
        res.headers.set("Cross-Origin-Embedder-Policy", "require-corp");
        return res;
    },
};
