import { createRequire } from 'node:module';
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { extname, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(fileURLToPath(new URL('..', import.meta.url)));
const requireFromShell = createRequire(
  new URL('../apps/uzel/package.json', import.meta.url),
);
const ts = requireFromShell('typescript');
const { compile: compileSvelte, parse: parseSvelte } = requireFromShell('svelte/compiler');
const forbiddenSpecifier = /(uzel|napd|tauri)/i;
const dependencyGroups = [
  'dependencies',
  'devDependencies',
  'optionalDependencies',
  'peerDependencies',
];
const directNetworkAuthority =
  /(fetch|XMLHttpRequest|WebSocket|EventSource|sendBeacon|serviceWorker)/;
const cssNetworkAuthority = /(@import\b|\b(?:image-set|url)\s*\()/i;
const resourceStyleDirective =
  /^(background|background-image|border-image|border-image-source|content|cursor|list-style|list-style-image|mask|mask-image)$/i;
const resourceAttributes = new Map([
  ['a', new Set(['href'])],
  ['area', new Set(['href'])],
  ['audio', new Set(['src'])],
  ['base', new Set(['href'])],
  ['button', new Set(['formaction'])],
  ['embed', new Set(['src'])],
  ['form', new Set(['action'])],
  ['iframe', new Set(['src', 'srcdoc'])],
  ['image', new Set(['href', 'xlink:href'])],
  ['img', new Set(['src', 'srcset'])],
  ['input', new Set(['formaction', 'src'])],
  ['link', new Set(['href'])],
  ['object', new Set(['data'])],
  ['script', new Set(['src'])],
  ['source', new Set(['src', 'srcset'])],
  ['track', new Set(['src'])],
  ['use', new Set(['href', 'xlink:href'])],
  ['video', new Set(['poster', 'src'])],
]);
const anyResourceAttribute = new Set(
  [...resourceAttributes.values()].flatMap((attributes) => [...attributes]),
);
const sourceExtensions = new Set([
  '.cjs',
  '.cts',
  '.html',
  '.js',
  '.jsx',
  '.mjs',
  '.mts',
  '.svelte',
  '.ts',
  '.tsx',
]);

function declarativeNetworkViolations(parsed, source) {
  const found = [];
  const seen = new WeakSet();

  function visit(node) {
    if (!node || typeof node !== 'object' || seen.has(node)) {
      return;
    }
    seen.add(node);

    if (node.type === 'RegularElement' || node.type === 'SvelteElement') {
      const attributes = node.type === 'SvelteElement'
        ? anyResourceAttribute
        : resourceAttributes.get(node.name);
      for (const attribute of node.attributes ?? []) {
        if (attribute.type === 'SpreadAttribute') {
          found.push(`unverifiable spread attributes on ${node.name}`);
        }
        if (attribute.type === 'Attribute' && attributes?.has(attribute.name)) {
          const localViteEntry =
            node.name === 'script' &&
            attribute.name === 'src' &&
            node.attributes.some(
              (candidate) =>
                candidate.type === 'Attribute' &&
                candidate.name === 'type' &&
                Array.isArray(candidate.value) &&
                candidate.value.length === 1 &&
                candidate.value[0].type === 'Text' &&
                candidate.value[0].data === 'module',
            ) &&
            Array.isArray(attribute.value) &&
            attribute.value.length === 1 &&
            attribute.value[0].type === 'Text' &&
            attribute.value[0].data === '/src/main.js';
          if (!localViteEntry) {
            found.push(`resource attribute ${node.name}.${attribute.name}`);
          }
        }
        if (attribute.type === 'Attribute' && attribute.name === 'style') {
          const attributeSource = source.slice(attribute.start, attribute.end);
          const dynamic = !Array.isArray(attribute.value) ||
            attribute.value.some((value) => value.type !== 'Text');
          if (dynamic || cssNetworkAuthority.test(attributeSource)) {
            found.push('resource-capable style attribute');
          }
        }
        if (attribute.type === 'StyleDirective' && resourceStyleDirective.test(attribute.name)) {
          found.push(`resource-capable style directive ${attribute.name}`);
        }
      }
      if (node.name === 'style' && cssNetworkAuthority.test(source.slice(node.start, node.end))) {
        found.push('resource-capable HTML style block');
      }
    }
    if (node.type === 'HtmlTag') {
      found.push('raw HTML template expression');
    }

    for (const value of Object.values(node)) {
      if (Array.isArray(value)) {
        value.forEach(visit);
      } else {
        visit(value);
      }
    }
  }

  visit(parsed.fragment);
  if (parsed.css?.content?.styles && cssNetworkAuthority.test(parsed.css.content.styles)) {
    found.push('resource-capable component CSS');
  }
  return found;
}

function sourceAnalysis(path, source) {
  if (extname(path) === '.html') {
    const parsed = parseSvelte(source, { modern: true });
    return {
      declarative: declarativeNetworkViolations(parsed, source),
      units: [],
    };
  }
  if (extname(path) !== '.svelte') {
    return { declarative: [], units: [{ path, source }] };
  }

  const parsed = parseSvelte(source, { modern: true });
  const compiled = compileSvelte(source, { filename: path, generate: 'client' });
  const scripts = [parsed.module, parsed.instance].flatMap((script, index) =>
    script?.content
      ? [{
          path: `${path}#script-${index + 1}.ts`,
          source: source.slice(script.content.start, script.content.end),
        }]
      : [],
  );
  return {
    declarative: declarativeNetworkViolations(parsed, source),
    units: [...scripts, { path: `${path}#compiled.js`, source: compiled.js.code }],
  };
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
    '.html',
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
  const svelteImport = sourceAnalysis(
    '<boundary-self-test>.svelte',
    "{#await import('../../../apps/uzel/src/main.ts')}<p>wait</p>{/await}",
  );
  if (!svelteImport.units.some((unit) => violations(unit.path, unit.source).length > 0)) {
    throw new Error('boundary self-test accepted a Svelte template import');
  }
  const svelteNetwork = sourceAnalysis(
    '<boundary-self-test>.svelte',
    '<button onclick={() => fetch(url)}>go</button>',
  );
  if (!svelteNetwork.units.some((unit) => directNetworkAuthority.test(unit.source))) {
    throw new Error('boundary self-test accepted Svelte template network authority');
  }
  const svelteTypeImport = sourceAnalysis(
    '<boundary-self-test>.svelte',
    '<script lang="ts">import type { Runtime } from "napd/runtime";</script>',
  );
  if (!svelteTypeImport.units.some((unit) => violations(unit.path, unit.source).length > 0)) {
    throw new Error('boundary self-test accepted a Svelte type import');
  }
  for (const source of [
    '<img src={remote}>',
    '<iframe src={remote}></iframe>',
    '<img {...{ src: remote }}>',
    '<svelte:element this={tag} src={remote}></svelte:element>',
    '{@html remoteMarkup}',
    '<div style:background-image={remote}></div>',
    '<style>.avatar { background-image: url(remote); }</style>',
  ]) {
    if (sourceAnalysis('<boundary-self-test>.svelte', source).declarative.length === 0) {
      throw new Error(`boundary self-test accepted declarative network authority: ${source}`);
    }
  }
  const allowedEntry = sourceAnalysis(
    '<boundary-self-test>.html',
    '<script type="module" src="/src/main.js"></script>',
  );
  if (allowedEntry.declarative.length > 0) {
    throw new Error('boundary self-test rejected the local Vite module entry');
  }
  for (const source of [
    '<img src="https://example.test/avatar.png">',
    '<link rel="stylesheet" href="https://example.test/theme.css">',
    '<iframe src="https://example.test"></iframe>',
    '<style>.avatar { background-image: url(https://example.test/avatar.png); }</style>',
  ]) {
    if (sourceAnalysis('<boundary-self-test>.html', source).declarative.length === 0) {
      throw new Error(`boundary self-test accepted HTML network authority: ${source}`);
    }
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
  const productNapplet =
    path.startsWith(join(nappletsRoot, 'follow-list')) ||
    path.startsWith(join(nappletsRoot, 'profile-card'));
  if (extname(path) === '.html' && !productNapplet) {
    continue;
  }
  const analysis = sourceAnalysis(path, source);
  for (const unit of analysis.units) {
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
  if (productNapplet) {
    for (const violation of analysis.declarative) {
      console.error(`${relative(repositoryRoot, path)}: ${violation} is forbidden`);
      failed = true;
    }
  }
}

if (failed) {
  process.exitCode = 1;
}
