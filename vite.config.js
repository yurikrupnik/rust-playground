import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import yargs from 'yargs';
const argv = yargs.argv;
const { port, open } = argv;
console.log(argv);
export default defineConfig(async () => {
  return {
    build: {
      // manifest: true,
    },
    server: {
      port,
      open,
    },
    plugins: [solidPlugin()],
  };
});
