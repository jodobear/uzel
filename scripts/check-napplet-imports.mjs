import { createRequire } from 'node:module';
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { extname, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(fileURLToPath(new URL('..', import.meta.url)));
const requireFromShell = createRequire(
  new URL('../apps/uzel/package.json', import.meta.url),
);
const ts = requireFromShell('typescript');
const { compile: compileSvelte } = requireFromShell('svelte/compiler');
const forbiddenSpecifier = /(uzel|napd|tauri)/i;
const dependencyGroups = [
  'dependencies',
  'devDependencies',
  'optionalDependencies',
  'peerDependencies',
];
const directNetworkAuthority =
  /(fetch|XMLHttpRequest|WebSocket|EventSource|sendBeacon|serviceWorker)/;
const sourceExtensions = new Set([
  '.cjs',
  '.cts',
  '.js',
  '.jsx',
  '.mjs',
  '.mts',
  '.svelte',
  '.ts',
  '.tsx',
]);

function sourceUnits(path, source) {
  if (extname(path) !== '.svelte') {
    return [{ path, source }];
  }

  const compiled = compileSvelte(source, { filename: path, generate: 'client' });
  return [{ path: `${path}#compiled`, source: compiled.js.code }];
}

function importedSpecifier(node) {
  if (ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) {
    return node.moduleSpecifier;
  }
  if (
    ts.isCallExpression(node) &&
    (node.expression.kind === ts.SyntaxKind.ImportKeyword ||
      (ts.isIdentifier(node.expression) && node.expression.text === 'require'))
  ) {
    return node.arguments[0] ?? null;
  }
  if (ts.isImportEqualsDeclaration(node) && ts.isExternalModuleReference(node.moduleReference)) {
    return node.moduleReference.expression;
  }
  if (ts.isImportTypeNode(node) && ts.isLiteralTypeNode(node.argument)) {
    return node.argument.literal;
  }
  return null;
}

function literalValue(node) {
  if (node && (ts.isStringLiteral(node) || ts.isNoSubstitutionTemplateLiteral(node))) {
    return node.text;
  }
  return null;
}

function violations(path, source) {
  const parsed = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true);
  const found = [];

  function visit(node) {
    const specifierNode = importedSpecifier(node);
    if (specifierNode) {
      const specifier = literalValue(specifierNode);
      const dynamic = ts.isCallExpression(node);
      if ((specifier && forbiddenSpecifier.test(specifier)) || (dynamic && specifier === null)) {
        const position = parsed.getLineAndCharacterOfPosition(node.getStart(parsed));
        found.push({
          column: position.character + 1,
          line: position.line + 1,
          specifier: specifier ?? '<dynamic>',
        });
      }
    }
    ts.forEachChild(node, visit);
  }

  visit(parsed);
  return found;
}

function dependencyViolations(packageJson) {
  return dependencyGroups.flatMap((group) =>
    Object.entries(packageJson[group] ?? {}).flatMap(([name, target]) =>
      forbiddenSpecifier.test(`${name}\n${target}`) ? [{ group, name, target }] : [],
    ),
  );
}

function runSelfTest() {
  const expectedExtensions = [
    '.cjs',
    '.cts',
    '.js',
    '.jsx',
    '.mjs',
    '.mts',
    '.svelte',
    '.ts',
    '.tsx',
  ];
  for (const extension of expectedExtensions) {
    if (!sourceExtensions.has(extension)) {
      throw new Error(`boundary self-test omitted source extension: ${extension}`);
    }
  }

  const rejected = [
    "import 'uzel/runtime';",
    'import("@tauri-apps/api");',
    'import(`@tauri-apps/api`);',
    "import /* lazy */ ('../../../apps/uzel/src/main.ts');",
    "import // lazy\n('../../../apps/uzel/src/main.ts');",
    "import(/* lazy */ '../../../apps/uzel/src/main.ts');",
    "export { runtime } from 'napd/runtime';",
    "const runtime = require('napd/runtime');",
    "import(prefix + '/tauri');",
  ];
  for (const source of rejected) {
    if (violations('<boundary-self-test>', source).length === 0) {
      throw new Error(`boundary self-test failed: ${source}`);
    }
  }
  if (violations('<boundary-self-test>', "import { get } from '@napplet/nap';").length > 0) {
    throw new Error('boundary self-test rejected an allowed NAP import');
  }
  if (
    dependencyViolations({ dependencies: { bridge: 'npm:@tauri-apps/api@2' } }).length === 0
  ) {
    throw new Error('boundary self-test accepted an aliased runtime dependency');
  }
  if (
    dependencyViolations({ dependencies: { '@napplet/nap': '0.29.0' } }).length > 0
  ) {
    throw new Error('boundary self-test rejected an allowed NAP dependency');
  }
  if (!directNetworkAuthority.test("const socket = new WebSocket('wss://example.test')")) {
    throw new Error('boundary self-test accepted direct network authority');
  }
  const svelteImport = sourceUnits(
    '<boundary-self-test>.svelte',
    "{#await import('../../../apps/uzel/src/main.ts')}<p>wait</p>{/await}",
  )[0];
  if (violations(svelteImport.path, svelteImport.source).length === 0) {
    throw new Error('boundary self-test accepted a Svelte template import');
  }
  const svelteNetwork = sourceUnits(
    '<boundary-self-test>.svelte',
    '<button onclick={() => fetch(url)}>go</button>',
  )[0];
  if (!directNetworkAuthority.test(svelteNetwork.source)) {
    throw new Error('boundary self-test accepted Svelte template network authority');
  }
}

function sourceFiles(directory) {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) {
      return entry.name === 'dist' || entry.name === 'node_modules' ? [] : sourceFiles(path);
    }
    return sourceExtensions.has(extname(path)) ? [path] : [];
  });
}

runSelfTest();

let failed = false;
const nappletsRoot = join(repositoryRoot, 'napplets');
for (const entry of readdirSync(nappletsRoot, { withFileTypes: true })) {
  if (!entry.isDirectory()) {
    continue;
  }
  const packagePath = join(nappletsRoot, entry.name, 'package.json');
  if (!existsSync(packagePath)) {
    continue;
  }
  const packageJson = JSON.parse(readFileSync(packagePath, 'utf8'));
  for (const violation of dependencyViolations(packageJson)) {
    console.error(
      `${relative(repositoryRoot, packagePath)}: forbidden napplet dependency ` +
        `${violation.group}.${violation.name} -> ${violation.target}`,
    );
    failed = true;
  }
}

for (const path of sourceFiles(nappletsRoot)) {
  const source = readFileSync(path, 'utf8');
  for (const unit of sourceUnits(path, source)) {
    const productNapplet =
      unit.path.startsWith(join(nappletsRoot, 'follow-list')) ||
      unit.path.startsWith(join(nappletsRoot, 'profile-card'));
    if (productNapplet && directNetworkAuthority.test(unit.source)) {
      console.error(
        `${relative(repositoryRoot, unit.path)}: direct browser network authority is forbidden`,
      );
      failed = true;
    }
    for (const violation of violations(unit.path, unit.source)) {
      console.error(
        `${relative(repositoryRoot, unit.path)}:${violation.line}:${violation.column}: ` +
          `forbidden napplet import ${violation.specifier}`,
      );
      failed = true;
    }
  }
}

if (failed) {
  process.exitCode = 1;
}
