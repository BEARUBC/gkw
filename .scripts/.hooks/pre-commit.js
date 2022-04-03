#!/usr/bin/env node
const { run, finish } = require('../utils');

run({
  title: 'Running formatter...',
  command: 'cargo',
  args: ['+nightly fmt -- --check'],
});

finish();
