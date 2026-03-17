#!/usr/bin/env node

const { Command } = require('commander');
const chalk = require('chalk');
const ora = require('ora');
const fs = require('fs');
const path = require('path');
const CryptoJS = require('crypto-js');
const inquirer = require('inquirer');

const program = new Command();
const NEXUS_DIR = '.env-nexus';
const CONFIG_FILE = 'config.json';
const ENCRYPTED_EXT = '.enc';

program
  .name('dotenv-nexus')
  .description('The modern .env file manager')
  .version('1.0.0');

// Initialize dotenv-nexus in the current project
program
  .command('init')
  .description('Initialize dotenv-nexus in the current project')
  .action(async () => {
    const spinner = ora('Initializing .env Nexus...').start();

    try {
      const cwd = process.cwd();
      const nexusPath = path.join(cwd, NEXUS_DIR);
      
      if (fs.existsSync(nexusPath)) {
        spinner.warn(chalk.yellow('.env Nexus is already initialized'));
        return;
      }

      fs.mkdirSync(nexusPath, { recursive: true });
      
      const config = {
        defaultEnv: 'development',
        environments: [],
        version: '1.0.0'
      };
      
      fs.writeFileSync(
        path.join(nexusPath, CONFIG_FILE),
        JSON.stringify(config, null, 2)
      );

      // Add to .gitignore if it exists
      const gitignorePath = path.join(cwd, '.gitignore');
      const gitignoreContent = fs.existsSync(gitignorePath) 
        ? fs.readFileSync(gitignorePath, 'utf8') 
        : '';
      
      if (!gitignoreContent.includes('.env-nexus/keys/')) {
        const newContent = gitignoreContent + '\n# dotenv-nexus\n.env-nexus/keys/\n';
        fs.writeFileSync(gitignorePath, newContent);
      }

      spinner.succeed(chalk.green('.env Nexus initialized successfully!'));
      console.log();
      console.log(chalk.cyan('Next steps:'));
      console.log(`  ${chalk.bold('dotenv-nexus add <your-env-file> --env <environment>')}`);
      console.log('  Then encrypt with: ' + chalk.bold('dotenv-nexus encrypt'));
    } catch (error) {
      spinner.fail(chalk.red(`Initialization failed: ${error.message}`));
      process.exit(1);
    }
  });

// Add an existing .env file
program
  .command('add <file>')
  .description('Add an existing .env file to nexus')
  .option('-e, --env <environment>', 'environment name', 'development')
  .action(async (file, options) => {
    const spinner = ora(`Adding ${file} to .env Nexus...`).start();
    
    try {
      const cwd = process.cwd();
      const nexusPath = path.join(cwd, NEXUS_DIR);
      
      if (!fs.existsSync(nexusPath)) {
        spinner.fail(chalk.red('Not initialized. Run `dotenv-nexus init` first.'));
        process.exit(1);
      }
      
      if (!fs.existsSync(file)) {
        spinner.fail(chalk.red(`File ${file} not found`));
        process.exit(1);
      }

      const content = fs.readFileSync(file, 'utf8');
      const envName = options.env;
      
      // Save plaintext temporarily until encryption
      const envPath = path.join(nexusPath, `${envName}.env`);
      fs.writeFileSync(envPath, content);
      
      // Update config
      const config = JSON.parse(fs.readFileSync(path.join(nexusPath, CONFIG_FILE), 'utf8'));
      if (!config.environments.includes(envName)) {
        config.environments.push(envName);
        fs.writeFileSync(path.join(nexusPath, CONFIG_FILE), JSON.stringify(config, null, 2));
      }
      
      // Generate .env.example automatically
      const lines = content.split('\n');
      const exampleLines = lines.map(line => {
        const trimmed = line.trim();
        if (!trimmed || trimmed.startsWith('#')) return line;
        const eqIndex = trimmed.indexOf('=');
        if (eqIndex === -1) return line;
        const key = trimmed.slice(0, eqIndex);
        return `${key}=`;
      });
      
      const examplePath = path.join(cwd, '.env.example');
      const existingExample = fs.existsSync(examplePath) 
        ? fs.readFileSync(examplePath, 'utf8').split('\n') 
        : [];
      
      // Merge existing example with new entries
      const keySet = new Set(existingExample.map(l => {
        const trimmed = l.trim();
        const eqIndex = trimmed.indexOf('=');
        return eqIndex !== -1 ? trimmed.slice(0, eqIndex).trim() : null;
      }).filter(Boolean));
      
      const mergedExample = [...existingExample];
      exampleLines.forEach(line => {
        const trimmed = line.trim();
        if (!trimmed || trimmed.startsWith('#')) return;
        const eqIndex = trimmed.indexOf('=');
        if (eqIndex === -1) return;
        const key = trimmed.slice(0, eqIndex).trim();
        if (!keySet.has(key)) {
          mergedExample.push(line);
        }
      });
      
      fs.writeFileSync(examplePath, mergedExample.join('\n'));
      
      spinner.succeed(chalk.green(`Added ${file} as environment '${envName}'`));
      console.log(`Generated/updated ${chalk.cyan('.env.example')}`);
    } catch (error) {
      spinner.fail(chalk.red(`Add failed: ${error.message}`));
      process.exit(1);
    }
  });

