"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
let config = require('./config.json');
const Eris = require("eris");
let client = new Eris.Client(config.token);
client.connect();
client.on('ready', () => {
    client.getDMChannel("168827261682843648").then((channel) => {
        client.joinVoiceChannel(/**channel.id**/ "274937002195943425").then((connection) => {
            channel.ring([channel.recipient.id]);
            connection.play('./nate_is_bad.ogg', { format: "ogg" });
            connection.on('end', () => {
                client.leaveVoiceChannel(channel.id);
            });
        }).catch((reason) => {
            client.leaveVoiceChannel(channel.id);
            console.error(reason);
        });
    });
});
