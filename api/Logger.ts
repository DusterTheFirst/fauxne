import * as Chalk from 'chalk';
import { inspect } from 'util';
import { EventEmitter } from 'events';

export class Logger extends EventEmitter{
    protected readonly category: string;
    protected readonly thread: string;
    static showColors: boolean = true;
    private muted: boolean = false;
    
    protected constructor(category: string, thread: string) {
        super();
        this.category = category;
        this.thread = thread;
    }

    protected log(message: string | object, depth: number = 1) {
        if (this.muted)
            return;
        
        if (message instanceof Object)
            message = inspect(message, { colors: Logger.showColors, customInspect: true, depth: depth});
            
        let output = `[${this.category}][${this.thread}] ${message}`;

        if (Logger.showColors)
            console.log(Chalk.default.gray(output));
        else
            console.log(output);
    }
    protected error(message: string | object) {
        if (this.muted)
            return;
        
        if (message instanceof Object)
            message = inspect(message, { colors: Logger.showColors, customInspect: true, depth: 1});        
        
        let output = `[${this.category}][${this.thread}][ERROR] ${message}`;

        if (Logger.showColors)
            console.error(Chalk.default.red(output))
        else
            console.error(output)
    }
    protected warn(message: string | object) {
        if (this.muted)
            return;
        
        if (message instanceof Object)
            message = inspect(message, { colors: Logger.showColors, customInspect: true, depth: 1});
        
        let output = `[${this.category}][${this.thread}][WARN] ${message}`

        if (Logger.showColors)
            console.warn(Chalk.default.yellow(output))
        else
            console.warn(output)
    }

    public mute() {
        this.muted = true;
    }

    public unmute() {
        this.muted = false;
    }
}