import * as WebSocket from 'ws'; 
import { Logger } from './Logger';
import { VoiceConnection } from './VoiceConnection';
import { request } from 'https';

interface VoiceClientOptions {
    verbose?: boolean,
    hideToken?: boolean;
}

export interface VoiceClient {
    on(event: 'READY', listener: () => void): this;
    on(event: 'RING', listener: () => void): this;
}
export class VoiceClient extends Logger {
    protected ws: WebSocket;
    private readonly options: VoiceClientOptions;
    private readonly token: string;
    private lasts: number;
    private ping: NodeJS.Timer;
    public voiceConnection: VoiceConnection;
    public id: string;
    private session: string;
    private user: string;

    constructor(token: string, options?: VoiceClientOptions) {
        super('ws', 'main');

        this.options = options;
        this.ws = new WebSocket('wss://gateway.discord.gg/?v=6');
        this.token = token;
        
        this.ws.onclose = (event) => this.error(`Connection closed | ${event.code}: '${event.reason}'`);
        this.ws.onerror = (error) => this.error(error.stack);
        this.ws.onopen = (event) => {
            this.log(`Connected to gateway \'${event.target.url}\'`);

            // LOGIN
            this.ws.send(JSON.stringify({
                "op": 2,
                "d": {
                    "token": this.token,
                    "properties": {
                        "$os": "macos",
                        "$browser": "DSelf.js",
                        "$device": "DSelf.js"
                    },
                    "compress": false,
                    "large_threshold": 50
                }
            }));
            if (!this.options.hideToken)
                this.log(`Logging in using token '${this.token}'`);
            else
                this.log(`Logging in`)
        }
        this.ws.onmessage = (event) => {
            let message = JSON.parse(event.data.toString());
            let data: any = message.d;
            let opcode: number = message.op;
            let t: string = message.t;
            let s: number = message.s;
            this.lasts = s || this.lasts;

            this.warn(message);

            if (t === 'READY') {
                this.log('READY');
                this.id = data.user.id;
                this.emit('READY');
            }

            if (t === 'CALL_UPDATE' && data.ringing.includes(this.id)) {
                this.log('Being Called');
                console.log(data);

                this.emit('RING');
            }

            if (opcode === 11 && this.options.verbose)
                this.log('Heartbeat ACK')

            if (data && data.heartbeat_interval) {
                this.ping = setInterval(() => {
                    if (this.ws.readyState !== this.ws.OPEN) {
                        this.warn('Connection closed, cancelled heartbeats')
                        clearInterval(this.ping);
                        return;
                    }
                    
                    if (this.options.verbose)
                        this.log('Sent heartbeat')
                    this.ws.send(JSON.stringify({
                        "op": 1,
                        "d": this.lasts
                    }))
                }, data.heartbeat_interval as number)
            }

            if (t === "VOICE_STATE_UPDATE") {
                this.session = data.session_id;
                this.user = data.user_id;
            }

            if (t === "VOICE_SERVER_UPDATE") {
                //this.log(data);
                this.voiceConnection = new VoiceConnection(this.token, data.token, data.endpoint, data.channel_id, this.session, this.user);        
            }
        }

    }

    public call(user: string) {
        let req = request({
            method: 'POST',
            hostname: 'discordapp.com',
            protocol: 'https:',
            path: `/api/v6/users/@me/channels`,
            headers: {
                "Authorization": this.token,
                "Content-Type": "application/json"
            },
        }, res => {
            let data = "";
            res.on('data', (chunk) => {
                data += chunk;
            });
            res.on('end', () => {
                //this.log(data);
                let body = JSON.parse(data);

                this.ws.send(JSON.stringify({
                    "op": 4,
                    "d": {
                        "channel_id": body.id,
                        "guild_id":null,
                        "self_mute": false,
                        "self_deaf": false
                    }
                }));
                this.log(`Calling ${user} as ${body.id}`);
            })
        });
        req.write(JSON.stringify({
            recipient_id: user
        }));
        req.end();
    }

    hangup() {
        this.ws.send(JSON.stringify({
            "op": 4,
            "d": {
                "channel_id": null,
                "guild_id":null,
                "self_mute": false,
                "self_deaf": false
            }
        }));
    }
    
}