"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const Chalk = require("chalk");
const util_1 = require("util");
const events_1 = require("events");
class Logger extends events_1.EventEmitter {
    constructor(category, thread) {
        super();
        this.muted = false;
        this.category = category;
        this.thread = thread;
    }
    log(message, depth = 1) {
        if (this.muted)
            return;
        if (message instanceof Object)
            message = util_1.inspect(message, { colors: Logger.showColors, customInspect: true, depth: depth });
        let output = `[${this.category}][${this.thread}] ${message}`;
        if (Logger.showColors)
            console.log(Chalk.default.gray(output));
        else
            console.log(output);
    }
    error(message) {
        if (this.muted)
            return;
        if (message instanceof Object)
            message = util_1.inspect(message, { colors: Logger.showColors, customInspect: true, depth: 1 });
        let output = `[${this.category}][${this.thread}][ERROR] ${message}`;
        if (Logger.showColors)
            console.error(Chalk.default.red(output));
        else
            console.error(output);
    }
    warn(message) {
        if (this.muted)
            return;
        if (message instanceof Object)
            message = util_1.inspect(message, { colors: Logger.showColors, customInspect: true, depth: 1 });
        let output = `[${this.category}][${this.thread}][WARN] ${message}`;
        if (Logger.showColors)
            console.warn(Chalk.default.yellow(output));
        else
            console.warn(output);
    }
    mute() {
        this.muted = true;
    }
    unmute() {
        this.muted = false;
    }
}
Logger.showColors = true;
exports.Logger = Logger;
