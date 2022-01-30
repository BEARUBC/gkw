#!/usr/bin/env node
const { command, run, finish } = require('../utils');

const { stdout } = command({
  command: 'git',
  args: ['rev-parse --abbrev-ref HEAD'],
  output: false,
});

const branchName = stdout.toString().replace('\n', '');
if (branch_name_policy.exec(branchName) === null) {
  error('Invalid branch name; please follow the branch-naming policy: /(bug|task)/WAVY-[0-9]+/');
  process.exit(1);
}

run({
  title: 'Building...',
  command: 'cargo',
  args: ['build'],
});

run({
  title: 'Running formatter...',
  command: 'cargo',
  args: ['+nightly fmt -- --check'],
});

run({
  title: 'Running clips...',
  command: 'cargo',
  args: ['+nightly clippy'],
});

run({
  title: 'Hello!',
  command: 'ls',
  args: ['-las'],
});

finish();
