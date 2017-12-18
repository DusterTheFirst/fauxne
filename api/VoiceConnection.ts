import { Logger } from "./Logger";
import * as WebSocket from 'ws'; 
import { request } from "https";
import * as udp from 'dgram';
import OpusScript = require('opusscript');

export class VoiceConnection extends Logger { 
    
    protected ws: WebSocket;
    private readonly authtoken: string;
    private readonly endpoint: string;
    private readonly channel: string;
    private address: string;
    private port: number;
    private localAddress: string;
    private localPort: number;
    private ping: NodeJS.Timer;
    private opus: OpusScript;

    constructor(authtoken :string, token: string, endpoint: string, channel: string, session: string, user: string) {
        super('ws', `voice`);

        console.log(arguments);

        this.authtoken = authtoken;
        this.endpoint = endpoint;
        this.channel = channel;

        this.ws = new WebSocket(`wss://${endpoint.replace(':80' ,'')}/?v=3`);
        
        this.ws.onclose = (event) => this.warn(`Closed, ${event.code}:${event.reason}`);
        this.ws.onerror = (error) => this.error(error.stack);

        this.ws.onmessage = (event) => {
            let message = JSON.parse(event.data.toString());
            let data: any = message.d;
            let opcode: number = message.op;

            this.warn(message);

            if (opcode === 2) {
                this.port = data.port;
                this.address = data.ip;

                var client = udp.createSocket('udp4');

                client.once('message', (msg,info) => {
                    this.log(this.parseLocalPacket(msg));
                    const packet = this.parseLocalPacket(msg);

                    this.localAddress = packet.address;
                    this.localPort = packet.port;

                    this.ws.send(JSON.stringify({
                        op: 1,
                        d: {
                          protocol: 'udp',
                          data: {
                            address: packet.address,
                            port: packet.port,
                            mode: 'xsalsa20_poly1305',
                          }
                        }
                    }));
                });

                const blankMessage = Buffer.alloc(70);
                blankMessage.writeUIntBE(data.ssrc, 0, 4);
                client.send(blankMessage, this.port, this.address);
                //this.warn(data.modes)
            }

            if (opcode === 4) {
                this.error(data.secret_key);

                this.opus = new OpusScript(48000, 2, OpusScript.Application.AUDIO);                
            }

            if (opcode === 8) {
                this.ping = setInterval(() => {
                    if (this.ws.readyState !== this.ws.OPEN) {
                        this.warn('Connection closed, cancelled heartbeats')
                        clearInterval(this.ping);
                        return;
                    }
                    
                    this.log('Sent heartbeat')
                    this.ws.send(JSON.stringify({
                        "op": 1,
                        "d": Math.floor(Math.random() * 10e10)
                    }))
                }, data.heartbeat_interval * .75)

                this.ws.send(JSON.stringify({
                    "op": 0,
                    "d": {
                        "server_id": channel,
                        "user_id": user,
                        "session_id": session,
                        "token": token
                    }
                }));
            }
        }

        this.ws.onopen = () => {
            this.log('Connected');
            this.ring();
        }
    }

    ring() {
        request({
            method: 'POST',
            hostname: 'discordapp.com',
            protocol: 'https:',
            path: `/api/v6/channels/${this.channel}/call/ring`,
            headers: {
                "Authorization": this.authtoken,
                "Content-Type": "application/json"
            }
        }).end();
    }

    parseLocalPacket(message: Buffer) {
        try {
          const packet = Buffer.from(message);
          let address = '';
          for (let i = 4; i < packet.indexOf(0, i); i++) address += String.fromCharCode(packet[i]);
          const port = parseInt(packet.readUIntLE(packet.length - 2, 2).toString(10), 10);
          return { address, port };
        } catch (error) {
          return { address: null, port: null };
        }
      }
    
}