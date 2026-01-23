export function helloModule() {
    console.log("📦 Module System: Operational");
    if (window.ipc) {
        window.ipc.postMessage("MODULE_LOADED");
    }
    return true;
}
