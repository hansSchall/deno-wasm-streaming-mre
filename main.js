import { foo } from "./pkg/src.js";

export function main() {
    foo().then(() => {
        console.log("finished");
    }).catch(() => {
        console.log("errored");
    });

    // setTimeout(() => {
    //     console.log("timeout");
    // }, 1000);
}
