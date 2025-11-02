import { join } from 'path';
import { readFileSync, writeFileSync } from 'fs';

// 此脚本应在根目录下执行
// 命令行参数
const args = process.argv.slice(2);
// patch, minor, major
const versionType = args[0] || 'patch';

// 读取 tauri.conf.json
const tauriConfPath = join(process.cwd(), 'src-tauri', 'tauri.conf.json');
const tauriConfig = JSON.parse( readFileSync(tauriConfPath, 'utf8'));

// 解析当前版本
const currentVersion = tauriConfig.version;
const [major, minor, patch] = currentVersion.split('.').map(Number);

// 计算新版本
let newVersion;

switch (versionType) {
  case 'major':
    newVersion = `${major + 1}.0.0`;
    break;
  case 'minor':
    newVersion = `${major}.${minor + 1}.0`;
    break;
  case 'patch':
  default:
    newVersion = `${major}.${minor}.${patch + 1}`;
    break;
}

// 更新 tauri.conf.json
tauriConfig.version = newVersion;
writeFileSync(tauriConfPath, JSON.stringify(tauriConfig, null, 2) + '\n');

// 更新 Cargo.toml
const cargoTomlPath = join(process.cwd(), 'src-tauri', 'Cargo.toml');
let cargoToml = readFileSync(cargoTomlPath, 'utf8');
cargoToml = cargoToml.replace(
  /(\[package\][\s\S]*?version\s*=\s*)"[^"]*"/,
  `$1"${newVersion}"`
);
writeFileSync(cargoTomlPath, cargoToml);

// 更新 package.json
const packageJsonPath = join(process.cwd(), 'package.json');
let packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf8'));
packageJson.version = newVersion;
writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2) + '\n');

console.log(`✅ 版本已从 ${currentVersion} 更新到 ${newVersion}`);
console.log(`📝 已更新以下文件：`);
console.log(`   - src-tauri/tauri.conf.json`);
console.log(`   - src-tauri/Cargo.toml`);
console.log(`   - package.json`);
