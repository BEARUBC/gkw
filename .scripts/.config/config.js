module.exports = {
    shellOptions: { shell: true, stdio: 'inherit' },
    branch_name_policy: /(bug|task)\/GKW-[0-9]+/,
    hooks: [
        'pre-commit',
        'pre-push',
    ],
    colors: {
        Reset: '\x1b[0m',
        Bright: '\x1b[1m',
        Dim: '\x1b[2m',
        Underscore: '\x1b[4m',
        Blink: '\x1b[5m',
        Reverse: '\x1b[7m',
        Hidden: '\x1b[8m',

        FgBlack: '\x1b[1;30m',
        FgRed: '\x1b[1;31m',
        FgGreen: '\x1b[1;32m',
        FgYellow: '\x1b[1;33m',
        FgBlue: '\x1b[1;34m',
        FgMagenta: '\x1b[1;35m',
        FgCyan: '\x1b[1;36m',
        FgWhite: '\x1b[1;37m',
    },
};
  