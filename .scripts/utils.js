const { spawnSync } = require('child_process');
const { default_tag, shellOptions, colors: { Reset, FgRed, FgGreen, FgBlue, FgYellow } } = require('./.config/config');
const fs = require('fs');
const toml = require('toml');

const INFO = 'info';
const WARN = 'warning'
const SUCCESS = 'success';
const ERROR = 'error';

const printer = (c1, description) => msg => console.log(
  c1,
  description + ':' + `${
    (description === INFO || description === ERROR)?
    '\t\t' : '\t'}`,
  Reset,
  msg,
);

const info = printer(FgBlue, INFO);
const warn = printer(FgYellow, WARN);
const success = printer(FgGreen, SUCCESS);
const error = printer(FgRed, ERROR);

module.exports = {
  command: ({ command, args, output = true }) => spawnSync(command, args, output ? shellOptions : { ...shellOptions, stdio: undefined }),

  info,
  warn,
  success,
  error,

  run: ({
    title,
    successMsg = 'passed...',
    errorMsg = 'failed...',
    command,
    args = [],
  }) => {
    info(title);

    const { status } = spawnSync(command, args, shellOptions);

    if (status === 0) {
      success(successMsg);
      console.log();
    }
    else {
      error(errorMsg);
      process.exit(1);
    }
  },

  finish: () => {
    success('all checks passed');
    process.exit(0);
  },

  extract: () => {
    try {
      const data = fs.readFileSync('./Cargo.toml', 'utf8');
      const tomlData = toml.parse(data);

      const {
        package: {
          version,
        },
      } = tomlData;

      return version;
    }
    
    catch (err) {
      return default_tag;
    };
  },
};
