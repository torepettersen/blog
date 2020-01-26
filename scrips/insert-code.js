const fs = require('fs');

if (process.argv.length < 3) {
  console.error('Missing path');
  process.exit(1);
}

const filePath = process.argv[2];
let file = fs.readFileSync(filePath, 'utf8');
const paths = file.match(/(?<=<<< ).*/g);
const languages = {
  rs: 'rust',
};

for (const path of paths) {
  const relativePath = path.replace(/^@/, './');
  const [ fileExtension ] = path.match(/(?<=\.).*$/);
  const lang = languages[fileExtension] || fileExtension;
  const codeFile = fs.readFileSync(relativePath, 'utf8');
  const code = '``` ' + lang + '\n' + codeFile + '\n```'
  file = file.replace(new RegExp(`<<< ${path}`), code);
}

fs.writeFileSync('./test.md', file);
