const esbuild = require('esbuild');

esbuild.build({
  entryPoints: ['package.js'],
  bundle: true,
  outfile: 'src/package.js',
  format: 'esm',
  sourcemap: 'inline',
  minify: true,
}).catch(() => process.exit(1));