// Encrypt all environments
program
  .command('encrypt')
  .description('Encrypt all environments with a password')
  .option('-p, --password <password>', 'encryption password')
  .action(async (options) => {
    let password = options.password;
    
    if (!password) {
      const answers = await inquirer.prompt([{
        type: 'password',
        name: 'password',
        message: 'Enter encryption password:',
        validate: (input) => input.length >= 8 || 'Password must be at least 8 characters'
      }, {
        type: 'password',
        name: 'confirm',
        message: 'Confirm encryption password:',
        validate: (input) => input === password || 'Passwords do not match'
      }]);
      password = answers.password;
    }
    
    const spinner = ora('Encrypting environments...').start();
    
    try {
      const cwd = process.cwd();
      const nexusPath = path.join(cwd, NEXUS_DIR);
      const config = JSON.parse(fs.readFileSync(path.join(nexusPath, CONFIG_FILE), 'utf8'));
      
      const encryptedDir = path.join(nexusPath, 'environments');
      fs.mkdirSync(encryptedDir, { recursive: true });
      
      for (const env of config.environments) {
        const envPath = path.join(nexusPath, `${env}.env`);
        if (!fs.existsSync(envPath)) continue;
        
        const content = fs.readFileSync(envPath, 'utf8');
        const encrypted = CryptoJS.AES.encrypt(content, password).toString();
        const encryptedPath = path.join(encryptedDir, `${env}${ENCRYPTED_EXT}`);
        fs.writeFileSync(encryptedPath, encrypted);
        fs.unlinkSync(envPath); // Remove plaintext
      }
      
      spinner.succeed(chalk.green('All environments encrypted successfully!'));
      console.log();
      console.log(chalk.yellow('⚠️  Make sure you remember your password!'));
      console.log(chalk.yellow('   Without it, your encrypted environment files cannot be recovered.'));
    } catch (error) {
      spinner.fail(chalk.red(`Encryption failed: ${error.message}`));
      process.exit(1);
    }
  });

// Decrypt all environments
program
  .command('decrypt')
  .description('Decrypt environments with password')
  .option('-p, --password <password>', 'decryption password')
  .action(async (options) => {
    let password = options.password;
    
    if (!password) {
      const answers = await inquirer.prompt([{
        type: 'password',
        name: 'password',
        message: 'Enter decryption password:'
      }]);
      password = answers.password;
    }
    
    const spinner = ora('Decrypting environments...').start();
    
    try {
      const cwd = process.cwd();
      const nexusPath = path.join(cwd, NEXUS_DIR);
      const encryptedDir = path.join(nexusPath, 'environments');
      const config = JSON.parse(fs.readFileSync(path.join(nexusPath, CONFIG_FILE), 'utf8'));
      
      if (!fs.existsSync(encryptedDir)) {
        spinner.fail(chalk.red('No encrypted environments found'));
        process.exit(1);
      }
      
      let successCount = 0;
      for (const env of config.environments) {
        const encryptedPath = path.join(encryptedDir, `${env}${ENCRYPTED_EXT}`);
        if (!fs.existsSync(encryptedPath)) continue;
        
        const encrypted = fs.readFileSync(encryptedPath, 'utf8');
        try {
          const decrypted = CryptoJS.AES.decrypt(encrypted, password).toString(CryptoJS.enc.Utf8);
          const outputPath = path.join(cwd, `.env.${env}`);
          fs.writeFileSync(outputPath, decrypted);
          successCount++;
        } catch (e) {
          spinner.fail(chalk.red(`Decryption failed for ${env}: Wrong password or corrupted data`));
          process.exit(1);
        }
      }
      
      spinner.succeed(chalk.green(`Decrypted ${successCount} environment(s) successfully!`));
    } catch (error) {
      spinner.fail(chalk.red(`Decryption failed: ${error.message}`));
      process.exit(1);
    }
  });

