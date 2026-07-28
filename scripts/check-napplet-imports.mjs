import { createRequire } from 'node:module';
import { readdirSync, readFileSync } from 'node:fs';
import { extname, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(fileURLToPath(new URL('..', import.meta.url)));
const requireFromShell = createRequire(
  new URL('../apps/uzel/package.json', import.meta.url),
);
const ts = requireFromShell('typescript');
const forbiddenSpecifier = /(uzel|napd|tauri)/i;
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

  return [...source.matchAll(/<script(?:\s[^>]*)?>([\s\S]*?)<\/script>/gi)].map(
    (match, index) => ({ path: `${path}#script-${index + 1}`, source: match[1] }),
  );
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
for (const path of sourceFiles(join(repositoryRoot, 'napplets'))) {
  const source = readFileSync(path, 'utf8');
  for (const unit of sourceUnits(path, source)) {
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
