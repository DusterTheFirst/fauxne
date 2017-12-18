import { VoiceClient } from "./VoiceClient";
import { Logger } from "./Logger";

let config = require('./config.json');

// Logger.showColors = false;

let client = new VoiceClient(config.token, {
    verbose: true
});
client.on("READY", () => {
    client.call(/*'140762569056059392'*//*'388878709735555082'*/"168827261682843648");  
    setTimeout(() => client.hangup(), 100000);  
}).on("RING", () => {
    console.log('BEING CALLED')
}).mute();