import init, { run_app } from './pkg/campagnamica';
async function main() {
   await init('/pkg/campagnamica_bg.wasm');
   run_app();
}
main()