// Switch to a different environment
program
  .command('switch <env>')
  .description('Switch to a different environment (copy to .env)')
  .action(async (env) => {
    const spinner = ora(`Switching to ${env}...`).start();
    
    try {
      const cwd = process.cwd();
      const sourcePath = path.join(cwd, `.env.${env}`);
      
      if (!fs.existsSync(sourcePath)) {
        spinner.fail(chalk.red(`.env.${env} not found. Decrypt first with 'dotenv-nexus decrypt'`));
        process.exit(1);
      }
      
      const content = fs.readFileSync(sourcePath, 'utf8');
      fs.writeFileSync(path.join(cwd, '.env'), content);
      
      // Update default in config
      const nexusPath = path.join(cwd, NEXUS_DIR);
      if (fs.existsSync(path.join(nexusPath, CONFIG_FILE))) {
        const config = JSON.parse(fs.readFileSync(path.join(nexusPath, CONFIG_FILE), 'utf8'));
        config.defaultEnv = env;
        fs.writeFileSync(path.join(nexusPath, CONFIG_FILE), JSON.stringify(config, null, 2));
      }
      
      spinner.succeed(chalk.green(`Switched to environment '${env}' -> .env`));
    } catch (error) {
      spinner.fail(chalk.red(`Switch failed: ${error.message}`));
      process.exit(1);
    }
  });

// List environments
program
  .command('list')
  .alias('ls')
  .description('List all configured environments')
  .action(() => {
    try {
      const cwd = process.cwd();
      const nexusPath = path.join(cwd, NEXUS_DIR);
      
      if (!fs.existsSync(nexusPath)) {
        console.log(chalk.yellow('Not initialized. Run `dotenv-nexus init` first.'));
        return;
      }
      
      const config = JSON.parse(fs.readFileSync(path.join(nexusPath, CONFIG_FILE), 'utf8'));
      const defaultEnv = config.defaultEnv;
      
      console.log(chalk.cyan('Configured environments:'));
      console.log();
      
      config.environments.forEach(env => {
        const isDefault = env === defaultEnv;
        console.log(`  ${isDefault ? chalk.green('✓') : ' '} ${env} ${isDefault ? chalk.dim('(default)') : ''}`);
      });
      
      console.log();
    } catch (error) {
      console.error(chalk.red(`List failed: ${error.message}`));
      process.exit(1);
    }
  });

// Diff - show difference between two environments
program
  .command('diff <env1> <env2>')
  .description('Show difference between two environments')
  .action((env1, env2) => {
    try {
      const cwd = process.cwd();
      const path1 = path.join(cwd, `.env.${env1}`);
      const path2 = path.join(cwd, `.env.${env2}`);
      
      if (!fs.existsSync(path1)) {
        console.log(chalk.red(`${path1} not found. Decrypt first.`));
        process.exit(1);
      }
      
      if (!fs.existsSync(path2)) {
        console.log(chalk.red(`${path2} not found. Decrypt first.`));
        process.exit(1);
      }
      
      const parseEnv = (content) => {
        const obj = {};
        content.split('\n').forEach(line => {
          const trimmed = line.trim();
          if (!trimmed || trimmed.startsWith('#')) return;
          const eqIndex = trimmed.indexOf('=');
          if (eqIndex === -1) return;
          const key = trimmed.slice(0, eqIndex).trim();
          const value = trimmed.slice(eqIndex + 1).trim();
          obj[key] = value;
        });
        return obj;
      };
      
      const env1Obj = parseEnv(fs.readFileSync(path1, 'utf8'));
      const env2Obj = parseEnv(fs.readFileSync(path2, 'utf8'));
      
      const allKeys = new Set([...Object.keys(env1Obj), ...Object.keys(env2Obj)]);
      const sortedKeys = Array.from(allKeys).sort();
      
      console.log(chalk.cyan(`Diff: ${env1} vs ${env2}`));
      console.log();
      
      sortedKeys.forEach(key => {
        const val1 = env1Obj[key];
        const val2 = env2Obj[key];
        
        if (val1 === val2) return;
        
        if (!(key in env1Obj)) {
          console.log(chalk.green(`+ ${key}=${val2}`));
        } else if (!(key in env2Obj)) {
          console.log(chalk.red(`- ${key}=${val1}`));
        } else {
          console.log(chalk.yellow(`~ ${key}:`));
          console.log(chalk.red(`  ${env1}: ${val1}`));
          console.log(chalk.green(`  ${env2}: ${val2}`));
        }
      });
      
    } catch (error) {
      console.error(chalk.red(`Diff failed: ${error.message}`));
      process.exit(1);
    }
  });

program.parse(process.argv);
